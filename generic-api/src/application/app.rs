use std::sync::Arc;

use generic_tool::read_json;

use crate::{application::state::AppState, config::Config, routes::ApiRoute};

pub struct App {
    app_state: Arc<AppState>,
    config: Config,
}

impl App {
    pub fn new(config_path: &str) -> anyhow::Result<Self> {
        let config: Config = read_json::<Config>(config_path.to_string())?;
        let state: AppState = Self::build_state(&config)?;

        anyhow::Ok(Self {
            app_state: Arc::new(state),
            config,
        })
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let app = self.build_router()?;

        let listener = tokio::net::TcpListener::bind(self.config.addr.format()?).await?;

        axum::serve(listener, app).await?;
        Ok(())
    }

    fn build_router(&self) -> anyhow::Result<axum::Router> {
        let mut app = axum::Router::new();

        //initialise tous les handler
        for route_api in inventory::iter::<ApiRoute> {
            app = app.route(route_api.uri, (route_api.handler)())
        }

        anyhow::Ok(app.with_state(Arc::clone(&self.app_state)))
    }

    // ---- Construction du Appstate ----

    fn build_state(_config: &Config) -> anyhow::Result<AppState> {
        anyhow::Ok(AppState {})
    }
}
