#[macro_use]
extern crate rocket;
extern crate kalosm;

use std::str::FromStr;
use std::sync::Arc;
use kalosm::language::{Llama, LlamaSource};
use rocket::{Build, Rocket};
use rocket_cors::{AllowedOrigins, CorsOptions};

#[cfg(any(feature = "part2", feature = "part3"))]
use rocket::tokio::sync::Mutex;

mod helpers;
mod endpoints;

#[cfg(feature = "part1")]
mod part1;
#[cfg(feature = "part2")]
mod part2;
#[cfg(feature = "part3")]
mod part3;

// Load the model initially.
async fn create_model() -> Llama {
    Llama::builder()
        .with_source(LlamaSource::llama_3_2_3b_chat())
        .build().await
        .unwrap()
}

#[rocket::launch]
async fn rocket() -> Rocket<Build> {
    // Create Llama model.
    let model = create_model().await;

    // Setup cors.
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            ["Get", "Post", "Put", "Delete", "Options"]
                .iter()
                .map(|s| FromStr::from_str(s).unwrap())
                .collect(),
        )
        .allow_credentials(true)
        .to_cors()
        .expect("Failed to setup cors configuration.");

    #[cfg(feature = "part2")]
    let state = part2::initialize_state(&model);

    #[cfg(feature = "part3")]
    let state = part3::initialize_state();

    let rocket = rocket::build()
        .configure(rocket::Config::figment().merge(("port", 3000)))
        .mount("/chat", routes![endpoints::chat])
        .mount("/", rocket_cors::catch_all_options_routes())
        .attach(cors.clone())
        .manage(cors)
        .manage(Arc::new(model));

    #[cfg(any(feature = "part2", feature = "part3"))]
    let rocket = rocket.manage(Arc::new(Mutex::new(state)));

    rocket
}