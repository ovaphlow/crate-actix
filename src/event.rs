use actix_web::{HttpRequest, Responder, HttpResponse, web};
use serde::Serialize;
use sqlx::prelude::*;

pub async fn get_event(pool: web::Data<sqlx::MySqlPool>, req: HttpRequest) -> impl Responder {
// pub async fn get_event(req: HttpRequest) -> impl Responder {
    print!("{} {}\n", req.method().to_string(), req.uri().to_string());

    let result = sqlx::query("SELECT * FROM events")
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(rows) => {
            let events: Vec<Event> = rows.iter().map(|row| Event {
                id: row.get("id"),
                relation_id: row.get("relation_id"),
                reference_id: row.get("reference_id"),
                tags: row.get::<String, _>("tags"),
                detail: row.get::<String, _>("detail"),
                time: row.get::<String, _>("time"),
            }).collect();
            HttpResponse::Ok().json(events)
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Serialize)]
struct Event {
    id: i64,
    relation_id: i64,
    reference_id: i64,
    tags: String,
    detail: String,
    time: String,
}
