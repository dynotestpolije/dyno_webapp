mod actions;
mod config;
mod handler;
mod middlewares;
mod models;
mod schema;
mod seeder;

use std::sync::{Arc, Mutex};

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{self, guard, http::header, middleware::Logger, web, App, HttpServer};
use dyno_core::{log, DynoConfig, DynoErr, DynoResult, UserSession};

// TODO: should i implement other databases?
#[cfg(feature = "db_sqlite")]
mod db {
    pub type DynoDBConn = diesel::SqliteConnection;
    pub type DynoDBConnManager = diesel::r2d2::ConnectionManager<DynoDBConn>;
    pub type DynoDBPool = diesel::r2d2::Pool<DynoDBConnManager>;
    pub type DynoDBPooledConnection = diesel::r2d2::PooledConnection<DynoDBConnManager>;
}
use db::*;
use models::ActiveUser;

async fn index() -> actix_files::NamedFile {
    actix_files::NamedFile::open("public/root/index.html").unwrap()
}

#[actix_web::main]
async fn main() -> DynoResult<()> {
    init_logger()?;

    let app_state = server_init()?;
    log::debug!("Server State is initiaize.. configuring Server..");

    let tls_acceptor = match app_state.cfg.tls_path() {
        Some((key, cert)) => Some(load_tls(key, cert)?),
        None => None,
    };

    let host = app_state.cfg.host.clone();
    let port = app_state.cfg.port;

    log::info!(
        "starting HTTP server at {}://{host}:{port}",
        if tls_acceptor.is_none() {
            "http"
        } else {
            "https"
        },
    );
    let root_path = get_and_check_path(&app_state.cfg.app_public_path, "root/");

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(
                Cors::default() // allowed_origin return access-control-allow-origin: * by default
                    .allow_any_origin()
                    .allowed_methods(["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
                    .max_age(3600),
            )
            .configure(handler::api)
            .service(
                Files::new("/", root_path.clone())
                    .index_file("index.html")
                    .guard(guard::Get()),
            )
            .default_service(web::get().to(index))
            .wrap(Logger::default())
    });
    if let Some(tls) = tls_acceptor {
        http_server
            .bind_openssl((host.as_str(), port), tls)?
            .run()
            .await
            .map_err(From::from)
    } else {
        http_server
            .bind((host.as_str(), port))?
            .run()
            .await
            .map_err(From::from)
    }
}

#[derive(Debug, Clone)]
pub struct ServerState {
    pub db: DynoDBPool,
    pub cfg: config::ServerConfig,

    pub active: Arc<Mutex<Option<ActiveUser>>>,
}
impl ServerState {
    pub fn new() -> DynoResult<Self> {
        server_init()
    }

    pub fn change_active_user(&self, user: UserSession) {
        let Ok(mut active_lock) = self.active.lock() else { return; };
        if let Some(active) = active_lock.clone() {
            *active_lock = Some(active.set_user(user));
        } else {
            *active_lock = Some(ActiveUser::new().set_user(user));
        }
    }
    pub fn change_active_dyno(&self, dyno: DynoConfig) {
        let Ok(mut active_lock) = self.active.lock() else { return; };
        if let Some(active) = active_lock.clone() {
            *active_lock = Some(active.set_dyno(dyno));
        } else {
            *active_lock = Some(ActiveUser::new().set_dyno(dyno));
        }
    }
    pub fn set_active(&self, other: Option<ActiveUser>) {
        let Ok(mut active) = self.active.lock() else { return; };
        *active = other;
    }

    pub fn get_active(&self) -> Option<ActiveUser> {
        let Ok(active) = self.active.lock() else { return None; };
        active.clone()
    }
}

fn server_init() -> DynoResult<ServerState> {
    let cfg = config::ServerConfig::init();
    let manager = DynoDBConnManager::new(&cfg.database_url);

    match diesel::r2d2::Pool::builder().build(manager) {
        Ok(db) => {
            log::info!("✅ Connection to the database is successful!");
            match db
                .get()
                .map_err(DynoErr::database_error)
                .and_then(seeder::seeds)
            {
                Ok(()) => log::info!("✅ Seeding database is successful!"),
                Err(err) => log::error!("❌ Failed to Seeding the database: {err}!"),
            }
            Ok(ServerState {
                db,
                cfg,
                active: Default::default(),
            })
        }
        Err(err) => Err(DynoErr::database_error(format!(
            "❌ Failed to connect to the database: {} - ({})",
            cfg.database_url, err
        ))),
    }
}

fn init_logger() -> DynoResult<()> {
    match dotenv::dotenv().map_err(DynoErr::any_error) {
        Ok(_) => (),
        Err(err) => dyno_core::log::error!("{err}"),
    }

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    Ok(())
}

fn get_and_check_path(
    root: impl AsRef<std::path::Path>,
    folder: impl AsRef<str>,
) -> std::path::PathBuf {
    let folder = folder.as_ref();
    let out = root.as_ref().join(folder);
    if !out.exists() {
        if let Err(err) = std::fs::create_dir_all(&out) {
            log::error!("Failed to create folder in path {} - {err}", out.display())
        }
    }
    log::info!("get {folder} path - {}", out.display());
    out
}

fn load_tls(
    key_path: impl AsRef<std::path::Path>,
    cert_path: impl AsRef<std::path::Path>,
) -> DynoResult<openssl::ssl::SslAcceptorBuilder> {
    use openssl::*;
    let key_path = key_path.as_ref();
    let cert_path = cert_path.as_ref();

    let key = {
        let mut file = std::fs::File::open(key_path)?;
        let mut buffer = Vec::new();
        std::io::Read::read_to_end(&mut file, &mut buffer)?;
        pkey::PKey::private_key_from_pem_passphrase(&buffer, b"password")
            .map_err(DynoErr::any_error)?
    };

    let mut builder = ssl::SslAcceptor::mozilla_intermediate(ssl::SslMethod::tls())
        .map_err(DynoErr::any_error)?;
    builder.set_private_key(&key).map_err(DynoErr::any_error)?;
    builder
        .set_certificate_chain_file(cert_path)
        .map_err(DynoErr::any_error)?;

    Ok(builder)
}
