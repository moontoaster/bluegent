use std::sync::Arc;

use zbus::{Connection, interface, zvariant::ObjectPath};

use crate::dbus::{Error, Result};
use crate::{config::Config, dbus::DeviceProxy};

pub struct Agent {
    config: Arc<Config>,
}

impl Agent {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

#[interface(name = "org.bluez.Agent1", introspection_docs = false)]
impl Agent {
    async fn release(&self) {
        todo!()
    }

    async fn request_pin_code(
        &mut self,
        device: ObjectPath<'_>,
        #[zbus(connection)] conn: &Connection,
    ) -> Result<&str> {
        let device = DeviceProxy::new(conn, device).await?;

        log::info!(
            "Received PIN code request for {} (\"{}\")",
            device.address().await?,
            device.name().await?
        );

        Ok(self.config.pin_code.as_str())
    }

    async fn display_pin_code(
        &self,
        device: ObjectPath<'_>,
        _pincode: &str,
        #[zbus(connection)] conn: &Connection,
    ) -> Result<()> {
        let device = DeviceProxy::new(conn, device).await?;

        // TODO: explain what's up
        log::error!(
            "Received PIN code display request for {} (\"{}\")... this is unsupported \
            and there's a configuration issue. Cancelling pairing",
            device.address().await?,
            device.name().await?
        );

        Err(Error::Canceled)
    }

    async fn request_passkey(
        &self,
        device: ObjectPath<'_>,
        #[zbus(connection)] conn: &Connection,
    ) -> Result<u32> {
        let device = DeviceProxy::new(conn, device).await?;

        log::error!(
            "Received passkey request for {} (\"{}\"), this is unsupported. Cancelling pairing",
            device.address().await?,
            device.name().await?,
        );

        Err(Error::Canceled)
    }

    async fn display_passkey(
        &self,
        device: ObjectPath<'_>,
        passkey: u32,
        entered: u16,
        #[zbus(connection)] conn: &Connection,
    ) {
        let device = DeviceProxy::new(conn, device).await.unwrap();

        log::error!(
            "Received passkey display request for {} (\"{}\"), this is unsupported",
            device.address().await.unwrap(),
            device.name().await.unwrap(),
        );
    }

    async fn request_confirmation(
        &self,
        device: ObjectPath<'_>,
        passkey: u32,
        #[zbus(connection)] conn: &Connection,
    ) -> Result<()> {
        let device = DeviceProxy::new(conn, device).await?;

        log::error!(
            "Received passkey confirm request for {} (\"{}\"), this is unsupported. Cancelling pairing",
            device.address().await?,
            device.name().await?,
        );

        Err(Error::Canceled)
    }

    async fn request_authorization(
        &self,
        device: ObjectPath<'_>,
        #[zbus(connection)] conn: &Connection,
    ) -> Result<()> {
        let device = DeviceProxy::new(conn, device).await?;

        log::info!(
            "Received authorization request for {} (\"{}\")",
            device.address().await?,
            device.name().await?,
        );

        Ok(())
    }

    async fn authorize_service(
        &self,
        device: ObjectPath<'_>,
        uuid: &str,
        #[zbus(connection)] conn: &Connection,
    ) -> Result<()> {
        let device = DeviceProxy::new(conn, device).await?;

        let is_authorized = self
            .config
            .authorized_services
            .iter()
            .any(|authed_uuid| authed_uuid.as_str() == uuid);

        if is_authorized {
            log::info!(
                "Device {} (\"{}\") wants to use service {}, permitted",
                device.address().await?,
                device.name().await?,
                uuid
            );
            Ok(())
        } else {
            log::info!(
                "Device {} (\"{}\") wants to use service {}, denied",
                device.address().await?,
                device.name().await?,
                uuid
            );
            Err(Error::Rejected)
        }
    }

    async fn cancel(&self) {
        log::debug!("Cancel method is currently stubbed out");
    }
}
