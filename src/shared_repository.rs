
pub static schemas: Vec<(&str, &str, Vec<&str>)> = vec![
    (
        "events",
        vec![
            "id",
            "relation_id",
            "reference_id",
            "json_unquote(tags) tags",
            "json_unquote(detail) detail",
            "date_format(time, '%Y-%m-%d %H:%i:%s') time"
        ],
    ),
]
