mod controller;

use salvo::prelude::*;

#[handler]
fn index() -> &'static str {
    r#"
        API  URI: /bond
             BODY: {}

    body is json, and for cubefs client config.json content
"#
}
#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() {
    use std::{fs, path::Path};
    let path = "/cfs/bond/salvo.sock";
    let sock_path = Path::new(path);
    if sock_path.exists() {
        fs::remove_file(sock_path).expect("unix socket file exist and can't remove");
    }

    let router = Router::new()
        .get(index)
        .push(Router::with_path("/mount/<volume_name>").delete(controller::mount))
        .push(Router::with_path("/bond/<volume_name>").delete(controller::mount))
        .push(Router::with_path("/umount/<volume_name>").delete(controller::umount));

    Server::new(UnixListener::bind(path)).serve(router).await;
}

#[cfg(not(target_os = "linux"))]
#[tokio::main]
async fn main() {
    println!("please run cubefs-bond in linux");
}
