use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Error {
    EnvNotSet(String),
    EnvMalformed(String, String),
    JsonMalformed(String),
    ServiceNotPresent(String),
    ServiceTypeNotPresent(String),
    UnknownMemoryUnit,
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
            Self::UnknownMemoryUnit => write!(formatter, "memory unit unknown"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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
            _ => Err(Error::UnknownMemoryUnit),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn byte_unit_gigabyte() {
        let unit = crate::ByteUnit::from_string("2G".to_string());

        assert!(unit.is_ok());
        assert_eq!(unit.unwrap(), crate::ByteUnit::Gigabyte);
    }

    #[test]
    fn display_env_not_set() {
        assert_eq!(
            format!("{}", crate::Error::EnvNotSet(crate::USER.to_string())),
            format!("environment variable {:?} is not set", crate::USER)
        );
    }

    #[test]
    fn display_json_mal_formed() {
        assert_eq!(
            format!("{}", crate::Error::JsonMalformed(crate::USER.to_string())),
            format!("the json from {:?} could not be parsed", crate::USER)
        );
    }

    #[test]
    fn display_service_not_present() {
        assert_eq!(
            format!(
                "{}",
                crate::Error::ServiceNotPresent(crate::USER.to_string())
            ),
            format!("service {:?} is not present in VCAP_SERVICES", crate::USER)
        );
    }

    #[test]
    fn display_service_type_not_present() {
        assert_eq!(
            format!(
                "{}",
                crate::Error::ServiceTypeNotPresent(crate::USER.to_string())
            ),
            format!(
                "service type {:?} is not present in VCAP_SERVICES",
                crate::USER
            )
        );
    }

    #[test]
    fn display_memory_unit_unknown() {
        assert_eq!(
            format!("{}", crate::Error::UnknownMemoryUnit),
            "memory unit unknown".to_string()
        );
    }

    #[test]
    fn display_env_mal_formed() {
        assert_eq!(
            format!(
                "{}",
                crate::Error::EnvMalformed(crate::USER.to_string(), "none".to_string())
            ),
            format!(
                "the env variable {:?} does not match the required criterial. \"none\"",
                crate::USER
            )
        );
    }
}
