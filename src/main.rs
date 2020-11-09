#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use clap::Clap;
use rocket::config::{Config, Environment, LoggingLevel};

mod common;
mod repository;
mod route;

/// Example Web API implementation written in Rust.
#[derive(Clap)]
#[clap(version = "0.0.1", author = "Kohei Asai <yo@kohei.dev>")]
struct Opts {
    /// The port the server listens.
    #[clap(long, default_value = "8000")]
    port: u16,

    /// The local address the server listens.
    #[clap(long, default_value = "0.0.0.0")]
    address: String,

    /// The level that server output logs. One of debug, normal, ciritical or off.
    #[clap(long, default_value = "normal")]
    log_level: LoggingLevel,
}

#[get("/")]
fn index() -> &'static str {
    r#"
    # Example API

    * /users - List all users
    * /users/<username> - Get an user
    * /users/<username>/followers - List all users
    "#
}

fn main() {
    let opts: Opts = Opts::parse();
    let config = Config::build(Environment::Staging)
        .address(opts.address)
        .port(opts.port)
        .log_level(opts.log_level)
        .finalize();

    if config.is_ok() {
        rocket::custom(config.unwrap())
            .mount("/", routes![index])
            .mount(
                "/users",
                routes![
                    route::user::index,
                    route::user::get,
                    route::user::follower_index
                ],
            )
            .launch();
    }
}
