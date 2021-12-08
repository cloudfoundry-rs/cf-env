mod env_constants;
use env_constants::*;
use guid_create::GUID;
use http::Uri;
use std::env;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use locale_types::Locale;
use std::str::FromStr;

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

pub fn get_instance_address() -> Result<SocketAddr, Error> {
    match env::var(CF_INSTANCE_ADDR) {
        Ok(addr_string) => match addr_string.parse::<SocketAddr>() {
            Ok(socket) => Ok(socket),
            Err(_) => Err(Error::EnvMalformed(
                CF_INSTANCE_ADDR.to_string(),
                "Doesn't match the format of addr:ip".to_string(),
            )),
        },
        Err(_) => Err(Error::EnvNotSet(CF_INSTANCE_ADDR.to_string())),
    }
}

pub fn get_instance_guid() -> Result<GUID, Error> {
    match env::var(CF_INSTANCE_GUID) {
        Ok(guid_string) => match GUID::parse(&guid_string) {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::EnvMalformed(
                CF_INSTANCE_GUID.to_string(),
                "Isn't a valid guid".to_string(),
            )),
        },
        Err(_) => Err(Error::EnvNotSet(CF_INSTANCE_GUID.to_string())),
    }
}

pub fn get_instance_index() -> Result<u128, Error> {
    match env::var(CF_INSTANCE_INDEX) {
        Ok(index_string) => match index_string.parse::<u128>() {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::EnvMalformed(
                CF_INSTANCE_INDEX.to_string(),
                "Ins't a valid positive (u128) number".to_string(),
            )),
        },
        Err(_) => Err(Error::EnvNotSet(CF_INSTANCE_INDEX.to_string())),
    }
}

pub fn get_instance_ip() -> Result<IpAddr, Error> {
    match env::var(CF_INSTANCE_IP) {
        Ok(ip_string) => match ip_string.parse::<IpAddr>() {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::EnvMalformed(
                CF_INSTANCE_IP.to_string(),
                "Ins't a valid ip address".to_string(),
            )),
        },
        Err(_) => Err(Error::EnvNotSet(CF_INSTANCE_IP.to_string())),
    }
}

pub fn get_instance_internal_ip() -> Result<IpAddr, Error> {
    match env::var(CF_INSTANCE_INTERNAL_IP) {
        Ok(ip_string) => match ip_string.parse::<IpAddr>() {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::EnvMalformed(
                CF_INSTANCE_INTERNAL_IP.to_string(),
                "Ins't a valid ip address".to_string(),
            )),
        },
        Err(_) => Err(Error::EnvNotSet(CF_INSTANCE_INTERNAL_IP.to_string())),
    }
}

pub fn get_instance_port() -> Result<u16, Error> {
    match env::var(CF_INSTANCE_PORT) {
        Ok(index_string) => match index_string.parse::<u16>() {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::EnvMalformed(
                CF_INSTANCE_PORT.to_string(),
                "Ins't a valid positive (u16) number".to_string(),
            )),
        },
        Err(_) => Err(Error::EnvNotSet(CF_INSTANCE_PORT.to_string())),
    }
}

pub fn get_database_url() -> Result<Uri, Error> {
    match env::var(DATABASE_URL) {
        Ok(index_string) => match index_string.parse::<Uri>() {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::EnvMalformed(
                DATABASE_URL.to_string(),
                "Ins't a valid uri".to_string(),
            )),
        },
        Err(_) => Err(Error::EnvNotSet(DATABASE_URL.to_string())),
    }
}

pub fn get_home() -> Result<PathBuf, Error> {
    match env::var(HOME) {
        Ok(home_string) => Ok(PathBuf::from(home_string)),
        Err(_) => Err(Error::EnvNotSet(HOME.to_string())),
    }
}

pub fn get_lang() -> Result<Locale, Error> {
    match env::var(LANG) {
        Ok(lang_string) => match Locale::from_str(&lang_string) {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::EnvMalformed(
                LANG.to_string(),
                "Ins't a valid locale".to_string(),
            )),
        },
        Err(_) => Err(Error::EnvNotSet(LANG.to_string())),
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

pub struct MemoryLimit {
    pub unit: ByteUnit,
    pub size: u128,
}

impl MemoryLimit {
    pub fn from_string(mut input: String, env_variable: String) -> Result<Self, Error> {
        match ByteUnit::from_string(input.clone()) {
            Ok(unit) => {
                input.pop();
                match input.parse::<u128>() {
                    Ok(size) => Ok(Self { unit, size }),
                    Err(_) => Err(Error::EnvMalformed(
                        env_variable,
                        "Ins't a valid u128".to_string(),
                    )),
                }
            }
            Err(_) => Err(Error::UnkownMemoryUnit),
        }
    }
}

pub fn get_memory_limit() -> Result<MemoryLimit, Error> {
    match env::var(MEMORY_LIMIT) {
        Ok(memory_string) => {
            match MemoryLimit::from_string(memory_string, MEMORY_LIMIT.to_string()) {
                Ok(result) => Ok(result),
                Err(_) => Err(Error::EnvMalformed(
                    MEMORY_LIMIT.to_string(),
                    "Ins't a valid memory size formatted after '<size><unit>'".to_string(),
                )),
            }
        }
        Err(_) => Err(Error::EnvNotSet(MEMORY_LIMIT.to_string())),
    }
}

pub fn get_port() -> Result<u16, Error> {
    match env::var(PORT) {
        Ok(port_string) => match port_string.parse::<u16>() {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::EnvMalformed(
                PORT.to_string(),
                "Ins't a valid positive (u16) number".to_string(),
            )),
        },
        Err(_) => Err(Error::EnvNotSet(PORT.to_string())),
    }
}

pub fn get_pwd() -> Result<PathBuf, Error> {
    match env::var(PWD) {
        Ok(pwd_string) => Ok(PathBuf::from(pwd_string)),
        Err(_) => Err(Error::EnvNotSet(PWD.to_string())),
    }
}

pub fn get_tmp_dir() -> Result<PathBuf, Error> {
    match env::var(TMPDIR) {
        Ok(tmp_dir) => Ok(PathBuf::from(tmp_dir)),
        Err(_) => Err(Error::EnvNotSet(TMPDIR.to_string())),
    }
}

pub fn get_user() -> Result<String, Error> {
    match env::var(USER) {
        Ok(user_string) => Ok(user_string),
        Err(_) => Err(Error::EnvNotSet(USER.to_string())),
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationLimits {
    disk: u128,
    fds: u128,
    mem: u128,
}

#[derive(Serialize, Deserialize)]
pub struct Application {
    application_id: GUID,
    application_name: String,
    application_uris: Vec<String>,
    application_version: GUID,
    cf_api: String,
    limits: ApplicationLimits,
    name: String,
    process_id: String,
    process_type: String,
    organization_id: GUID,
    organization_name: String,
    space_id: GUID,
    space_name: String,
    start: String,
    started_at: Option<String>,
    started_at_timestamp: Option<String>,
    state_timestamp: Option<String>,
    uris: Vec<String>,
    version: GUID,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceVolumeMount {
    container_dir: String,
    device_type: String,
    mode: String,
}

use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Service {
    binding_guid: GUID,
    binding_name: String,
    instance_guid: GUID,
    instance_name: String,
    name: String,
    label: String,
    tags: Vec<String>,
    plan: String,
    credentials: Value,
    syslog_drain_url: String,
    volume_mounts: Vec<ServiceVolumeMount>,
}

use std::collections::HashMap;

pub fn get_services() -> Result<HashMap<String, Vec<Service>>, Error> {

}
pub fn get_service_by_name(name: String) -> Result<Service, Error>{

}
pub fn get_services_by_type(type_name: String) -> Result<Vec<Service>, Error>{}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
