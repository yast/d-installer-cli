//! # DBus interface proxies for: `org.opensuse.DInstaller.**.*`
//!
//! This code was generated by `zbus-xmlgen` `3.1.0` from DBus introspection data.`.
//!
//!
//! These DBus objects implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::PropertiesProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.
//! Also some proxies can be used against multiple services when they share interface.

use zbus::dbus_proxy;

/// Progress1Proxy can be used also with Software and Storage object.
///
/// TODO: example
#[dbus_proxy(
    interface = "org.opensuse.DInstaller.Progress1",
    default_service = "org.opensuse.DInstaller",
    default_path = "/org/opensuse/DInstaller/Manager1",
    gen_async = false,
    gen_blocking = true
)]
trait Progress1 {
    /// CurrentStep property
    #[dbus_proxy(property)]
    fn current_step(&self) -> zbus::Result<(u32, String)>;

    /// Finished property
    #[dbus_proxy(property)]
    fn finished(&self) -> zbus::Result<bool>;

    /// TotalSteps property
    #[dbus_proxy(property)]
    fn total_steps(&self) -> zbus::Result<u32>;
}

/// ServiceStatus1Proxy can be used also for other objects.
#[dbus_proxy(
    interface = "org.opensuse.DInstaller.ServiceStatus1",
    default_service = "org.opensuse.DInstaller",
    default_path = "/org/opensuse/DInstaller/Manager1",
    gen_async = false,
    gen_blocking = true
)]
trait ServiceStatus1 {
    /// All property
    #[dbus_proxy(property)]
    fn all(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// Current property
    #[dbus_proxy(property)]
    fn current(&self) -> zbus::Result<u32>;
}

#[dbus_proxy(
    interface = "org.opensuse.DInstaller.Manager1",
    default_service = "org.opensuse.DInstaller",
    default_path = "/org/opensuse/DInstaller/Manager1",
    gen_async = false,
    gen_blocking = true
)]
trait Manager1 {
    /// CanInstall method
    fn can_install(&self) -> zbus::Result<bool>;

    /// Commit method
    fn commit(&self) -> zbus::Result<()>;

    /// Probe method
    fn probe(&self) -> zbus::Result<()>;

    /// BusyServices property
    #[dbus_proxy(property)]
    fn busy_services(&self) -> zbus::Result<Vec<String>>;

    /// CurrentInstallationPhase property
    #[dbus_proxy(property)]
    fn current_installation_phase(&self) -> zbus::Result<u32>;

    /// InstallationPhases property
    #[dbus_proxy(property)]
    fn installation_phases(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;
}

#[dbus_proxy(
    interface = "org.opensuse.DInstaller.Language1",
    default_service = "org.opensuse.DInstaller.Language",
    default_path = "/org/opensuse/DInstaller/Language1"
)]
trait Language1 {
    /// Finish method
    fn finish(&self) -> zbus::Result<()>;

    /// ToInstall method
    fn to_install(&self, lang_ids: &[&str]) -> zbus::Result<()>;

    /// AvailableLanguages property
    #[dbus_proxy(property)]
    fn available_languages(
        &self,
    ) -> zbus::Result<
        Vec<(
            String,
            String,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
    >;

    /// MarkedForInstall property
    #[dbus_proxy(property)]
    fn marked_for_install(&self) -> zbus::Result<Vec<String>>;
}

#[dbus_proxy(
    interface = "org.opensuse.DInstaller.Questions1",
    default_service = "org.opensuse.DInstaller.Questions",
    default_path = "/org/opensuse/DInstaller/Questions1"
)]
trait Questions1 {
    /// Delete method
    fn delete(&self, question: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// New method
    #[dbus_proxy(name = "New")]
    fn create(
        &self,
        text: &str,
        options: &[&str],
        default_option: &[&str],
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// NewLuksActivation method
    fn new_luks_activation(
        &self,
        device: &str,
        label: &str,
        size: &str,
        attempt: u8,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
}

#[dbus_proxy(
    interface = "org.opensuse.DInstaller.Software1",
    default_service = "org.opensuse.DInstaller.Software",
    default_path = "/org/opensuse/DInstaller/Software1"
)]
trait Software1 {
    /// Finish method
    fn finish(&self) -> zbus::Result<()>;

    /// Install method
    fn install(&self) -> zbus::Result<()>;

    /// IsPackageInstalled method
    fn is_package_installed(&self, name: &str) -> zbus::Result<bool>;

    /// Probe method
    fn probe(&self) -> zbus::Result<()>;

    /// Propose method
    fn propose(&self) -> zbus::Result<()>;

    /// ProvisionSelected method
    fn provision_selected(&self, provision: &str) -> zbus::Result<bool>;

    /// ProvisionsSelected method
    fn provisions_selected(&self, provisions: &[&str]) -> zbus::Result<Vec<bool>>;

    /// SelectProduct method
    fn select_product(&self, product_id: &str) -> zbus::Result<()>;

    /// AvailableBaseProducts property
    #[dbus_proxy(property)]
    fn available_base_products(
        &self,
    ) -> zbus::Result<
        Vec<(
            String,
            String,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
    >;

    /// SelectedBaseProduct property
    #[dbus_proxy(property)]
    fn selected_base_product(&self) -> zbus::Result<String>;
}

#[dbus_proxy(
    interface = "org.opensuse.DInstaller.Software.Proposal1",
    default_service = "org.opensuse.DInstaller.Software",
    default_path = "/org/opensuse/DInstaller/Software/Proposal1"
)]
trait SoftwareProposal1 {
    /// AddResolvables method
    fn add_resolvables(
        &self,
        id: &str,
        r#type: u8,
        resolvables: &[&str],
        optional: bool,
    ) -> zbus::Result<()>;

    /// GetResolvables method
    fn get_resolvables(&self, id: &str, r#type: u8, optional: bool) -> zbus::Result<Vec<String>>;

    /// RemoveResolvables method
    fn remove_resolvables(
        &self,
        id: &str,
        r#type: u8,
        resolvables: &[&str],
        optional: bool,
    ) -> zbus::Result<()>;

    /// SetResolvables method
    fn set_resolvables(
        &self,
        id: &str,
        r#type: u8,
        resolvables: &[&str],
        optional: bool,
    ) -> zbus::Result<()>;
}

#[dbus_proxy(
    interface = "org.opensuse.DInstaller.Validation1",
    default_service = "org.opensuse.DInstaller.Storage",
    default_path = "/org/opensuse/DInstaller/Storage1"
)]
trait Validation1 {
    /// Errors property
    #[dbus_proxy(property)]
    fn errors(&self) -> zbus::Result<Vec<String>>;

    /// Valid property
    #[dbus_proxy(property)]
    fn valid(&self) -> zbus::Result<bool>;
}

#[dbus_proxy(
    interface = "org.opensuse.DInstaller.Storage1",
    default_service = "org.opensuse.DInstaller.Storage",
    default_path = "/org/opensuse/DInstaller/Storage1",
    gen_async = false,
    gen_blocking = true
)]
trait Storage1 {
    /// Finish method
    fn finish(&self) -> zbus::Result<()>;

    /// Install method
    fn install(&self) -> zbus::Result<()>;

    /// Probe method
    fn probe(&self) -> zbus::Result<()>;
}

#[dbus_proxy(
    default_service = "org.opensuse.DInstaller.Storage",
    default_path = "/org/opensuse/DInstaller/Storage1",
    interface = "org.opensuse.DInstaller.Storage1.Proposal.Calculator",
    assume_defaults = true,
    gen_async = false,
    gen_blocking = true
)]
trait Calculator {
    /// Calculate method
    fn calculate(
        &self,
        settings: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<u32>;

    /// AvailableDevices property
    #[dbus_proxy(property)]
    fn available_devices(
        &self,
    ) -> zbus::Result<
        Vec<(
            String,
            String,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
    >;

    /// Result property
    #[dbus_proxy(property)]
    fn result(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// VolumeTemplates property
    #[dbus_proxy(property)]
    fn volume_templates(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;
}

#[dbus_proxy(
    interface = "org.opensuse.DInstaller.Storage1.Proposal",
    default_service = "org.opensuse.DInstaller.Storage",
    default_path = "/org/opensuse/DInstaller/Storage1/Proposal",
    gen_async = false,
    gen_blocking = true
)]
trait StorageProposal {
    /// Actions property
    #[dbus_proxy(property)]
    fn actions(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// CandidateDevices property
    #[dbus_proxy(property)]
    fn candidate_devices(&self) -> zbus::Result<Vec<String>>;

    /// EncryptionPassword property
    #[dbus_proxy(property)]
    fn encryption_password(&self) -> zbus::Result<String>;

    /// LVM property
    #[dbus_proxy(property, name = "LVM")]
    fn lvm(&self) -> zbus::Result<bool>;

    /// Volumes property
    #[dbus_proxy(property)]
    fn volumes(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;
}

#[dbus_proxy(
    interface = "org.opensuse.DInstaller.Users1",
    default_service = "org.opensuse.DInstaller.Users",
    default_path = "/org/opensuse/DInstaller/Users1",
    gen_async = false,
    gen_blocking = true
)]
trait Users1 {
    /// RemoveFirstUser method
    fn remove_first_user(&self) -> zbus::Result<u32>;

    /// RemoveRootPassword method
    fn remove_root_password(&self) -> zbus::Result<u32>;

    /// SetFirstUser method
    fn set_first_user(
        &self,
        full_name: &str,
        user_name: &str,
        password: &str,
        auto_login: bool,
        data: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<(bool, Vec<String>)>;

    /// SetRootPassword method
    fn set_root_password(&self, value: &str, encrypted: bool) -> zbus::Result<u32>;

    /// SetRootSSHKey method
    #[dbus_proxy(name = "SetRootSSHKey")]
    fn set_root_sshkey(&self, value: &str) -> zbus::Result<u32>;

    /// Write method
    fn write(&self) -> zbus::Result<u32>;

    /// FirstUser property
    #[dbus_proxy(property)]
    fn first_user(
        &self,
    ) -> zbus::Result<(
        String,
        String,
        bool,
        std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
    )>;

    /// RootPasswordSet property
    #[dbus_proxy(property)]
    fn root_password_set(&self) -> zbus::Result<bool>;

    /// RootSSHKey property
    #[dbus_proxy(property, name = "RootSSHKey")]
    fn root_sshkey(&self) -> zbus::Result<String>;
}
