use actix_web::{web, Responder, HttpResponse};
use migration::{Value, ArrayType};
use sea_orm::{ConnectionTrait, Statement};
use serde_json::json;
use crate::{ApiError, AppState};

use super::UserEntity;

pub async fn update(
    body: web::Json<Vec<UserEntity>>,
    data: web::Data<AppState>,
) -> Result<impl Responder, ApiError> {
    let body = body.into_inner();

    let uuids = body.iter().map(|v| v.uuid.clone()).collect::<Vec<String>>();
    let last_ips= body.iter().map(|v| v.last_ip.clone()).collect::<Vec<String>>();
    let nicks = body.iter().map(|v| v.nick.clone()).collect::<Vec<String>>();

    let query_sql = r#"
        update users
        set (last_ip, nick) = (tmp_table.last_ip, tmp_table.nick)
        from (select 
            unnest($1::VARCHAR[]) as uuid,
            unnest($2::VARCHAR[]) as last_ip,
            unnest($3::VARCHAR[]) as nick
        ) as tmp_table
        where users.uuid = tmp_table.uuid;
        "#;

    let mut params = Vec::new();
    let mut uuid_values = Vec::new();
    let mut last_ip_values = Vec::new();
    let mut nick_values = Vec::new();

    for uuid in uuids.iter() { uuid_values.push(Value::String(Some(Box::new(uuid.clone())))); }
    params.push(Value::Array(ArrayType::String, Some(Box::new(uuid_values))));

    for ip in last_ips.iter() { last_ip_values.push(Value::String(Some(Box::new(ip.clone())))); }
    params.push(Value::Array(ArrayType::String, Some(Box::new(last_ip_values))));

    for nick in nicks.iter() { nick_values.push(Value::String(Some(Box::new(nick.clone())))); }
    params.push(Value::Array(ArrayType::String, Some(Box::new(nick_values))));

    let query_statement = Statement::from_sql_and_values(sea_orm::DatabaseBackend::Postgres, &query_sql, params);

    data.conn.execute(query_statement).await?;

    Ok(
        HttpResponse::Ok().json(json!{ "test" })
    )
}
