use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::Path;
use hmac::{Hmac, Mac};
use sha2::Sha256;

// Type alias for cryptographic health signatures
type HmacSha256 = Hmac<Sha256>;

pub async fn init_db(app_dir: &Path) -> Result<SqlitePool, sqlx::Error> {
    let db_path = app_dir.join("qleaner.db");
    
    if !db_path.exists() {
        std::fs::File::create(&db_path).map_err(|e| sqlx::Error::Io(e.into()))?;
    }
    
    let db_url = format!("sqlite://{}", db_path.display());
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
        
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS audit_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL,
            size_reclaimed INTEGER NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            signature TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS schedules (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            cron_expr TEXT NOT NULL,
            is_active BOOLEAN NOT NULL DEFAULT 1
        );
        CREATE TABLE IF NOT EXISTS excluded_paths (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL UNIQUE
        );
        "#
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

pub fn sign_metrics(path: &str, size: u64, secret: &[u8]) -> String {
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC can take key of any size");
    mac.update(path.as_bytes());
    mac.update(&size.to_be_bytes());
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

pub async fn insert_audit_log(pool: &SqlitePool, path: &str, size_reclaimed: u64, secret: &[u8]) -> Result<(), sqlx::Error> {
    let signature = sign_metrics(path, size_reclaimed, secret);
    sqlx::query(
        "INSERT INTO audit_logs (path, size_reclaimed, signature) VALUES (?, ?, ?)"
    )
    .bind(path)
    .bind(size_reclaimed as i64)
    .bind(signature)
    .execute(pool)
    .await?;
    Ok(())
}
