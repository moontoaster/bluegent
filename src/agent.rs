use std::sync::Arc;

use zbus::{
    DBusError, interface, proxy,
    zvariant::{ObjectPath, OwnedObjectPath},
};

use crate::config::Config;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(DBusError, Debug)]
#[zbus(prefix = "org.bluez.Error")]
pub enum Error {
    #[zbus(error)]
    ZBus(zbus::Error),

    Rejected,
    Canceled,
    InvalidArguments,
    AlreadyExists,
    DoesNotExist,
}

pub struct Agent {
    config: Arc<Config>,
    pairing_with: Option<OwnedObjectPath>,
}

impl Agent {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            pairing_with: None,
        }
    }
}

fn ssp_error() {
    log::error!(
        "Received request related to Bluetooth Simple Secure Pairing. This is currently unsupported. Cancelling pairing"
    );
}

#[interface(name = "org.bluez.Agent1", introspection_docs = false)]
impl Agent {
    async fn release(&self) {
        todo!()
    }

    async fn request_pin_code(&mut self, device: ObjectPath<'_>) -> Result<&str> {
        log::info!("Received PIN code request for {}", device);

        self.pairing_with = Some(OwnedObjectPath::from(device));

        Ok(self.config.pin_code.as_str())
    }

    async fn display_pin_code(&self, device: ObjectPath<'_>, _pincode: &str) -> Result<()> {
        // TODO: explain what's up
        log::error!(
            "Received PIN code display request for {}... this is unsupported \
            and there's a configuration issue. Cancelling pairing",
            device
        );

        Err(Error::Canceled)
    }

    async fn request_passkey(&self, device: ObjectPath<'_>) -> Result<u32> {
        ssp_error();
        Err(Error::Canceled)
    }

    async fn display_passkey(&self, device: ObjectPath<'_>, passkey: u32, entered: u16) {
        ssp_error();
    }

    async fn request_confirmation(&self, device: ObjectPath<'_>, passkey: u32) -> Result<()> {
        ssp_error();
        Err(Error::Canceled)
    }

    async fn request_authorization(&self, device: ObjectPath<'_>) -> Result<()> {
        ssp_error();
        Err(Error::Canceled)
    }

    async fn authorize_service(&self, device: ObjectPath<'_>, uuid: &str) -> Result<()> {
        let is_authorized = self
            .config
            .authorized_services
            .iter()
            .any(|authed_uuid| authed_uuid.as_str() == uuid);

        if is_authorized {
            log::info!("Device {} wants to use service {}, permitted", device, uuid);
            Ok(())
        } else {
            log::info!("Device {} wants to use service {}, denied", device, uuid);
            Err(Error::Rejected)
        }
    }

    async fn cancel(&self) {
        if self.pairing_with == None {
            log::debug!("Spurious cancel call??? Ignoring");
            return;
        }

        log::info!(
            "Device {} canceled pairing",
            self.pairing_with.as_ref().unwrap()
        );
    }
}

#[proxy(
    interface = "org.bluez.AgentManager1",
    default_service = "org.bluez",
    default_path = "/org/bluez"
)]
pub trait AgentManager {
    fn register_agent(&self, agent: ObjectPath<'_>, capability: &str) -> Result<()>;
    fn unregister_agent(&self, agent: ObjectPath<'_>) -> Result<()>;
    fn request_default_agent(&self, agent: ObjectPath<'_>) -> Result<()>;
}
