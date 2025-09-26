use zbus::{DBusError, proxy, zvariant::ObjectPath};

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

#[proxy(interface = "org.bluez.Device1", default_service = "org.bluez")]
pub trait Device {
    #[zbus(property)]
    fn address(&self) -> Result<String>;

    #[zbus(property)]
    fn address_type(&self) -> Result<String>;

    #[zbus(property)]
    fn name(&self) -> Result<String>;

    #[zbus(property)]
    fn adapter(&self) -> Result<ObjectPath>;

    #[zbus(property)]
    fn legacy_pairing(&self) -> Result<bool>;
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
