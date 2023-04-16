use actix_cors::Cors;
use actix_web::{middleware, HttpServer, App, web, HttpRequest, HttpResponse, http::header};
use lib::{Config, get_config, ApiError, AppState};
use actix_web_actors::ws;

use lib::server::WebSocket;
use sea_orm::Database;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use migration::{Migrator, MigratorTrait};

async fn websocket_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    ws::start(WebSocket::new(), &req, stream)
}

#[actix_web::main]
async fn main() -> Result<(), ApiError> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config: Config = get_config().unwrap_or_default();
    
    let conn = Database::connect(&config.database_url).await?;
    let state = AppState {
        conn: conn.clone(),
    };

    Migrator::up(&conn, None).await?;
    Migrator::status(&conn).await?;

    #[derive(OpenApi)]
    #[openapi(
        paths(
            lib::controllers::auth::auth
        ),
        tags(
            (name = "ExotiaCore", description = "ExotiaCore documentation")
        )
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    log::info!("Starting HTTP server at {}:{}", &config.addr, &config.port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(middleware::Logger::default().log_target("ExotiaCore"))
            .wrap(cors)
            .app_data(web::Data::new(state.clone()))
            .configure(lib::controllers::configure())
            .service(web::resource("/ws").route(web::get().to(websocket_handler)))
            .route("/docs", web::get().to(|| async {
                HttpResponse::Found()
                    .insert_header((header::LOCATION, "/docs/"))
                    .finish()
            }))
            .service(
                SwaggerUi::new("/docs/{_:.*}").url("/api-doc/openapi.json", openapi.clone()),
            )
    })
    .workers(config.threads)
    .bind((config.addr, config.port))?
    .run()
    .await?;
    Ok(())
}
