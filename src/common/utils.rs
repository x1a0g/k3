use crate::common::k3error::K3error;
use crate::common::k3error::K3error::K3Err;

pub fn get_os_type() -> String {
    let os_info = std::env::consts::OS;
    match os_info {
        "windows" => "win".to_string(),
        "linux" => "linux".to_string(),
        "macos" => "mac".to_string(),
        _ => "unknow".to_string()
    }
}


pub fn get_k3_path()->Result<(String,String),K3error>{
    let dir = dirs_2::home_dir();
    if let Some(dir) = dir{
        let path = dir.join(".k3");
        let yml_path = dir.join(".k3/k3.yaml");
        if let Some(real) =  path.to_str(){
         Ok ((yml_path.to_str().unwrap().to_string(),real.to_string()))
        }else {
            Err(K3error::K3Err("fail read config".to_string()))
        }
    }else {
        Err(K3error::K3Err("fail read config".to_string()))
    }

}