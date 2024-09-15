use leptos::config::LeptosOptions;

use axum::extract::FromRef;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
}
