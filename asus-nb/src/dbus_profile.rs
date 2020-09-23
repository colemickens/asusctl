// This code was autogenerated with `dbus-codegen-rust -s -d org.asuslinux.Daemon -f org.asuslinux.Daemon -c blocking -p /org/asuslinux/Profile -m None`, see https://github.com/diwic/dbus-rs
use dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgAsuslinuxDaemon {
    fn set_profile(&self, profile: &str) -> Result<(), dbus::Error>;
    fn profile(&self) -> Result<String, dbus::Error>;
    fn profiles(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgAsuslinuxDaemon
    for blocking::Proxy<'a, C>
{
    fn set_profile(&self, profile: &str) -> Result<(), dbus::Error> {
        self.method_call("org.asuslinux.Daemon", "SetProfile", (profile,))
    }

    fn profile(&self) -> Result<String, dbus::Error> {
        self.method_call("org.asuslinux.Daemon", "Profile", ())
            .and_then(|r: (String,)| Ok(r.0))
    }

    fn profiles(&self) -> Result<String, dbus::Error> {
        self.method_call("org.asuslinux.Daemon", "Profiles", ())
            .and_then(|r: (String,)| Ok(r.0))
    }
}

#[derive(Debug)]
pub struct OrgAsuslinuxDaemonNotifyProfile {
    pub profile: String,
}

impl arg::AppendAll for OrgAsuslinuxDaemonNotifyProfile {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.profile, i);
    }
}

impl arg::ReadAll for OrgAsuslinuxDaemonNotifyProfile {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgAsuslinuxDaemonNotifyProfile { profile: i.read()? })
    }
}

impl dbus::message::SignalArgs for OrgAsuslinuxDaemonNotifyProfile {
    const NAME: &'static str = "NotifyProfile";
    const INTERFACE: &'static str = "org.asuslinux.Daemon";
}