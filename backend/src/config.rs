use std::env;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_addr: String,
    pub jwt_secret: String,
    pub smtp: SmtpConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_email: String,
    pub from_name: String,
    pub use_tls: bool,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://ghosthub:ghosthub@localhost/ghosthub".to_string()),
            server_addr: env::var("SERVER_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string()),
            smtp: SmtpConfig {
                // SMTP2GO configuration
                host: env::var("SMTP_HOST").unwrap_or_else(|_| "mail.smtp2go.com".to_string()),
                port: env::var("SMTP_PORT")
                    .unwrap_or_else(|_| "2525".to_string())
                    .parse()
                    .unwrap_or(2525),
                username: env::var("SMTP_USERNAME")
                    .unwrap_or_else(|_| "".to_string()),
                password: env::var("SMTP_PASSWORD") 
                    .unwrap_or_else(|_| "".to_string()),
                from_email: env::var("SMTP_FROM_EMAIL")
                    .unwrap_or_else(|_| "support@cktechx.com".to_string()),
                from_name: env::var("SMTP_FROM_NAME")
                    .unwrap_or_else(|_| "GhostHub Support".to_string()),
                use_tls: env::var("SMTP_USE_TLS")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
            },
        })
    }
}