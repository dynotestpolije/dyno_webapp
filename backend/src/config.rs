use dyno_core::log;
use std::path::PathBuf;
use std::process::exit;

#[inline]
pub fn get_env<S: AsRef<str>>(env: S) -> String {
    let env = env.as_ref();
    match std::env::var(env) {
        Ok(value) => {
            log::debug!("Success getting ENV: [{env}]");
            value
        }
        Err(err) => {
            log::error!("Failed to get config paths for ENV:`{env}` - {err}");
            exit(1);
        }
    }
}

#[inline]
pub fn get_env_optional<S: AsRef<str>>(env: S) -> Option<String> {
    let env = env.as_ref();
    match std::env::var(env) {
        Ok(value) => {
            log::debug!("Success getting ENV: [{env}]");
            Some(value)
        }
        Err(_) => None,
    }
}

#[derive(Debug, Default, Clone, dyno_core::serde::Deserialize, dyno_core::serde::Serialize)]
#[serde(crate = "dyno_core::serde")]
pub struct ServerConfig {
    pub secret: Option<Secrets>,
    pub jwt: Jwt,

    pub app_root_path: PathBuf,
    pub app_public_path: PathBuf,
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn init() -> Self {
        let app_root_path = PathBuf::from(get_env("DYNO_APP_ROOT_PATH"));
        let app_public_path = get_env_optional("DYNO_APP_PUBLIC_PATH")
            .map(PathBuf::from)
            .unwrap_or(app_root_path.join("public"));
        let database_url = get_env("DATABASE_URL");

        let host = get_env_optional("DYNO_HOST").unwrap_or("127.0.0.1".to_owned());
        let port = get_env_optional("DYNO_PORT")
            .unwrap_or("8080".to_owned())
            .parse()
            .expect("`DYNO_PORT` ENV should be numerical value");

        Self {
            host,
            port,
            app_root_path,
            app_public_path,
            database_url,
            secret: Secrets::init(),
            jwt: Jwt::init(),
        }
    }
    pub fn tls_path(&self) -> Option<(PathBuf, PathBuf)> {
        self.secret
            .as_ref()
            .map(|scr| (PathBuf::from(&scr.key), PathBuf::from(&scr.cert)))
    }
}

#[derive(Debug, Default, Clone, dyno_core::serde::Deserialize, dyno_core::serde::Serialize)]
#[serde(crate = "dyno_core::serde")]
pub struct Secrets {
    pub cert: String,
    pub key: String,
}

impl Secrets {
    fn init() -> Option<Self> {
        let cert = get_env_optional("DYNO_CERT")?;
        let key = get_env_optional("DYNO_KEY")?;
        Some(Self { cert, key })
    }
}

#[derive(Debug, Default, Clone, dyno_core::serde::Deserialize, dyno_core::serde::Serialize)]
#[serde(crate = "dyno_core::serde")]
pub struct Jwt {
    pub access_token_private_key: String,
    pub access_token_public_key: String,
    pub access_token_expires_in: String,
    pub access_token_max_age: i64,

    pub refresh_token_private_key: String,
    pub refresh_token_public_key: String,
    pub refresh_token_expires_in: String,
    pub refresh_token_max_age: i64,
}

impl Jwt {
    fn init() -> Self {
        let access_token_private_key = get_env("ACCESS_TOKEN_PRIVATE_KEY");
        let access_token_public_key = get_env("ACCESS_TOKEN_PUBLIC_KEY");
        let access_token_expires_in = get_env("ACCESS_TOKEN_EXPIRED_IN");
        let access_token_max_age = get_env("ACCESS_TOKEN_MAXAGE").parse().unwrap_or(120);

        let refresh_token_private_key = get_env("REFRESH_TOKEN_PRIVATE_KEY");
        let refresh_token_public_key = get_env("REFRESH_TOKEN_PUBLIC_KEY");
        let refresh_token_expires_in = get_env("REFRESH_TOKEN_EXPIRED_IN");
        let refresh_token_max_age = get_env("REFRESH_TOKEN_MAXAGE").parse().unwrap_or(240);
        Self {
            access_token_private_key,
            access_token_public_key,
            access_token_expires_in,
            access_token_max_age,
            refresh_token_private_key,
            refresh_token_public_key,
            refresh_token_expires_in,
            refresh_token_max_age,
        }
    }
}
