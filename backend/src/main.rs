mod actions;
mod config;
mod handler;
mod middlewares;
mod models;
mod schema;

use actix_files::Files;
use actix_web::{self, guard, middleware::Logger, web, App, HttpServer};
use dyno_core::{log, DynoErr, DynoResult};

use std::sync::Arc;

pub type DynoDB = diesel::sqlite::Sqlite;
pub type DynoDBConn = diesel::SqliteConnection;
pub type DynoDBConnManager = diesel::r2d2::ConnectionManager<DynoDBConn>;
pub type DynoDBPool = diesel::r2d2::Pool<DynoDBConnManager>;
pub type DynoDBPooledConnection = diesel::r2d2::PooledConnection<DynoDBConnManager>;

#[actix_web::main]
async fn main() -> DynoResult<()> {
    init_logger()?;

    let (dbpool, config) = server_init()?;
    log::debug!("Server State is initiaize.. configuring Server..");

    let tls_acceptor = match config.tls_path() {
        Some((key, cert)) => Some(load_tls(key, cert)?),
        None => None,
    };

    let host = config.host.clone();
    let port = config.port;

    log::info!(
        "starting HTTP server at {}://{host}:{port}",
        if tls_acceptor.is_none() {
            "http"
        } else {
            "https"
        },
    );
    let root_path = get_and_check_path(&config.app_public_path, "root");
    let img_path = get_and_check_path(&config.app_public_path, "img");
    let dyno_path = get_and_check_path(&config.app_public_path, "dyno");

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(dbpool.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(web::redirect("/", "/index.html"))
            .service(Files::new("/", root_path.clone()).index_file("index.html"))
            .service(
                Files::new("/static", img_path.clone())
                    .method_guard(guard::Get())
                    .show_files_listing(),
            )
            .service(
                Files::new("/dyno", dyno_path.clone())
                    .method_guard(guard::Get())
                    .show_files_listing(),
            )
            .service(handler::api())
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

fn server_init() -> DynoResult<(DynoDBPool, Arc<config::ServerConfig>)> {
    let cfg = Arc::new(config::ServerConfig::init());

    let manager = DynoDBConnManager::new(&cfg.database_url);

    match diesel::r2d2::Pool::builder().build(manager) {
        Ok(db) => {
            log::info!("✅ Connection to the database is successful!");
            Ok((db, cfg))
        }
        Err(err) => Err(DynoErr::database_error(format!(
            "Failed to connect to the database: {} - ({})",
            cfg.database_url, err
        ))),
    }
}

fn init_logger() -> DynoResult<()> {
    match dotenv::dotenv().map_err(DynoErr::any_error) {
        Ok(_) => (),
        Err(err) => dyno_core::log::error!("{err}"),
    }

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
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
