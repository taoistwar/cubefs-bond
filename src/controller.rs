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
        let input = input.unwrap();
        println!("{}", input);
        let config: HashMap<String, String> = match serde_json::from_str(&input) {
            Ok(text) => text,
            Err(e) => return Err(format!("error parsing from string to json: {:?}", e)),
        };
        if let Err(e) = serde_json::to_string_pretty(&config) {
            return Err(format!("error parsing from json to string: {:?}", e));
        };
        Ok(config)
    }
    fn deal_item(
        param: &Option<String>,
        config: &mut HashMap<String, String>,
        key: &str,
    ) -> Result<String, String> {
        match param {
            Some(v) => {
                config.insert(key.to_string(), v.clone());
                Ok(v.clone())
            }
            None => match config.get(key) {
                Some(v) => Ok(v.clone()),
                None => Err(format!("{} missing", key)),
            },
        }
    }
    fn setup(
        volume_name: Option<String>,
        master_address: Option<String>,
        exporter_port: Option<String>,
        prof_port: Option<String>,
        owner: Option<String>,
        input: Option<String>,
    ) -> Result<Bond, String> {
        let mut config: HashMap<String, String> = if input.is_none() {
            HashMap::new()
        } else {
            match Self::deal_body(input) {
                Ok(res) => res,
                Err(e) => return Err(format!("fail: input param missing, {}", e)),
            }
        };

        let volume_name = match Self::deal_item(&volume_name, &mut config, "volName") {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        match Self::deal_item(&master_address, &mut config, "masterAddr") {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        match Self::deal_item(&exporter_port, &mut config, "exporterPort") {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        match Self::deal_item(&prof_port, &mut config, "profPort") {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        match Self::deal_item(&owner, &mut config, "owner") {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let mount_file_path = format!("/cfs/mount/{}", volume_name);
        config.insert("mountPoint".to_string(), mount_file_path);
        let log_path = format!("/cfs/client/{}", &volume_name);
        config.insert("logDir".to_string(), log_path.clone());
        let config_file_path = format!("/cfs/client/{}", volume_name);
        config.insert("logLevel".to_string(), "info".to_string());
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
        // 2. write json config file
        if let Err(e) = &self.write_config_file() {
            return Err(format!("fail: write config to file, {}", e));
        }
        let shell = format!(
            "cd /cfs/client/{} && nohup /cfs/client/cfs-client -f -c {} &",
            &self.log_path, &self.config_file_path
        );
        match Command::new("/bin/sh").arg("-c").arg(&shell).output() {
            Ok(output) => match String::from_utf8(output.stdout) {
                Ok(v) => Ok(v),
                Err(e) => Err(format!("fail: parse shell output fail, {}", e)),
            },
            Err(e) => Err(format!("fail: start[{}] fail, output:{}\n", &shell, e)),
        }
    }
}

#[post(
    "/bond?<volName>&<masterAddr>&<exporterPort>&<profPort>&<owner>",
    data = "<input>"
)]
pub fn bond(
    volName: Option<String>,
    masterAddr: Option<String>,
    exporterPort: Option<String>,
    profPort: Option<String>,
    owner: Option<String>,
    input: Option<String>,
) -> String {
    // 1. setup and start
    let bond = match Bond::setup(volName, masterAddr, exporterPort, profPort, owner, input) {
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
        let volume_name = Some("test".to_string());
        let master_address = Some("x".to_string());
        let exporter_port = Some("9503".to_string());
        let prof_port = Some("17511".to_string());
        let input = Some(
            r#"{
            "mountPoint": "/cfs/mountpoint",
            "volName": "ltptest",
            "owner": "ltptest",
            "masterAddr": "10.196.59.198:17010,10.196.59.199:17010,10.196.59.200:17010",
            "logDir": "/cfs/client/log",
            "profPort": "17510",
            "exporterPort": "9504",
            "logLevel": "info"
          }"#
            .to_string(),
        );
        let owner = Some("cfs".to_string());
        // 1. setup and start
        let bond = match Bond::setup(
            volume_name,
            master_address,
            exporter_port,
            prof_port,
            owner,
            input,
        ) {
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
