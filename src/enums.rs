use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Error {
    EnvNotSet(String),
    EnvMalformed(String, String),
    JsonMalformed(String),
    ServiceNotPresent(String),
    ServiceTypeNotPresent(String),
    UnkownMemoryUnit,
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match &*self {
            Self::EnvNotSet(variable_name) => write!(
                formatter,
                "environment variable {:?} is not set",
                variable_name
            ),
            Self::ServiceNotPresent(service_name) => write!(
                formatter,
                "service {:?} is not present in VCAP_SERVICES",
                service_name
            ),
            Self::ServiceTypeNotPresent(service_type_name) => write!(
                formatter,
                "service type {:?} is not present in VCAP_SERVICES",
                service_type_name
            ),
            Self::JsonMalformed(variable_to_parse_name) => write!(
                formatter,
                "the json from {:?} could not be parsed",
                variable_to_parse_name
            ),
            Self::EnvMalformed(variable_name, comment) => write!(
                formatter,
                "the env variable {:?} does not match the required criterial. {:?}",
                variable_name, comment
            ),
            Self::UnkownMemoryUnit => write!(formatter, "memory unit unkown"),
        }
    }
}

pub enum ByteUnit {
    Gigabyte,
    Megabyte,
}

impl ByteUnit {
    pub fn from_string(input: String) -> Result<Self, Error> {
        let last_char = input.chars().rev().next().unwrap();

        match last_char {
            'M' | 'm' => Ok(Self::Megabyte),
            'G' | 'g' => Ok(Self::Gigabyte),
            _ => Err(Error::UnkownMemoryUnit),
        }
    }
}
