use std::{io::ErrorKind, sync::Arc, time::Duration};

use actix_web::{App, HttpResponse, HttpServer, Responder, http::header::CONTENT_TYPE, web};
use tokio::sync::RwLock;

use crate::{
    config::Config,
    utils::{Client, ServerEventBus},
};

mod config;
mod error;
mod game;
mod identity;
mod logs;
mod players;
mod utils;

#[actix_web::get("/events")]
async fn events(event_bus: web::Data<ServerEventBus>) -> impl Responder {
    use actix_web_lab::sse::*;

    let (tx, rx) = tokio::sync::mpsc::channel(1);
    let mut bus_receiver = event_bus.receiver();
    tokio::spawn(async move {
        tracing::info!("Awaiting server events...");
        while let Ok(event) = bus_receiver.recv().await {
            tracing::debug!("New server event: {event:?}");

            let ev = Data::new(serde_json::to_string(&event).unwrap())
                .id(uuid::Uuid::new_v4().to_string())
                .event("server_event");
            let Ok(_) = tx
                .send(ev.into())
                .await
                .inspect_err(|e| tracing::warn!("Event bus died: {e}"))
            else {
                break;
            };
        }
        tracing::info!("Server events channel closed");
    });

    Sse::from_infallible_receiver(rx).with_keep_alive(Duration::from_secs(10))
}

#[actix_web::get("/")]
async fn index() -> HttpResponse {
    #[cfg(debug_assertions)]
    {
        HttpResponse::Ok()
            .insert_header((CONTENT_TYPE, "text/html"))
            .body(
                tokio::fs::read_to_string("./assets/index.html")
                    .await
                    .unwrap(),
            )
    }
    #[cfg(not(debug_assertions))]
    {
        HttpResponse::Ok()
            .insert_header((CONTENT_TYPE, "text/html"))
            .body(include_str!("../assets/index.html"))
    }
}

#[actix_web::get("/app.js")]
async fn app_js() -> HttpResponse {
    #[cfg(debug_assertions)]
    {
        HttpResponse::Ok()
            .insert_header((CONTENT_TYPE, "text/javascript"))
            .body(tokio::fs::read_to_string("./assets/app.js").await.unwrap())
    }
    #[cfg(not(debug_assertions))]
    {
        HttpResponse::Ok()
            .insert_header((CONTENT_TYPE, "text/javascript"))
            .body(include_str!("../assets/app.js"))
    }
}
#[actix_web::get("/style.css")]
async fn style_css() -> HttpResponse {
    #[cfg(debug_assertions)]
    {
        HttpResponse::Ok()
            .insert_header((CONTENT_TYPE, "text/css"))
            .body(
                tokio::fs::read_to_string("./assets/style.css")
                    .await
                    .unwrap(),
            )
    }
    #[cfg(not(debug_assertions))]
    {
        HttpResponse::Ok()
            .insert_header((CONTENT_TYPE, "text/css"))
            .body(include_str!("../assets/style.css"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let config = Arc::new(RwLock::new(load_config().await?));
    let client = Client::new(config.clone());
    let event_bus = ServerEventBus::new();

    {
        let config = config.read().await;
        crate::game::agent::start(client.clone(), &event_bus, &*config);
        crate::logs::start(&event_bus, &*config);
        crate::players::agent::start(client.clone(), &event_bus, &*config);
    }

    let event_bus = web::Data::new(event_bus);
    let client = web::Data::new(client);
    let config = web::Data::new(config);

    HttpServer::new(move || {
        App::new()
            .app_data(event_bus.clone())
            .app_data(client.clone())
            .app_data(config.clone())
            .service(events)
            .service(index)
            .service(app_js)
            .service(style_css)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

async fn load_config() -> std::io::Result<Config> {
    let config = tokio::fs::read_to_string("./config.toml").await;

    let config = match config {
        Ok(config) => toml::from_str(&config).expect("Malformed config file"),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            let config = Config::default();
            let contents = toml::to_string(&config).expect("Unable to serialize config");
            tokio::fs::write("./config.toml", contents).await?;
            config
        }
        Err(e) => return Err(e),
    };

    Ok(config)
}
