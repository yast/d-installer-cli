use super::proxies::Users1Proxy;
use crate::attributes::{AttributeValue, Attributes};
use serde::Serialize;
use zbus::blocking::Connection;

pub struct UsersClient<'a> {
    users_proxy: Users1Proxy<'a>,
}

#[derive(Serialize, Debug, Default)]
pub struct FirstUser {
    pub full_name: String,
    pub user_name: String,
    pub password: String,
    pub autologin: bool,
    pub data: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
}

impl FirstUser {
    fn from_dbus(
        dbus_data: zbus::Result<(
            String,
            String,
            bool,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
    ) -> zbus::Result<Self> {
        let data = dbus_data?;
        Ok(Self {
            full_name: data.0,
            user_name: data.1,
            autologin: data.2,
            data: data.3,
            password: "".to_string(),
        })
    }
}

impl Attributes for FirstUser {
    fn set_attribute(&mut self, attr: &str, value: AttributeValue) -> Result<(), &'static str> {
        match attr {
            "full_name" => self.full_name = value.try_into()?,
            "user_name" => self.user_name = value.try_into()?,
            "password" => self.password = value.try_into()?,
            "autologin" => self.autologin = value.try_into()?,
            _ => return Err("unknown attribute"),
        }
        Ok(())
    }
}

impl<'a> UsersClient<'a> {
    pub fn new(connection: Connection) -> zbus::Result<Self> {
        Ok(Self {
            users_proxy: Users1Proxy::new(&connection)?,
        })
    }

    /// Returns the settings for first non admin user
    pub fn first_user(&self) -> zbus::Result<FirstUser> {
        FirstUser::from_dbus(self.users_proxy.first_user())
    }

    pub fn is_root_password(&self) -> zbus::Result<bool> {
        self.users_proxy.root_password_set()
    }

    pub fn root_ssh_key(&self) -> zbus::Result<String> {
        self.users_proxy.root_sshkey()
    }

    pub fn set_first_user(&self, first_user: &FirstUser) -> zbus::Result<(bool, Vec<String>)> {
        self.users_proxy.set_first_user(
            &first_user.full_name,
            &first_user.user_name,
            &first_user.password,
            first_user.autologin,
            std::collections::HashMap::new(),
        )
    }
}
