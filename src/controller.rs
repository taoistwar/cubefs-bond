use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::{collections::HashMap, fs};

use std::process::Command;

use salvo::prelude::*;

struct Bond {
    config: HashMap<String, String>,
    log_path: String,
    config_file: String,
}

impl Bond {
    fn setup(config: HashMap<String, String>) -> Result<Bond, String> {
        let mut config = config;
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

        let config_file = format!("/cfs/client/{}/config.json", volume_name);
        if config.get("logLevel").is_none() {
            config.insert("logLevel".to_string(), "info".to_string());
        }
        if config.get("owner").is_none() {
            config.insert("owner".to_string(), "cfs".to_string());
        }
        let bond = Bond {
            config,
            log_path,
            config_file,
        };

        Ok(bond)
    }
    pub fn write_config_file(&self) -> Result<(), String> {
        let path = Path::new(&self.config_file);
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
        if let Err(e) = fs::write(&self.config_file, content) {
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
            &self.log_path, &self.config_file
        );
        match Command::new("/bin/sh").arg("-c").arg(&shell).spawn() {
            Ok(child) => {
                let pid = child.id();
                sleep(Duration::from_millis(1500));
                let res = self.pid_exists(pid);
                if res {
                    Ok(format!("{}", pid))
                } else {
                    Ok("fail: child process exit".to_string())
                }
            }
            Err(e) => {
                println!("{}", e);
                Err(format!("fail: start[{}] fail, output:{}\n", &shell, e))
            }
        }
    }

    fn pid_exists(&self, pid: u32) -> bool {
        let shell = format!(
            "ps aux|grep {}|grep '/cfs/client/cfs-client -f -c {}'|wc -l",
            pid, &self.config_file
        );
        match Command::new("/bin/sh").arg("-c").arg(&shell).output() {
            Ok(output) => match String::from_utf8(output.stdout) {
                Ok(v) => {
                    let v = v.trim();
                    v != "1"
                }
                Err(_) => false,
            },
            Err(_) => false,
        }
    }
}

#[handler]
async fn mount(req: &mut Request, res: &mut Response) {
    let config = req.parse_body::<HashMap<String, String>>().await;
    if let Err(e) = config {
        res.render(Text::Plain(format!("parse body fail: {}", e)));
        return;
    }
    let config = config.unwrap();
    // 1. setup and start
    let bond = match Bond::setup(config) {
        Ok(v) => v,
        Err(e) => {
            res.render(Text::Plain(format!("fail: parse param, {}", e)));
            return;
        }
    };

    // 2. start cfs-client
    match bond.startup() {
        Ok(v) => {
            res.render(Text::Plain(v));
        }
        Err(e) => {
            res.render(Text::Plain(format!("fail: start client fail, {}", e)));
        }
    }
}

#[handler]
pub async fn umount(req: &mut Request, res: &mut Response) {
    let volume_name = req.param::<String>("volume_name");
    if volume_name.is_none() {
        res.render(Text::Plain("fail: volume_name missing"));
        return;
    }
    let volume_name = volume_name.unwrap();
    if volume_name.is_empty() {
        res.render(Text::Plain("fail: volume_name empty"));
        return;
    }

    let shell = format!("umount /cfs/mount/{}", volume_name);
    match Command::new("/bin/sh").arg("-c").arg(&shell).output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(v) => {
                res.render(Text::Plain(v));
            }
            Err(e) => {
                res.render(Text::Plain(format!("fail: parse shell output fail, {}", e)));
            }
        },
        Err(e) => {
            res.render(Text::Plain(format!(
                "fail: start[{}] fail, output:{}\n",
                &shell, e
            )));
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::Bond;

    #[test]
    fn test() {
        let input = r#"{
            "volName": "test",
            "owner": "cfs",
            "masterAddr": "10.201.3.28:8868,10.201.3.29:8868,10.201.3.30:8868",
            "profPort": "17510",
            "exporterPort": "9504"
          }"#;
        // 1. setup and start
        let config: HashMap<String, String> = serde_json::from_str(input).unwrap();
        let bond = match Bond::setup(config) {
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
    #[test]
    fn test1() {
        let s1 = format!("x{}", "xx");
        let x = s1.as_str() == "xxx";
        println!("{}", x);
    }
}
