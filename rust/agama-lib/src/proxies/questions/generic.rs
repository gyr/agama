//! # D-Bus interface proxy for: `org.opensuse.Agama1.Questions.Generic`
//!
//! This code was generated by `zbus-xmlgen` `5.0.0` from D-Bus introspection data.
//! Source: `org.opensuse.Agama1.Questions.WithPassword.bus.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the [Writing a client proxy] section of the zbus
//! documentation.
//!
//! This type implements the [D-Bus standard interfaces], (`org.freedesktop.DBus.*`) for which the
//! following zbus API can be used:
//!
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PeerProxy`]
//! * [`zbus::fdo::PropertiesProxy`]
//!
//! Consequently `zbus-xmlgen` did not generate code for the above interfaces.
//!
//! [Writing a client proxy]: https://dbus2.github.io/zbus/client.html
//! [D-Bus standard interfaces]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces,
use zbus::proxy;
#[proxy(
    default_service = "org.opensuse.Agama1",
    interface = "org.opensuse.Agama1.Questions.Generic",
    assume_defaults = true
)]
pub trait Generic {
    /// Answer property
    #[zbus(property)]
    fn answer(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn set_answer(&self, value: &str) -> zbus::Result<()>;

    /// Class property
    #[zbus(property)]
    fn class(&self) -> zbus::Result<String>;

    /// Data property
    #[zbus(property)]
    fn data(&self) -> zbus::Result<std::collections::HashMap<String, String>>;

    /// DefaultOption property
    #[zbus(property)]
    fn default_option(&self) -> zbus::Result<String>;

    /// Id property
    #[zbus(property)]
    fn id(&self) -> zbus::Result<u32>;

    /// Options property
    #[zbus(property)]
    fn options(&self) -> zbus::Result<Vec<String>>;

    /// Text property
    #[zbus(property)]
    fn text(&self) -> zbus::Result<String>;
}