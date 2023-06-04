use dyno_core::log;
use std::path::PathBuf;
use std::process::exit;

#[inline]
pub fn get_env_default<S: AsRef<str>>(env: S, def: String) -> String {
    let env = env.as_ref();
    match std::env::var(env) {
        Ok(path) => {
            log::info!("ENV Variable - [{env}:`{path}`]");
            path
        }
        Err(err) => {
            log::error!("Failed to get config paths for ENV:`{env}` - {err} [defaulting to {def}]",);
            def
        }
    }
}

#[inline]
pub fn get_env<S: AsRef<str>>(env: S) -> String {
    let env = env.as_ref();
    match std::env::var(env) {
        Ok(path) => {
            log::info!("ENV Variable - [{env}:`{path}`]");
            path
        }
        Err(err) => {
            log::error!("Failed to get config paths for ENV:`{env}` - {err}");
            exit(1);
        }
    }
}

pub fn get_env_optional<S: AsRef<str>>(env: S) -> Option<String> {
    let env = env.as_ref();
    match std::env::var(env) {
        Ok(path) => {
            log::info!("ENV Variable - [{env}:`{path}`]");
            Some(path)
        }
        Err(_) => None,
    }
}

#[cfg_attr(debug_assert, derive(Debug))]
#[derive(Default, Clone, dyno_core::serde::Deserialize, dyno_core::serde::Serialize)]
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

#[cfg_attr(debug_assert, derive(Debug))]
#[derive(Default, Clone, dyno_core::serde::Deserialize, dyno_core::serde::Serialize)]
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

#[cfg_attr(debug_assert, derive(Debug))]
#[derive(Default, Clone, dyno_core::serde::Deserialize, dyno_core::serde::Serialize)]
#[serde(crate = "dyno_core::serde")]
pub struct Jwt {
    pub secret: String,
    pub expires_in: String,
    pub maxage: i32,
}

impl Jwt {
    fn init() -> Self {
        Self {
            secret: get_env("JWT_SECRET"),
            expires_in: get_env_default("JWT_EXPIRED_IN", "120m".to_owned()),
            maxage: get_env_default("JWT_MAXAGE", "120".to_owned())
                .parse()
                .unwrap(),
        }
    }
}
