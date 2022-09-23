mod controller;
mod errors;
mod utils;
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
    let figment = Figment::from(rocket::Config::default())
        .merge(Toml::file("conf/cubefs-bond.toml").nested());
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
