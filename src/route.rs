use askama::Template;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

pub async fn index() -> impl IntoResponse {
    Index::get().await
}

pub async fn login(name: Path<String>) -> impl IntoResponse {
    Login::get(name).await
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

impl Index {
    fn new() -> Self {
        Self
    }

    async fn get() -> impl IntoResponse {
        Self::new()
    }
}

impl IntoResponse for Index {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("500: Failed to render template: {err}"),
            )
                .into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "login.html")]
struct Login {
    name: String,
}

impl Login {
    fn new(name: String) -> Self {
        Self { name }
    }

    async fn get(Path(name): Path<String>) -> impl IntoResponse {
        Self::new(name)
    }
}

impl IntoResponse for Login {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("500: Failed to render template: {err}"),
            )
                .into_response(),
        }
    }
}
