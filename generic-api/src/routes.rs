use axum::routing::MethodRouter;
use std::sync::Arc;

use crate::application::state::AppState;

pub struct ApiRoute {
    pub uri: &'static str,
    pub handler: fn() -> MethodRouter<Arc<AppState>>,
}

impl ApiRoute {
    pub const fn new(uri: &'static str, handler: fn() -> MethodRouter<Arc<AppState>>) -> Self {
        ApiRoute { uri, handler }
    }
}

inventory::collect!(ApiRoute);
