use std::path::Path;

use crate::{errors::ErrorCode, CFS_BOND_HOME, CFS_CLIENT_FILE, CFS_MOUNT_HOME};

pub fn parent_mkdirs(current_file: &Path) -> Result<(), ErrorCode> {
    if let Some(parent) = current_file.parent() {
        return mkdirs(parent);
    }
    Ok(())
}

pub fn mkdirs(current_file: &Path) -> Result<(), ErrorCode> {
    if current_file.exists() {
        return Ok(());
    }
    if current_file.is_dir() {
        return Err(ErrorCode::E10003_MKDIRS_NEED_DIR(
            current_file.to_str().unwrap_or("").to_string(),
        ));
    }
    std::fs::create_dir_all(current_file)?;
    Ok(())
}

pub fn get_bond_shell(volume_name: &str) -> String {
    let shell = format!(
        "ps aux|grep '/cfs/client/cfs-client -f -c {}'|grep -v grep|wc -l",
        gen_config_file(volume_name)
    );
    shell
}
pub fn gen_config_file(volume_name: &str) -> String {
    let config_file = format!("{}/{}/config.json", CFS_BOND_HOME, volume_name);
    config_file
}
pub fn gen_start_file(volume_name: &str) -> String {
    let config_file = format!("{}/{}/start.sh", CFS_BOND_HOME, volume_name);
    config_file
}
pub fn gen_log_path(volume_name: &str) -> String {
    let config_file = format!("{}/{}/log", CFS_BOND_HOME, volume_name);
    config_file
}
pub fn gen_mount_path(volume_name: &str) -> String {
    let mount_file_path = format!("{}/{}", CFS_MOUNT_HOME, &volume_name);
    mount_file_path
}

pub fn gen_start_shell_content(volume_name: &str) -> String {
    let content = format!(
        r#"#!/usr/bin/env bash
            cd {}
            nohup {} -f -c {} 2>&1 &
        "#,
        volume_name,
        CFS_CLIENT_FILE,
        gen_config_file(volume_name)
    );
    let content = content
        .split('\n')
        .map(|x| x.trim())
        .collect::<Vec<&str>>()
        .join("\n");
    content
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use super::{mkdirs, ErrorCode};

    #[test]
    fn test_mkdirs() -> Result<(), ErrorCode> {
        let path = Path::new("/");
        mkdirs(path)?;
        Ok(())
    }
}
