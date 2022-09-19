use std::fs;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use std::process::Command;

use serde::{Deserialize, Serialize};

#[allow(unused, non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default)]
struct ClientConfig {
    mountPoint: Option<String>, // 挂载点，是

    volName: Option<String>, // 卷名称，是

    owner: Option<String>, // 所有者，是

    masterAddr: String, // Master节点地址，是

    logDir: Option<String>, // 日志存放路径，否

    logLevel: Option<String>, // 日志级别：debug, info, warn, error， 否

    profPort: Option<String>, // golang pprof调试端口，否

    exporterPort: Option<String>, // prometheus获取监控数据端口，否

    consulAddr: Option<String>, // 监控注册服务器地址，否

    lookupValid: Option<String>, // 内核FUSE lookup有效期，单位：秒，否

    attrValid: Option<String>, // 内核FUSE attribute有效期，单位：秒，否

    icacheTimeout: Option<String>, //客户端inode cache有效期，单位：秒；否

    enSyncWrite: Option<String>, // 使能DirectIO同步写，即DirectIO强制数据节点落盘；否

    autoInvalData: Option<String>, // FUSE挂载使用AutoInvalData选项；否

    ronly: Option<bool>, // 以只读方式挂载，默认为false，否

    writecache: Option<bool>, // 利用内核FUSE的写缓存功能，需要内核FUSE模块支持写缓存，默认为false；否

    keepcache: Option<bool>, // 保留内核页面缓存。此功能需要启用writecache选项，默认为false；否

    token: Option<String>, // 如果创建卷时开启了enableToken，此参数填写对应权限的token；否

    readRate: Option<u32>, // 限制每秒读取次数，默认无限制；否

    writeRate: Option<u32>, // 限制每秒写入次数，默认无限制；否

    followerRead: Option<bool>, // 从follower中读取数据，默认为false；否

    accessKey: Option<String>, //卷所属用户的鉴权密钥；否

    secretKey: Option<String>, // 卷所属用户的鉴权密钥；否

    disableDcache: Option<bool>, // 禁用Dentry缓存，默认为false；否

    subdir: Option<String>, // 设置子目录挂载；否

    fsyncOnClose: Option<bool>, // 文件关闭后执行fsync操作，默认为true；否

    maxcpus: Option<u32>, // 最大可使用的cpu核数，可限制client进程cpu使用率；否

    enableXattr: Option<bool>, // 是否使用*xattr*，默认是false；否

    enableBcache: Option<bool>, // 是否开启本地一级缓存，默认false
}

struct Bond {
    config: ClientConfig,
    log_path: String,
    config_file: String,
    mount_file_path: String,
}

impl Bond {
    fn setup(config: ClientConfig) -> Result<Bond, String> {
        let mut config = config;

        if config.volName.is_none() {
            return Err("volName missing".to_string());
        }
        if config.masterAddr.is_empty() {
            return Err("masterAddr missing".to_string());
        }
        let volume_name = &config.volName.as_ref().unwrap();

        let mount_file_path = format!("/cfs/mount/{}", volume_name);
        config.mountPoint = Some(mount_file_path.clone());
        let log_path = format!("/cfs/client/{}", &volume_name);
        config.logDir = Some(log_path.clone());

        let config_file = format!("/cfs/client/{}/config.json", volume_name);
        if config.logLevel.is_none() {
            config.logLevel = Some("info".to_string());
        }
        if config.owner.is_none() {
            config.owner = Some("cfs".to_string());
        }
        let bond = Bond {
            config,
            log_path,
            config_file,
            mount_file_path,
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
        if let Err(e) = std::fs::create_dir_all(&self.mount_file_path) {
            return Err(format!(
                "create mount dir[{}] fail, {}",
                &self.mount_file_path, e
            ));
        }
        let shell = format!("/cfs/client/cfs-client -f -c {}", &self.config_file);
        match Command::new("/cfs/client/cfs-client")
            .arg("-f")
            .arg("-c")
            .arg(&self.config_file)
            .spawn()
        {
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
                    v != "0"
                }
                Err(_) => false,
            },
            Err(_) => false,
        }
    }
}

#[post("/mount", data = "<input>")]
pub fn mount(input: Option<String>) -> String {
    if input.is_none() {
        return "body is empty".to_string();
    }

    let config = serde_json::from_str(input.unwrap().as_str());
    if let Err(e) = config {
        return format!("parse body fail: {}", e);
    }
    let config = config.unwrap();
    // 1. setup and start
    let bond = match Bond::setup(config) {
        Ok(v) => v,
        Err(e) => {
            return format!("fail: parse param, {}", e);
        }
    };

    // 2. start cfs-client
    match bond.startup() {
        Ok(v) => v,
        Err(e) => {
            format!("fail: start client fail, {}", e)
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Bond, ClientConfig};

    #[test]
    fn test_serde() {
        let input = r#"{
            "volName": "test",
            "owner": "cfs",
            "masterAddr": "10.201.3.28:8868,10.201.3.29:8868,10.201.3.30:8868",
            "profPort": "17510",
            "exporterPort": "9504"
          }"#;
        let x: ClientConfig = serde_json::from_str(input).unwrap();
        println!("{:?}", x)
    }
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
        let config: ClientConfig = serde_json::from_str(input).unwrap();
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
}
