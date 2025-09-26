mod agent;
mod config;
mod dbus;

use std::sync::Arc;

use smol::Executor;

use crate::agent::Agent;
use crate::config::Config;
use crate::dbus::AgentManagerProxy;

const OBJ_PATH: &str = "/moontoaster/bluegent";

fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(
        env_logger::Env::new()
            .filter_or("BLUEGENT_LOG", "info")
            .write_style("BLUEGENT_LOG_STYLE"),
    );

    let exec = Executor::new();

    smol::block_on(exec.run(async {
        let cfg = Arc::new(Config::load());

        let conn = zbus::connection::Builder::system()?
            .internal_executor(false)
            .serve_at(OBJ_PATH, Agent::new(cfg.clone()))?
            .build()
            .await
            .expect("failed to connect to the system bus");

        let zbus_exec_task = {
            let conn = conn.clone();

            exec.spawn(async move {
                loop {
                    conn.executor().tick().await;
                }
            })
        };

        let agent_manager = AgentManagerProxy::new(&conn)
            .await
            .expect("failed to get the agent manager");

        log::debug!("Registering agent at {}", OBJ_PATH);

        agent_manager
            .register_agent(OBJ_PATH.try_into()?, "NoInputNoOutput")
            .await
            .expect("failed to register agent");

        log::debug!("Requesting default agent for {}", OBJ_PATH);

        agent_manager
            .request_default_agent(OBJ_PATH.try_into()?)
            .await
            .expect("failed to become default agent");

        log::info!("Ready!");

        zbus_exec_task.await
    }))
}
