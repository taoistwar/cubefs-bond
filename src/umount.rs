use std::process::Command;

#[get("/umount/<volume_name>")]
pub fn umount(volume_name: Option<String>) -> String {
    if volume_name.is_none() {
        return "fail: volume_name missing".to_string();
    }
    let volume_name = volume_name.unwrap();
    if volume_name.is_empty() {
        return "fail: volume_name empty".to_string();
    }
    let mount_path = format!("umount /cfs/mount/{}", volume_name);
    let shell = format!("umount /cfs/mount/{}", volume_name);
    match Command::new("umount").arg(&mount_path).output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(v) => "ok".to_string(),
            Err(e) => format!("fail: parse shell output fail, {}", e).to_string(),
        },
        Err(e) => {
            format!("fail: start[{}] fail, output:{}\n", &shell, e)
        }
    }
}
