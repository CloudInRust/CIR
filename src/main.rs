// Crates for logging purposes
#[macro_use]
extern crate log;
extern crate log4rs;

// Diesel crates
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate dotenv;
extern crate chrono;

// Web crates
extern crate actix;
extern crate actix_web;

//Misc
#[macro_use]
extern crate failure;
extern crate futures;
#[macro_use]
extern crate serde_derive;
extern crate bytes;

mod build_info;
mod db;
mod web;
mod error;

use web::web_frontend;


fn main() {
    print_boot_info();

    //Create the central Actor System
    let system = actix::System::new("actix-web");

    // Init Db Connection
    let dbsys = db::DBSubsystem::init();

    // Start Angular WebApp
    web_frontend::start_frontend_webapp(&dbsys, 4);

    info!("Server started!");
    system.run();
}

fn print_boot_info() {
    log4rs::init_file("log.yaml", Default::default()).expect("Unable to init log system");
    error!("{} {} booting", build_info::PKG_NAME, build_info::PKG_VERSION);
    debug!("Built on {}, using {} for {}:{}", build_info::BUILT_TIME_UTC, build_info::RUSTC_VERSION, build_info::TARGET, build_info::PROFILE);
    if let Some(git) = build_info::GIT_VERSION {
        debug!("Git version: {}", git);
    }
    if let Some(ci) = build_info::CI_PLATFORM {
        debug!("Built using: {}", ci);
    }
}