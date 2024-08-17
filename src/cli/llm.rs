use crate::cli::xh_model::{XhResponse, XingHuoModel};
use crate::config::settings::K3;

pub enum Llm{
    Xinghuo(K3,String),
    Tongyi(),
}

impl Llm {
    pub fn exec(&self) ->String{
        match self {
            Llm::Xinghuo(k3, context) => {
                let client = reqwest::blocking::Client::new();

                let url = k3.k3.get_url();
                let http_key = k3.k3.get_http_key();
                let mut auth = "Bearer ".to_string();
                auth.push_str(http_key);
                println!("当前使用大模型：星火");
                let xh = XingHuoModel::new(context.to_string());
                let resp = client.post(url)
                    .header("Content-Type","application/json")
                    .header("Authorization",auth)
                    .body(serde_json::to_string_pretty(&xh).unwrap())
                    .send().map_err(|err| {println!("请求星火大模型失败")});
                match resp.unwrap().text() {
                    Ok(res) => {
                        let resp_obj:serde_json::Result<XhResponse> = serde_json::from_str(res.as_str());
                        if let Ok(reqwest) = resp_obj {
                            if reqwest.is_success() {
                                println!("{}",reqwest.get_msg())
                            }else {
                                println!("fail request")
                            }
                        }else {
                            println!("响应解析失败")
                        }

                    }
                    Err(err) => {}
                }
                "".to_string()
            }
            Llm::Tongyi() => {
                "".to_string()
            }
        }
    }
}