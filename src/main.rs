mod controller;
mod utils;
use controller::delete_bond::umount;
use controller::get_bond::get_bond;
use controller::index::index;
use controller::post_bond::mount;

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
    let figment = Figment::from(rocket::Config::default()).merge(Toml::file("cubefs-bond.toml").nested());
    rocket::custom(figment).mount("/api", routes![index, mount, umount, get_bond])
}
