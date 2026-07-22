use generic_api::application::app::App;

pub mod application;

#[tokio::main]
async fn main() {
    let app: App = App::new("res/config.json")
        .await
        .expect("Impossible d'initialiser l'application");
    app.run()
        .await
        .expect("Problème detecter durant l'execution de l'application");
}
