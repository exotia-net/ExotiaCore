use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{middleware, HttpServer, App, web, HttpRequest, HttpResponse, http::header, guard};
use lib::{Config, get_config, ApiError, AppState, utils::token::encrypt, MINECRAFT_ADDRESS, MINECRAFT_PORT, DEFAULT_AUTH, get_auth_key};
use actix_web_actors::ws;

use lib::server::WebSocket;
use sea_orm::Database;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use migration::{Migrator, MigratorTrait};

#[allow(clippy::unused_async)]
async fn websocket_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    ws::start(WebSocket::new(), &req, stream)
}

#[derive(OpenApi)]
#[openapi(
    paths(
        lib::controllers::users::auth::auth,
        lib::controllers::users::create::create,
    ),
    components(
        schemas(lib::entities::users::Model),
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

    HttpServer::new(move || {
        let state = AppState {
            conn: conn.clone(),
            user: Mutex::new(None),
            exotia_key: Mutex::new(None),
        };

        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default().log_target("ExotiaCore"))
            .app_data(web::Data::new(state))
            .service(web::scope("/auth").configure(lib::controllers::users::configure()))
            .service(
                web::scope("/api")
                    .configure(lib::controllers::servers::configure())
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

pub(self) async fn unauthorized() -> HttpResponse { HttpResponse::Unauthorized().finish() }

