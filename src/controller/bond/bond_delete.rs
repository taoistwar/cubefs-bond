use std::process::Command;

#[delete("/bond/<volume_name>")]
pub fn bond_delete_router(volume_name: Option<String>) -> String {
    if volume_name.is_none() {
        return "fail: volume_name missing".to_string();
    }
    let volume_name = volume_name.unwrap();
    if volume_name.is_empty() {
        return "fail: volume_name empty".to_string();
    }
    let mount_path = format!("umount /cfs/mount/{}", volume_name);

    let shell = format!(
        "ps aux|grep cfs-client |grep {}|cut -d ' ' -f 2|xargs kill -9 && umount {}",
        volume_name, mount_path
    );
    match Command::new("sh").arg("-c").arg(&mount_path).output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(_v) => "OK".to_string(),
            Err(e) => format!("fail: parse shell output fail, {}", e),
        },
        Err(e) => {
            format!("fail: start[{}] fail, output:{}\n", &shell, e)
        }
    }
}
