use std::process::Command;

use crate::utils;

#[get("/bond/<volume_name>")]
pub fn bond_get_router(volume_name: Option<String>) -> String {
    if volume_name.is_none() {
        return "fail: volume_name missing".to_string();
    }
    let volume_name = volume_name.unwrap();
    if volume_name.is_empty() {
        return "fail: volume_name empty".to_string();
    }
    let shell = utils::get_bond_shell(&volume_name);
    match Command::new("sh").arg("-c").arg(&shell).output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(v) => {
                let v = v.trim();
                if v == "0" {
                    "cfs-client not start".to_string()
                } else {
                    "OK".to_string()
                }
            }
            Err(e) => format!("fail: parse shell output fail, {}", e),
        },
        Err(e) => {
            format!("fail: start[{}] fail, output:{}\n", &shell, e)
        }
    }
}
