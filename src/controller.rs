use std::path::Path;
use std::{collections::HashMap, fs};

use std::process::Command;

struct Bond {
    config: HashMap<String, String>,
    log_path: String,
    config_file_path: String,
}

impl Bond {
    fn deal_body(input: Option<String>) -> std::result::Result<HashMap<String, String>, String> {
        if input.is_none() {
            return Ok(HashMap::new());
        }
        let input = input.unwrap();
        if input.is_empty() {
            return Ok(HashMap::new());
        }
        let config: HashMap<String, String> = match serde_json::from_str(&input) {
            Ok(config) => config,
            Err(e) => return Err(format!("error parsing from string to json: {:?}", e)),
        };
        if let Err(e) = serde_json::to_string_pretty(&config) {
            return Err(format!("error parsing from json to string: {:?}", e));
        };
        Ok(config)
    }

    fn setup(input: Option<String>) -> Result<Bond, String> {
        let mut config = match Self::deal_body(input) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        let volume_name = match config.get("volName") {
            Some(v) => v.clone(),
            None => return Err("volName missing".to_string()),
        };
        match config.get("masterAddr") {
            Some(v) => v.clone(),
            None => return Err("masterAddr missing".to_string()),
        };

        let mount_file_path = format!("/cfs/mount/{}", volume_name);
        config.insert("mountPoint".to_string(), mount_file_path);
        let log_path = format!("/cfs/client/{}", &volume_name);
        config.insert("logDir".to_string(), log_path.clone());

        let config_file_path = format!("/cfs/client/{}/config.json", volume_name);
        if config.get("logLevel").is_none() {
            config.insert("logLevel".to_string(), "info".to_string());
        }
        if config.get("owner").is_none() {
            config.insert("owner".to_string(), "cfs".to_string());
        }
        let bond = Bond {
            config,
            log_path,
            config_file_path,
        };

        Ok(bond)
    }
    pub fn write_config_file(&self) -> Result<(), String> {
        let path = Path::new(&self.config_file_path);
        if let Some(parent) = path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                return Err(format!(
                    "create config parent dir[{}] fail, {}",
                    parent.to_str().unwrap_or(""),
                    e
                ));
            }
        }
        let content = match serde_json::to_string_pretty(&self.config) {
            Ok(v) => v,
            Err(e) => return Err(format!("error parsing from json to string: {:?}", e)),
        };
        if let Err(e) = fs::write(&self.config_file_path, content) {
            return Err(format!("fail: write body to path file fail, {}", e));
        }
        Ok(())
    }
    pub fn startup(&self) -> Result<String, String> {
        // write json config file
        if let Err(e) = &self.write_config_file() {
            return Err(format!("fail: write config to file, {}", e));
        }
        // create log path
        if let Err(e) = std::fs::create_dir_all(&self.log_path) {
            return Err(format!("create log dir[{}] fail, {}", self.log_path, e));
        }
        let shell = format!(
            "cd {} && nohup /cfs/client/cfs-client -f -c {} &",
            &self.log_path, &self.config_file_path
        );
        match Command::new("/bin/sh").arg("-c").arg(&shell).output() {
            Ok(output) => {
                if !output.stderr.is_empty() {
                    match String::from_utf8(output.stderr) {
                        Ok(v) => return Err(v),
                        Err(e) => return Err(format!("fail: parse shell stderr fail, msg:{}", e)),
                    }
                }

                match String::from_utf8(output.stdout) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(format!("fail: parse shell stdout fail, msg:{}", e)),
                }
            }
            Err(e) => Err(format!("fail: start[{}] fail, output:{}\n", &shell, e)),
        }
    }
}

#[post("/mount", data = "<input>")]
pub fn mount(input: Option<String>) -> String {
    // 1. setup and start
    let bond = match Bond::setup(input) {
        Ok(v) => v,
        Err(e) => return format!("fail: parse param, {}", e),
    };

    // 2. start cfs-client
    match bond.startup() {
        Ok(v) => v,
        Err(e) => format!("fail: start client fail, {}", e),
    }
}

#[cfg(test)]
mod test {
    use super::Bond;

    #[test]
    fn test() {
        let input = Some(
            r#"{
            "volName": "test",
            "owner": "cfs",
            "masterAddr": "10.201.3.28:8868,10.201.3.29:8868,10.201.3.30:8868",
            "profPort": "17510",
            "exporterPort": "9504"
          }"#
            .to_string(),
        );
        // 1. setup and start
        let bond = match Bond::setup(input) {
            Ok(v) => v,
            Err(e) => {
                println!("fail: parse param, {}", e);
                return;
            }
        };
        // 2. start cfs-client
        match bond.startup() {
            Ok(v) => {
                println!("{}", v)
            }
            Err(e) => {
                println!("fail: start client fail, {}", e)
            }
        }
    }
}

#[get("/umount/<volume_name>")]
pub fn umount(volume_name: Option<String>) -> String {
    if volume_name.is_none() {
        return "fail: volume_name missing".to_string();
    }
    let volume_name = volume_name.unwrap();
    if volume_name.is_empty() {
        return "fail: volume_name empty".to_string();
    }

    let shell = format!("umount /cfs/mount/{}", volume_name);
    match Command::new("/bin/sh").arg("-c").arg(&shell).output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(v) => v,
            Err(e) => format!("fail: parse shell output fail, {}", e),
        },
        Err(e) => format!("fail: start[{}] fail, output:{}\n", &shell, e),
    }
}
