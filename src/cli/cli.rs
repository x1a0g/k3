use base64::prelude::*;
use chrono::{TimeZone, Utc};
use crate::cli::command::get_check_init;
use crate::cli::llm::Llm;
use crate::common::k3error::K3error;
use crate::common::utils::get_k3_path;
use crate::config::settings::{read_yaml_to_obj, write_yaml_to_os};

pub fn init() -> () {
    let path = get_k3_path();
    match path {
        Ok((dir,path)) => {
            let res = write_yaml_to_os(path.as_str());
            if let Err(r) = res {
                println!("写入失败:{:?}",r)
            }else {
                println!("配置文件路径为:[{}]",dir);
                println!("1.请打开文件修改对应的大模型相关数据");
            }
        }
        Err(_) => {}
    }

}


pub fn base64_to_str(value: String) -> () {
    let b64 = BASE64_STANDARD.decode(value).map_err(|err| { println!("input param is not a base64 str") });

    if let Ok(res) = b64 {
        let r = String::from_utf8(res);
        match r {
            Ok(re) => { println!("{:?}", re) }
            Err(err) => { println!("input param is not a base64 str") }
        }
    }
}

pub fn time_to_str(value: String) -> () {
    let timest = value.parse::<i64>().map_err(|err| println!("The input is not a timestamp"));
    if let Ok(time) = timest {
        if time < 999_999_9999 {
            let res = Utc.timestamp_opt(time, 0);
            let datetime = res.single().expect("Failed to convert date and time");
            println!("{}", datetime.format("%Y-%m-%d %H:%M:%S"));
        } else {
            let res = Utc.timestamp_millis_opt(time);
            let datetime = res.single().expect("Failed to convert date and time");
            println!("{}", datetime.format("%Y-%m-%d %H:%M:%S"));
        }
    }
}


pub fn request_llm(value: String) -> () {
    if let Ok(check) = get_check_init() {
        if check {
            match get_k3_path() {
                Ok((dir,path)) => {
                    let k3 = read_yaml_to_obj(dir.as_str());
                    if let Ok(k33) = k3 {
                        let fc = Llm::Xinghuo(k33,value);
                        fc.exec();
                    }
                }
                Err(_) => {
                    println!("fail req llm")
                }
            }
        } else {
            println!("please exec `k3 init` first")
        }
    } else {
        println!("System error")
    }
}