use api_id_assigner_lib::AppState;
use api_id_assigner_lib::router;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use axum::Router;
use std::{net::SocketAddr, str::FromStr,fs, path::Path};
use tower_http::cors::{Any, CorsLayer};
use serde::Deserialize;



#[derive(Deserialize)]
struct Config {
   ip: String,
   port: u16,
   db_location: String
}


#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{




    if !cfg!(debug_assertions){
        //check if running in production or debug env.
        if !std::path::Path::new("/var/assigner").exists() {
            println!("in release creating folder");
            match fs::create_dir_all("/var/assigner") {
                Ok(_res) => println!("Directory created succesfully"),
                Err(err) => panic!("cannot create dir {err}")
            }

        } 
        //check if config file exists, if not, write default values in it
        if !std::path::Path::new("/var/assigner/config.toml").exists() {
            println!("config file doesnt exist");
            let _ = match fs::write(Path::new("/var/assigner/config.toml"), b"ip = '0.0.0.0'\nport = 2000\ndb_location = '/var/assigner/db.sqlite'"){
                Ok(res) => res,
                Err(err) => panic!("error: {err}")
            };
        }
    }

    // read the toml config file into a string, if file not found give default values
    let config_string = match fs::read_to_string("/var/assigner/config.toml") {
        Ok(res) => res,
        Err(_) => "
            ip = '0.0.0.0'
            port = 2000
            db_location = '/var/assigner/db.sqlite'".to_string()

    };
    let config:Config = match toml::from_str(&config_string) {
        Ok(res) => res,
        Err(_) => Config { ip: "0.0.0.0".to_string(), port: 2000, db_location: "db.sqlite".to_string() }
    };
    // there is a better way but for now there isnt, if in development will be using a db.sqlite
    // file in the cargo run location, in release will be using a db located at the path specified.


    println!("Using db: {}",config.db_location);
    let options = SqliteConnectOptions::new()
    .filename(config.db_location)
    .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    sqlx::migrate!().run(&pool).await.expect("Migrations failed :(");

    let domain:String = format!("{}:{}",config.ip,config.port);
    let state = AppState { domain,pool};
    let cors = CorsLayer::new().allow_origin(Any);

    let api_router = router::create_api_router(state.clone());

    let app = Router::new()
        .nest("/api", api_router)
        .layer(cors);


    //let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let addr = SocketAddr::from_str(&state.domain).unwrap();
    println!("listening on {}", addr.to_string());

    axum::Server::bind(&addr)
        .serve(
            //app.into_make_service()
            app.into_make_service_with_connect_info::<SocketAddr>()
            )
        .await
        .unwrap();

    Ok(())
}
