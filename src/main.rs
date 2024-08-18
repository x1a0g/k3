use std::fmt::Debug;
use clap::Parser;
use k3::cli::cli;
use k3::cli::command::{Command, get_check_init, Mode};

fn main() {
    let args = Command::parse();
    match args.mode {
        Mode::Init => {cli::init()},
        Mode::Q {value} => {cli::request_llm(value)},
        Mode::B64 {value}=> {cli::base64_to_str(value)},
        Mode::Ts {value}=> {cli::time_to_str(value)},
    };

}
