use std::{path::Path, string::FromUtf8Error};

use tokio::time::error::Error;

use crate::{CFS_BOND_HOME, CFS_CLIENT_FILE, CFS_MOUNT_HOME};

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum ErrorCode {
    E10001_IO_ERROR(std::io::Error),
    E10002_SERDE(String, serde_json::Error),
    /** mkdirs but param is file*/
    E10003_MKDIRS(String),
    E10004_WRITE_FILE(String, std::io::Error),
    E10005(FromUtf8Error),
    E10006_EXEC_PROGRAM(String, std::io::Error),
    E10007_CHMOD_NO_POWER(String),
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::E10001_IO_ERROR(e) => write!(f, "10001:{:?}", e),
            ErrorCode::E10002_SERDE(obj, e) => {
                write!(f, "10002: serde objet[{:?}] fail, {:?}", obj, e)
            }
            ErrorCode::E10003_MKDIRS(file) => {
                write!(f, "10003: mkdirs needs dir, but file[{:?}]", file)
            }
            ErrorCode::E10004_WRITE_FILE(file, e) => {
                write!(f, "10004: write file[{}] fail, {:?}", file, e)
            }
            ErrorCode::E10005(e) => {
                write!(f, "10005: FromUtf8Error, {:?}", e)
            }
            ErrorCode::E10006_EXEC_PROGRAM(program, e) => {
                write!(f, "10006: EXEC PROGRAM[{}] fail, {:?}", program, e)
            }
            ErrorCode::E10007_CHMOD_NO_POWER(program) => {
                write!(f, "10007: CHMOD_NO_POWER {}", program)
            }
        }
    }
}

impl From<std::io::Error> for ErrorCode {
    fn from(e: std::io::Error) -> Self {
        ErrorCode::E10001_IO_ERROR(e)
    }
}

impl From<FromUtf8Error> for ErrorCode {
    fn from(e: FromUtf8Error) -> Self {
        ErrorCode::E10005(e)
    }
}

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
        return Err(ErrorCode::E10003_MKDIRS(
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
