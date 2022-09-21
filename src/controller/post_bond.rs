use std::fs;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use std::process::Command;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::utils;

#[allow(unused, non_snake_case)]
#[skip_serializing_none]
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
    volume_name: String,
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
        let volume_name = config.volName.as_ref().unwrap().clone();
        // mount file
        config.mountPoint = Some(utils::gen_mount_path(&volume_name));
        // bond volume
        config.logDir = Some(utils::gen_log_path(&volume_name));

        // set default value
        if config.logLevel.is_none() {
            config.logLevel = Some("info".to_string());
        }
        if config.owner.is_none() {
            config.owner = Some("cfs".to_string());
        }
        let bond = Bond {
            config,
            volume_name,
        };

        Ok(bond)
    }
    pub fn write_config_file(&self) -> Result<(), String> {
        let config_file = utils::gen_config_file(&self.volume_name);
        utils::parent_mkdirs(Path::new(&config_file))?;

        let content = match serde_json::to_string_pretty(&self.config) {
            Ok(v) => v,
            Err(e) => return Err(format!("error parsing from json to string: {:?}", e)),
        };
        if let Err(e) = fs::write(&config_file, content) {
            return Err(format!("fail: write body to path file fail, {}", e));
        }
        Ok(())
    }

    pub fn write_start_file(&self) -> Result<(), String> {
        let start_file = utils::gen_start_file(&self.volume_name);
        // 父目录创建
        utils::parent_mkdirs(Path::new(&start_file))?;
        // 创建 start.sh 文件
        let content = utils::gen_start_shell_content(&self.volume_name);
        if let Err(e) = fs::write(&start_file, content) {
            return Err(format!("fail: write start.sh fail, {}", e));
        }
        // 添加执行权限 start.sh
        match Command::new("chmod").arg("+x").arg(&start_file).output() {
            Ok(output) => match String::from_utf8(output.stdout) {
                Ok(_v) => Ok(()),
                Err(e) => Err(format!("encode output to utf8 fail, {}", e)),
            },
            Err(e) => Err(format!("chmod +x start.sh fail, {}", e)),
        }
    }
    pub fn startup(&self) -> Result<String, String> {
        // not first start
        if self.ready() {
            return Ok("OK".to_string());
        }
        // write json config file
        if let Err(e) = &self.write_config_file() {
            return Err(format!("write config to file, {}", e));
        }
        // write start shell file
        if let Err(e) = &self.write_start_file() {
            return Err(format!("write config to file, {}", e));
        }
        // create log path
        let log_path = utils::gen_log_path(&self.volume_name);
        if let Err(e) = std::fs::create_dir_all(&log_path) {
            return Err(format!("create log dir[{}] fail, {}", &log_path, e));
        }
        // mount file mkdirs
        let mount_file_path = utils::gen_mount_path(&self.volume_name);
        utils::mkdirs(Path::new(&mount_file_path))?;
        // execute: /cfs/bond/{volume_name}/start.sh
        let start_file = utils::gen_start_file(&self.volume_name);
        match Command::new(&start_file).spawn() {
            Ok(child) => {
                let pid = child.id();
                println!("sub child {}", pid);
                sleep(Duration::from_millis(1500));
                let res = self.ready();
                if res {
                    Ok(format!("OK,{}", pid))
                } else {
                    Ok("child process exit".to_string())
                }
            }
            Err(e) => {
                println!("{}", e);
                Err(format!("exec {} fail, output:{}\n", &start_file, e))
            }
        }
    }

    fn ready(&self) -> bool {
        let shell = utils::get_bond_shell(&self.volume_name);
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

#[post("/bond", data = "<input>")]
pub fn mount(input: Option<String>) -> String {
    if input.is_none() {
        return "fail: body is empty".to_string();
    }

    let config = serde_json::from_str(input.unwrap().as_str());
    if let Err(e) = config {
        return format!("fail: parse body fail: {}", e);
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
