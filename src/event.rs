use std::collections::HashMap;

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use serde_json::json;
use sqlx::prelude::*;

use crate::AppState;
use crate::condition_builder::equal_builder;

const COLUMNS: [&str; 6] = ["id", "relation_id", "reference_id", "json_unquote(tags) tags", "json_unquote(detail) detail", "date_format(time, '%Y-%m-%d %H:%i:%s') time"];

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

// pub async fn test_event(app_data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
//     let result = retrieve(
//         app_data.db.clone(),
//         "events".to_string(),
//         0,
//         10,
//         HashMap::new()
//     ).await;
//     let body = match result {
//         Ok(data) => serde_json::to_string(&data).unwrap_or_else(|_| String::from("Error converting to JSON")),
//         Err(e) => format!("Error: {}", e),
//     };
//     HttpResponse::Ok().body(body)
// }

pub async fn retrieve_event(app_data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let query_string = req.query_string();
    let query: HashMap<String, String> = serde_qs::from_str(query_string).unwrap();
    if !query.contains_key("option") {
        return HttpResponse::NotAcceptable().json(json!({
            "error": "option is required"
        }));
    }
    if query.get("option") == Some(&"default".to_string()) {
        return filter_event_default(app_data, query).await;
    }
    HttpResponse::Ok().body("ok")
}

async fn filter_event_default(app_data: web::Data<AppState>, query: HashMap<String, String>) -> HttpResponse {
    let mut q = format!("select {} from events", COLUMNS.join(", "));
    let mut conditions: Vec<String> = Vec::new();
    let mut params: Vec<String> = Vec::new();
    if query.contains_key("equal") {
        let equal = query.get("equal").unwrap();
        let (c, p) = equal_builder(equal.split(",").collect::<Vec<&str>>().as_slice());
        conditions.extend(c);
        params.extend(p);
    }
    if conditions.len() > 0 {
        q.push_str(&format!(" where {}", conditions.join(" and ")));
    }
    q.push_str(&format!(" order by {} desc", "id"));
    q.push_str(&format!(" limit {}, {}", "0", "10"));
    print!("{}\n", q);
    let mut query = sqlx::query(&q);
    for param in &params {
        query = query.bind(param);
    }
    let result = query.fetch_all(&app_data.db).await;
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

pub async fn get_event(app_data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    print!("{} {}\n", req.method().to_string(), req.uri().to_string());

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
