use actix_cors::Cors;
use actix_web::{middleware, HttpServer, App, web, HttpRequest, HttpResponse, http::header};
use lib::{Config, load_config};
use actix_web_actors::ws;

use lib::server::WebSocket;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;


async fn websocket_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    ws::start(WebSocket::new(), &req, stream)
} 

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config: Config = load_config().unwrap_or_default();

    #[derive(OpenApi)]
    #[openapi(
        paths(
            lib::controllers::hello
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
    .await
}
