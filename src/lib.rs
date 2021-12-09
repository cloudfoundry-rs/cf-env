//! Easy and typed abstraction on Cloud Foundry Environment variables
//!
//! It allows you to get:
//! - Services, optionally filtered by name or type
//! - Application infos
//! - Cloud Foundry app instance set variables, most of the times starting with `CF_`
//!
//! It's meant to get you away from boilerplating and get you started with typed variables while keeping simplicity

pub mod constants;
pub mod enums;
pub mod models;
pub use constants::*;
pub use enums::*;
use guid_create::GUID;
use http::Uri;
use locale_types::Locale;
pub use models::*;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;

/// Get's the value from `CF_INSTANCE_ADDR` as a typed SocketAddr
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

/// Get's the value from `CF_INSTANCE_GUID` as a typed GUID
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

/// Get's the value from `CF_INSTANCE_INDEX` as a typed u128
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

/// Get's the value from `CF_INSTANCE_IP` as a typed IpAddr
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

/// Get's the value from `CF_INSTANCE_INTERNAL_IP` as a typed IpAddr
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

/// Get's the value from `CF_INSTANCE_PORT` as a typed u16
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

/// Get's the value from `DATABASE_URL` as a typed Uri
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

/// Get's the value from `HOME` as a typed PathBuf
pub fn get_home() -> Result<PathBuf, Error> {
    match env::var(HOME) {
        Ok(home_string) => Ok(PathBuf::from(home_string)),
        Err(_) => Err(Error::EnvNotSet(HOME.to_string())),
    }
}

/// Get's the value from `LANG` as a typed Locale
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

/// Get's the value from `MEMORY_LIMIT` as a typed MemoryLimit
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

/// Get's the value from `PORT` as a typed 16
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

/// Get's the value from `PWD` as a typed PathBuf
pub fn get_pwd() -> Result<PathBuf, Error> {
    match env::var(PWD) {
        Ok(pwd_string) => Ok(PathBuf::from(pwd_string)),
        Err(_) => Err(Error::EnvNotSet(PWD.to_string())),
    }
}

/// Get's the value from `TMPDIR` as a typed PathBuf
pub fn get_tmp_dir() -> Result<PathBuf, Error> {
    match env::var(TMPDIR) {
        Ok(tmp_dir) => Ok(PathBuf::from(tmp_dir)),
        Err(_) => Err(Error::EnvNotSet(TMPDIR.to_string())),
    }
}

/// Get's the value from `USER`
pub fn get_user() -> Result<String, Error> {
    match env::var(USER) {
        Ok(user_string) => Ok(user_string),
        Err(_) => Err(Error::EnvNotSet(USER.to_string())),
    }
}

type ServiceMap = HashMap<String, Vec<Service>>;

/// Get's the value from `VCAP_SERVICES` as a typed HashMap of Strings and a list of Services
pub fn get_services() -> Result<ServiceMap, Error> {
    match env::var(VCAP_SERVICES) {
        Ok(services) => match serde_json::from_str::<ServiceMap>(&services) {
            Ok(value) => Ok(value),
            Err(_) => Err(Error::JsonMalformed(VCAP_SERVICES.to_string())),
        },
        Err(_) => Err(Error::EnvNotSet(VCAP_SERVICES.to_string())),
    }
}

/// Get's you a single service from`VCAP_SERVICES` by it's name
///
/// This allows you to get a service with it's credentials. The type `T` can be used to have a typed credentials struct. As the format of credentials is up to your provider it defaults to a generic Value from serde_json.
///
/// ```no_run
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize)]
/// pub struct CustomCredentials {
///     pub password: String,
///     pub username: String,
///     pub host: String,
///     pub port: u16,
/// }
///
/// // After that you can use it
///
/// let service = cf_env::get_service_by_name::<CustomCredentials>("my_db").unwrap();
/// ```
///
/// There is no need for typed credentials if you would like to parse it anyway
/// 
/// ```no_run
/// use serde_json::Value;
/// use cf_env::Service;
/// 
/// let service: Service<Value> = cf_env::get_service_by_name("my_db").unwrap();
/// 
/// let uri = service.credentials["uri"].as_str().unwrap();
/// ```
pub fn get_service_by_name<T>(name: &str) -> Result<Service<T>, Error>
where
    T: DeserializeOwned,
{
    match get_services() {
        Ok(services) => {
            for key in services.keys() {
                for service in services.get(key).unwrap().iter() {
                    if service.name == name {
                        let service_json = serde_json::to_string(service).unwrap();
                        match serde_json::from_str::<Service<T>>(&service_json) {
                            Ok(service) => return Ok(service),
                            Err(_) => {
                                return Err(Error::JsonMalformed(format!(
                                    "{}.credentials",
                                    service.name
                                )))
                            }
                        }
                    }
                }
            }
            Err(Error::ServiceNotPresent(name.to_string()))
        }
        Err(e) => Err(e),
    }
}

/// Get's you a a list services from`VCAP_SERVICES` by their type
/// 
/// This allows you to get the services with their credentials. The type `T` can be used to have a typed credentials struct. As the format of credentials is up to your provider it defaults to a generic Value from serde_json.
///
/// ```no_run
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize)]
/// pub struct CustomCredentials {
///     pub password: String,
///     pub username: String,
///     pub host: String,
///     pub port: u16,
/// }
///
/// // After that you can use it
///
/// let services = cf_env::get_services_by_type::<CustomCredentials>("a_type_of_service").unwrap();
/// ```
/// 
/// There is no need for typed credentials if you would like to parse it anyway
/// 
/// ```no_run
/// use serde_json::Value;
/// use cf_env::Service;
/// 
/// let services: Vec<Service<Value>>  = cf_env::get_services_by_type("a_type_of_service").unwrap();
/// 
/// let uri = services[0].credentials["uri"].as_str().unwrap();
/// ```
pub fn get_services_by_type<T>(type_name: &str) -> Result<Vec<Service<T>>, Error>
where
    T: DeserializeOwned,
{
    match get_services() {
        Ok(services) => {
            if services.get(type_name).is_some() {
                let service_json = serde_json::to_string(services.get(type_name).unwrap()).unwrap();
                match serde_json::from_str::<Vec<Service<T>>>(&service_json) {
                    Ok(service) => return Ok(service),
                    Err(_) => {
                        return Err(Error::JsonMalformed(format!("<{}>.credentials", type_name)))
                    }
                }
            }
            Err(Error::ServiceTypeNotPresent(type_name.to_string()))
        }
        Err(e) => Err(e),
    }
}

/// Get's you the information from `VCAP_APPLICATION` as a typed Application
pub fn get_application_info() -> Result<Application, Error> {
    match env::var(VCAP_APPLICATION) {
        Ok(application) => match serde_json::from_str::<Application>(&application) {
            Ok(value) => Ok(value),
            Err(_) => Err(Error::JsonMalformed(VCAP_APPLICATION.to_string())),
        },
        Err(_) => Err(Error::EnvNotSet(VCAP_APPLICATION.to_string())),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_instance_addr_valid() {
        std::env::set_var("CF_INSTANCE_ADDR", "10.24.8.2:8080");
        let instance_addr_result = crate::get_instance_address();

        assert_eq!(instance_addr_result.is_ok(), true);

        let instance_addr = instance_addr_result.unwrap();
        assert_eq!(instance_addr.to_string(), "10.24.8.2:8080".to_string());
        assert_eq!(instance_addr.ip(), "10.24.8.2".parse::<std::net::IpAddr>().unwrap());
        assert_eq!(instance_addr.port(), 8080);
    }

    #[test]
    fn get_instance_addr_invalid_port() {
        std::env::set_var("CF_INSTANCE_ADDR", "10.24.8.2:port");
        let instance_addr_result = crate::get_instance_address();

        assert_eq!(instance_addr_result.is_ok(), false);
    }

    #[test]
    fn get_instance_addr_invalid_host() {
        std::env::set_var("CF_INSTANCE_ADDR", "host:8080");
        let instance_addr_result = crate::get_instance_address();

        assert_eq!(instance_addr_result.is_ok(), false);
    }

    #[test]
    fn get_guid_valid() {
        std::env::set_var("CF_INSTANCE_GUID", "046463bc-1ba9-4046-bf5a-bd95672ee871");
        let guid_result = crate::get_instance_guid();

        assert_eq!(guid_result.is_ok(), true);
        assert_eq!(guid_result.unwrap(), guid_create::GUID::parse("046463bc-1ba9-4046-bf5a-bd95672ee871").unwrap());
    }

    #[test]
    fn get_guid_invalid() {
        std::env::set_var("CF_INSTANCE_GUID", "046463bc-1ba9-4046-bf5a-bd95672ee81");
        let guid_result = crate::get_instance_guid();

        assert_eq!(guid_result.is_ok(), false);
    }

    #[test]
    fn get_instance_index_valid() {
        std::env::set_var("CF_INSTANCE_INDEX", "8");
        let index_result = crate::get_instance_index();

        assert_eq!(index_result.is_ok(), true);
        assert_eq!(index_result.unwrap(), 8);
    }

    #[test]
    fn get_instance_index_invalid_negative() {
        std::env::set_var("CF_INSTANCE_INDEX", "-1");
        let index_result = crate::get_instance_index();

        assert_eq!(index_result.is_ok(), false);
    }

    #[test]
    fn get_instance_index_invalid_non_number() {
        std::env::set_var("CF_INSTANCE_INDEX", "hello");
        let index_result = crate::get_instance_index();

        assert_eq!(index_result.is_ok(), false);
    }
}
