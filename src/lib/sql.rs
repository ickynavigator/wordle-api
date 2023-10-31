pub const INIT_TABLE: &str = r#"
CREATE TABLE games (
    id TEXT NOT NULL PRIMARY KEY,
    creator_id TEXT NOT NULL,
    answer TEXT NOT NULL,
    attempts TEXT [],
    start_time INT
);
"#;
