use std::fmt::Display;

use axum_login::secrecy::SecretVec;
use axum_login::AuthUser;
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct User {
    name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl AuthUser for User {
    fn get_id(&self) -> String {
        self.name.clone()
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(Default::default())
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}
