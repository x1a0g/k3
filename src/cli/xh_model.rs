use std::collections::HashMap;
use std::iter::Map;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct XingHuoModel {
    pub model: String,
    pub messages: Vec<XhMsg>,
    pub stream: bool,
    pub max_tokens: i64,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct XhMsg {
    pub role: String,
    pub content: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct XhResponse {
    code: i32,
    message: String,
    sid: String,
    choices: Vec<XhChoice>,
    usage: HashMap<String,usize>,
}
impl XhResponse{
    pub fn get_msg(&self)->&String{
        &self.choices[0].message.content
    }

    pub fn is_success(&self)->bool{
        self.code==0
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct XhChoice {
    message: XhMessage,
    index: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XhMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XhUsage {
    total_tokens: usize,
}
impl XingHuoModel {
    pub fn new(val: String) -> Self {
        let mut msgs: Vec<XhMsg> = vec![];
        let msg = XhMsg {
            role: "user".to_string(),
            content: val,
        };
        msgs.push(msg);

        XingHuoModel{
            model:"general".to_string(),
            messages:msgs,
            stream:false,
            max_tokens:512_i64
        }

    }
}