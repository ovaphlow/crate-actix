use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use sqlx::prelude::*;

use crate::AppState;
use crate::shared_repository::schemas;

#[derive(Serialize)]
struct Event {
    id: i64,
    #[serde(rename = "relationId")]
    relation_id: i64,
    #[serde(rename = "referenceId")]
    reference_id: i64,
    tags: String,
    detail: String,
    time: String,
    _id: String,
    #[serde(rename = "_relationId")]
    _relation_id: String,
    #[serde(rename = "_referenceId")]
    _reference_id: String,
}

pub async fn get_event(app_data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    print!("{} {}\n", req.method().to_string(), req.uri().to_string());

    let columns: String = schemas.iter().find(|(table, _)| table == &"events").unwrap().1.join(", ");
    println!(columns);
    // let q: String = r#"
    // select {}
    // "#;

    let result: Result<Vec<sqlx::mysql::MySqlRow>, sqlx::Error> = sqlx::query(
        r#"
        select id
            , relation_id
            , reference_id
            , json_unquote(tags) tags
            , json_unquote(detail) detail
            , date_format(time, '%Y-%m-%d %H:%i:%s') time
        from events
        "#,
    )
    .fetch_all(&app_data.db)
    .await;

    match result {
        Ok(rows) => {
            let events: Result<Vec<Event>, sqlx::Error> = rows
                .iter()
                .map(|row| {
                    Ok(Event {
                        id: row.get::<i64, _>("id"),
                        relation_id: row.get::<i64, _>("relation_id"),
                        reference_id: row.get::<i64, _>("reference_id"),
                        tags: row.get::<String, _>("tags"),
                        detail: row.get::<String, _>("detail"),
                        time: row.get::<String, _>("time"),
                        _id: row.get::<i64, _>("id").to_string(),
                        _relation_id: row.get::<i64, _>("relation_id").to_string(),
                        _reference_id: row.get::<i64, _>("reference_id").to_string(),
                    })
                })
                .collect();
            match events {
                Ok(events) => HttpResponse::Ok().json(events),
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
