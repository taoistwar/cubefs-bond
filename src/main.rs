mod commons;
mod controller;
mod errors;
mod utils;
use std::path::Path;

use controller::bond::bond_delete_router;
use controller::bond::bond_get_router;
use controller::bond::bond_post_router;
use controller::index_router;

use figment::{
    providers::{Format, Toml},
    Figment,
};

#[macro_use]
extern crate rocket;

const CFS_CLIENT_FILE: &str = "/cfs/client/cfs-client";
const CFS_BOND_HOME: &str = "/cfs/bond";
const CFS_MOUNT_HOME: &str = "/cfs/mount";

#[launch]
fn rocket() -> _ {
    let figment =
        Figment::from(rocket::Config::default()).merge(Toml::file(get_config_file()).nested());
    rocket::custom(figment).mount(
        "/api",
        routes![
            index_router,
            bond_get_router,
            bond_post_router,
            bond_delete_router
        ],
    )
}

fn get_config_file() -> String {
    let files = vec!["./cargo.toml", "./vscode", "./git"];

    let count = files.iter().filter(|file| Path::new(file).exists()).count();
    // 开发模式
    if count == files.len() {
        "build/conf/cubefs-bond.toml".to_string()
    } else {
        "conf/cubefs-bond.toml".to_string()
    }
}
