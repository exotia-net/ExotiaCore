use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{middleware, HttpServer, App, web, HttpRequest, HttpResponse, http::header, dev::{ServiceRequest, ServiceResponse}, body::MessageBody};
use lib::{Config, get_config, ApiError, AppState, utils::token::decrypt, entities::{users, prelude::*}, UserInfoTrait};
use actix_web_actors::ws;
use actix_web_lab::middleware::{Next, from_fn};

use lib::server::WebSocket;
use sea_orm::{Database, ColumnTrait, EntityTrait, QueryFilter};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use migration::{Migrator, MigratorTrait};

#[allow(clippy::unused_async)]
async fn websocket_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    ws::start(WebSocket::new(), &req, stream)
}

async fn validate_token(token: &str, data: &web::Data<AppState>) -> Option<lib::entities::users::Model> {
    let key = get_config().ok()?.key;
    let plain_token = decrypt(token, &key).ok()?;
    let user_info = plain_token.extract();
    let uuid = user_info.uuid.clone();
    let mut exotia_key = data.exotia_key.lock().unwrap();
    *exotia_key = Some(user_info);
    drop(exotia_key);
    
    Users::find()
        .filter(users::Column::Uuid.like(uuid.as_str()))
        .one(&data.conn)
        .await
        .ok()?
}

async fn auth_middleware(
    data: web::Data<AppState>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let token = req
        .headers()
        .get("ExotiaKey")
        .and_then(|value| value.to_str().ok()).map(str::to_owned);
    
    let Some(token_v) = token else {
        return Err(actix_web::error::ErrorUnauthorized(""));
    };
    
    let call = match validate_token(&token_v, &data).await {
        Some(v) => {
            let mut user = data.user.lock().unwrap();
            *user = Some(v);
            drop(user);
            next.call(req).await
        },
        None => return Err(actix_web::error::ErrorUnauthorized(""))
    };
    //After Request
    call
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
            .service(
                web::scope("/auth")
                    .wrap(from_fn(auth_middleware))
                    .route("/me", web::get().to(lib::controllers::users::auth::auth))
                    .route("/signUp", web::post().to(lib::controllers::users::create::create))
                    // .configure(lib::controllers::users::blocked())
            )
            .service(
                web::scope("/api")
                    .wrap(from_fn(auth_middleware))
                    .route("/servers/:serverId", web::get().to(lib::controllers::servers::get::get))
            )
            // .service(
            //     web::resource("/auth/signUp").route(web::post().to(lib::controllers::users::create::create))
            //         // .configure(lib::controllers::users::configure())
            // )
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
