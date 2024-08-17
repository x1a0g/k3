use std::fs;
use clap::{Parser, Subcommand, ValueEnum};
use crate::common::k3error::K3error;

#[derive(Parser)]
#[command(author = "x1a0g", version = "0.0.1", about, long_about = None)]
pub struct Command {
    #[command(subcommand)]
    pub mode: Mode,

}

#[derive(Subcommand,Debug)]
pub enum Mode{
    /// Init K3
    Init,
    /// Request LLM
    Q{
        value:String
    },
    /// timestemp to yyyy-MM-dd HH:mm:ss
    Ts{
        value:String
    },
    /// base 64 to str
    B64{
        value:String
    },
}



pub fn get_check_init() -> Result<bool, K3error> {
    if let Some(path) = dirs_2::home_dir() {
        let cur_path = path.join(".k3/k3.yaml");
        if let Some(cur_path_res) = cur_path.to_str(){
            return Ok(fs::metadata(cur_path_res).is_ok());
        }
    } else {
        return Err(K3error::K3Err("fail read current_path".to_string()));
    }

    Ok(false)
}