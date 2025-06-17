pub enum Env {
    Development,
    Production,
}

impl<S: AsRef<str>> From<S> for Env {
    fn from(env: S) -> Self {
        match env.as_ref() {
            "development" | "dev" => Env::Development,
            "production" | "prod" => Env::Production,
            _ => Env::Development,
        }
    }
}

pub enum Config {
    Development {
        url: String,
    },
    Production {
        url: String,
        cert_file: std::path::PathBuf,
        private_key_file: std::path::PathBuf,
    },
}

impl Config {
    pub fn development() -> Self {
        Config::Development {
            url: std::env::var("APP_URL").unwrap_or_else(|_| String::from("localhost:3000")),
        }
    }

    pub fn production() -> Self {
        let url = std::env::var("APP_URL").expect("APP_URL must be set in production");
        // Strip protocol if present to get just host:port for binding
        let bind_addr = if url.starts_with("http://") {
            url.strip_prefix("http://").unwrap()
        } else if url.starts_with("https://") {
            url.strip_prefix("https://").unwrap()
        } else {
            &url
        };

        Config::Production {
            url: bind_addr.to_string(),

            cert_file: std::env::var("TLS_CERT_FILE")
                .map(std::path::PathBuf::from)
                .expect("TLS_CERT_FILE must be set in production"),

            private_key_file: std::env::var("TLS_PRIVATE_KEY_FILE")
                .map(std::path::PathBuf::from)
                .expect("TLS_PRIVATE_KEY_FILE must be set in production"),
        }
    }
}

pub static CONFIG: std::sync::LazyLock<Config> = std::sync::LazyLock::new(|| match std::env::var("APP_ENV") {
    Ok(s) if s == "production" => Config::production(),
    _ => Config::development(),
});
