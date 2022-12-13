use axum_login::memory_store::MemoryStore;
use axum_login::{extractors, RequireAuthorizationLayer};
use log::info;

use crate::user::User;

pub type AuthContext = extractors::AuthContext<User, MemoryStore<User>>;
pub type RequireAuth = RequireAuthorizationLayer<User>;

pub async fn login(mut auth: AuthContext, user: User) {
    auth.login(&user).await.unwrap();
    info!("logged in: {}", auth.current_user.unwrap());
}

pub async fn logout(mut auth: AuthContext) {
    auth.logout().await;
    info!("logged out");
}
