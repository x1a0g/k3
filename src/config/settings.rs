use std::fs;
use serde::{Deserialize, Serialize};
use crate::common::k3error::K3error;

#[derive(Serialize,Deserialize,Debug)]
pub struct K3{
    pub k3:Setting
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Setting{
    model:String,
    url:String,
    appid:String,
    secret:String,
    apikey:String,
    http_key:String
}

impl Setting {
    pub fn get_model(&self)->&String{
            &self.model
    }

    pub fn get_url(&self)->&String{
        &self.url
    }

    pub fn get_appid(&self)->&String{
        &self.appid
    }

    pub fn get_secret(&self)->&String{
        &self.secret
    }

    pub fn get_apikey(&self)->&String{
        &self.apikey
    }

    pub fn get_http_key(&self)->&String{
        &self.http_key
    }
}
const YAML_TEMP: &str = r#"
k3:
  model: xinghuo
  url: https://spark-api-open.xf-yun.com/v1/chat/completions
  appid: xxx
  secret: xx
  apikey: xxx
  http_key: DnAWftGreLUzYxEPHcSs:WpXcHGziMmVATlwEDbRy

"#;

pub fn read_yaml_to_obj(path:&str)->Result<K3,K3error>{
    let yaml_str = fs::read_to_string(path)?;

    let k3_conf:K3 = serde_yaml::from_str(&yaml_str)?;

    Ok(k3_conf)

}

pub fn write_yaml_to_os(path:&str)->Result<(),K3error> {
    if !fs::exists(path).expect(format!("Can't check existence of file {}",path.to_string()).as_str()){
        if let Err(err) = fs::create_dir_all(path){
            return Err(K3error::Io(err));
        }
    }

    fs::write(path.to_string() +"/k3.yaml", YAML_TEMP.as_bytes()).map_err(|err| K3error::Io(err))
}



