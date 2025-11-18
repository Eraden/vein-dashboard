use std::{io::ErrorKind, str::FromStr, sync::Arc, time::Duration};

use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError, cookie::Cookie,
    http::header::CONTENT_TYPE, web,
};
use serde::Deserialize;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    config::Config,
    identity::SessionManager,
    utils::{Client, ServerEvent, ServerEventBus, Session},
};

mod config;
mod error;
mod game;
mod identity;
mod logs;
mod players;
mod utils;

#[derive(Debug, thiserror::Error)]
#[error("")]
struct Forbidden;

impl ResponseError for Forbidden {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::FORBIDDEN
    }
}

#[actix_web::get("/events")]
async fn events(
    req: HttpRequest,
    event_bus: web::Data<ServerEventBus>,
    session_manager: web::Data<SessionManager>,
    config: web::Data<Arc<RwLock<Config>>>,
) -> Result<impl Responder, Forbidden> {
    use actix_web_lab::sse::*;

    let uuid = if config.read().await.auth_enabled {
        resolve_uuid(req, session_manager.clone()).await
    } else {
        Session::Disabled
    };

    let (tx, rx) = tokio::sync::mpsc::channel(1);
    tokio::spawn(event_loop(
        tx,
        uuid,
        event_bus.clone(),
        session_manager.clone(),
    ));

    Ok(Sse::from_infallible_receiver(rx).with_keep_alive(Duration::from_secs(10)))
}

async fn resolve_uuid(req: HttpRequest, session_manager: web::Data<SessionManager>) -> Session {
    let Some(session) = req.cookie("session") else {
        return Session::None;
    };
    let Some(uuid): Option<Uuid> = Uuid::from_str(session.value()).ok() else {
        return Session::None;
    };
    if !session_manager.exists(uuid).await {
        return Session::None;
    }
    Session::Some(uuid)
}

async fn event_loop(
    tx: tokio::sync::mpsc::Sender<actix_web_lab::sse::Event>,
    session: Session,
    event_bus: web::Data<ServerEventBus>,
    session_manager: web::Data<SessionManager>,
) {
    use actix_web_lab::sse::*;
    let mut bus_receiver = event_bus.receiver();
    tracing::info!("Awaiting server events...");
    while let Ok(event) = bus_receiver.recv().await {
        match session {
            Session::Some(uuid) if !session_manager.exists(uuid).await => {
                emit_forbidden(&tx).await;
                break;
            }
            Session::None => {
                emit_forbidden(&tx).await;
                break;
            }
            _ => {}
        };
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
        if let Session::Some(uuid) = session {
            session_manager.refresh(uuid).await;
        }
    }
    tracing::info!("Server events channel closed");
}

async fn emit_forbidden(tx: &tokio::sync::mpsc::Sender<actix_web_lab::sse::Event>) {
    use actix_web_lab::sse::*;
    tx.send(
        Data::new(serde_json::to_string(&ServerEvent::Forbidden).unwrap())
            .id(uuid::Uuid::new_v4().to_string())
            .event("forbidden")
            .into(),
    )
    .await
    .ok();
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

#[derive(Debug, Deserialize)]
struct AdminSignIn {
    login: String,
    password: String,
}

#[actix_web::post("/sign-in")]
async fn sign_in(
    payload: web::Form<AdminSignIn>,
    admins: web::Data<Admins>,
    session_manager: web::Data<SessionManager>,
) -> impl Responder {
    let is_valid = admins
        .into_inner()
        .0
        .iter()
        .any(|admin| admin.password == payload.password && admin.login == payload.login);
    if !is_valid {
        return HttpResponse::SeeOther()
            .append_header(("Location", "/?forbidden"))
            .finish();
    }
    let uuid = session_manager.new_session().await;
    HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .cookie(
            Cookie::build("session", uuid.to_string())
                .path("/")
                .secure(true)
                .finish(),
        )
        .finish()
}

#[actix_web::get("/app.js")]
async fn app_js() -> HttpResponse {
    #[cfg(debug_assertions)]
    {
        HttpResponse::Ok()
            .insert_header((CONTENT_TYPE, "text/javascript"))
            //.body(tokio::fs::read_to_string("./assets/app.js").await.unwrap())
            .body(tokio::fs::read_to_string("./dist/web.js").await.unwrap())
    }
    #[cfg(not(debug_assertions))]
    {
        HttpResponse::Ok()
            .insert_header((CONTENT_TYPE, "text/javascript"))
            //.body(include_str!("../assets/app.js"))
            .body(include_str!("../dist/web.js"))
    }
}
#[actix_web::get("/web.js.map")]
async fn web_js_map() -> HttpResponse {
    #[cfg(debug_assertions)]
    {
        HttpResponse::Ok()
            .insert_header((CONTENT_TYPE, "text/javascript"))
            .body(
                tokio::fs::read_to_string("./dist/web.js.map")
                    .await
                    .unwrap(),
            )
    }
    #[cfg(not(debug_assertions))]
    {
        HttpResponse::Ok()
            .insert_header((CONTENT_TYPE, "text/javascript"))
            .body(include_str!("../dist/web.js.map"))
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

#[derive(Debug, Clone)]
struct Admin {
    login: String,
    password: String,
}

impl FromStr for Admin {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((login, password)) = s.split_once(':') else {
            panic!("Invalid admin credentials: {s:?}");
        };
        Ok(Self {
            login: login.to_string(),
            password: password.to_string(),
        })
    }
}

#[derive(Debug, Clone)]
struct Admins(Vec<Admin>);

impl FromStr for Admins {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let admins = s
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<Admin>>();
        Ok(Self(admins))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let admins: web::Data<Admins> = web::Data::new(
        std::env::var("ADMINS")
            .expect("No admins found")
            .parse()
            .unwrap(),
    );
    let session_manager = web::Data::new(identity::SessionManager::new());

    let config = load_config().await?;
    let (bind, port) = (
        config
            .bind
            .as_ref()
            .cloned()
            .unwrap_or_else(|| "0.0.0.0".to_string()),
        config.port.unwrap_or(8765),
    );
    let config = Arc::new(RwLock::new(config));
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
            .app_data(admins.clone())
            .app_data(session_manager.clone())
            .service(sign_in)
            .service(events)
            .service(index)
            .service(app_js)
            .service(web_js_map)
            .service(style_css)
    })
    .bind((bind.as_str(), port))?
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
