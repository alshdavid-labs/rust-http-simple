use std::net::IpAddr;
use std::str::FromStr;

use rocket::config::LogLevel;
use rocket::get;
use rocket::launch;
use rocket::post;
use rocket::routes;
use rocket::Config;

#[get("/")]
fn index_get() -> String {
    return "Hello, world!".to_string();
}

#[post("/", data = "<body>")]
fn index_post(body: String) -> String {
    return body;
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(Config {
            address: IpAddr::from_str("0.0.0.0").unwrap(),
            port: 9000,
            // workers: 4,
            workers: Default::default(),
            log_level: LogLevel::Off,
            ..Config::default()
        })
        .mount("/", routes![index_get, index_post,])
}
