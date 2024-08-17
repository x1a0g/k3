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