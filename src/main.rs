use std::sync::Arc;
use actix_cors::Cors;
use actix_web::{middleware, HttpServer, App, web, HttpRequest, HttpResponse, http::header, guard};
use futures::lock::Mutex;
use lib::{Config, get_config, ApiError, AppState, utils::token::encrypt, MINECRAFT_ADDRESS, MINECRAFT_PORT, DEFAULT_AUTH, get_auth_key};
use actix_web_actors::ws;

use lib::server::WebSocket;
use sea_orm::Database;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use migration::{Migrator, MigratorTrait};

#[allow(clippy::unused_async)]
async fn websocket_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    ws::start(WebSocket::new(req.clone()), &req, stream)
}

// This just looks bad
// But it works ¯\_(ツ)_/¯
#[derive(OpenApi)]
#[openapi(
    paths(
        // Calendars
        lib::controllers::calendars::get::get,
        lib::controllers::calendars::update::update,
        lib::controllers::calendars::rewards::rewards,

        // Servers
        lib::controllers::servers::get::get,
        lib::controllers::servers::economy::economy,
        lib::controllers::servers::economy_top::economy_top,

        // Users
        lib::controllers::users::auth::auth,
        lib::controllers::users::create::create,
        lib::controllers::users::update::update,
        lib::controllers::users::setup::setup,

        // Wallet
        lib::controllers::wallet::get::get,
        lib::controllers::wallet::buy::buy,

        // Calendars (Websocket)
        lib::websocket_handlers::calendars::get::get,
        lib::websocket_handlers::calendars::update::update,
        lib::websocket_handlers::calendars::rewards::rewards,

        // Servers (WebSocket)
        lib::websocket_handlers::servers::economy::economy,
        lib::websocket_handlers::servers::economy_add::economy_add,
        lib::websocket_handlers::servers::get::get,

        // Wallet (WebSocket)
        lib::websocket_handlers::wallet::buy::buy,
        lib::websocket_handlers::wallet::get::get,

        // Public
        lib::websocket_handlers::public::get_online::get_online,
    ),
    components(
        // Calendars
        schemas(lib::controllers::calendars::CalendarEntity),

        // Users
        schemas(lib::controllers::users::UserEntity),

        // Servers
        schemas(lib::controllers::servers::Economy),
        schemas(lib::controllers::servers::ServerType),
        schemas(lib::controllers::servers::TopsQuery),
        schemas(lib::controllers::servers::EconomyTop),

        // Wallet
        schemas(lib::controllers::wallet::WalletBuy),

        // Entities
        schemas(lib::entities::calendars::Model),
        schemas(lib::entities::users::Model),
        schemas(lib::entities::servers::Model),
        schemas(lib::entities::wallet::Model),
    ),
    tags(
        (name = "ExotiaCore", description = "ExotiaCore documentation")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> Result<(), ApiError> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config: Config = get_config().unwrap_or_default();

    unsafe {
        *MINECRAFT_ADDRESS = config.minecraft_address;
        MINECRAFT_PORT = config.minecraft_port;
        *DEFAULT_AUTH = encrypt(&DEFAULT_AUTH, &config.key);
    }

    let conn = Database::connect(&config.database_url).await?;

    // Migrator::refresh(&conn).await?;
    Migrator::up(&conn, None).await?;
    Migrator::status(&conn).await?;

    let openapi = ApiDoc::openapi();

    log::info!("Starting HTTP server at {}:{}", &config.addr, &config.port);

    let state = AppState {
        conn: Arc::new(Mutex::new(conn)),
        user: Arc::new(Mutex::new(None)),
        exotia_key: Arc::new(Mutex::new(None)),
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default().log_target("ExotiaCore"))
            .app_data(web::Data::new(state.clone()))
            .service(web::scope("/auth").configure(lib::controllers::users::configure()))
            .service(
                web::scope("/api")
                    .configure(lib::controllers::servers::configure())
                    .configure(lib::controllers::wallet::configure())
                    .configure(lib::controllers::calendars::configure())
            )
            .service(
                web::resource("/ws")
                    .default_service(web::route().to(unauthorized))
                    .route(web::get().guard(guard::Header("ExotiaKey", get_auth_key())).to(websocket_handler))
            )
            .route("/docs", web::get().to(|| async { HttpResponse::Found().insert_header((header::LOCATION, "/docs/")).finish() }))
            .service(SwaggerUi::new("/docs/{_:.*}").url("/api-doc/openapi.json", openapi.clone()))
    })
    .workers(config.threads)
    .bind((config.addr, config.port))?
    .run()
    .await?;
    Ok(())
}

#[allow(clippy::unused_async)]
pub(self) async fn unauthorized() -> HttpResponse { HttpResponse::Unauthorized().finish() }

