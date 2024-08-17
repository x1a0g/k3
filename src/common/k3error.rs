use std::io;

#[derive(Debug)]
pub enum K3error{
    Io(io::Error),
    SerdeYamlError(serde_yaml::Error),
    K3Err(String)
}

impl From<io::Error> for K3error{
    fn from(value:io::Error)->Self{
        K3error::Io(value)
    }
}

impl From<serde_yaml::Error> for K3error{
    fn from(value: serde_yaml::Error) -> Self {
        K3error::SerdeYamlError(value)
    }
}