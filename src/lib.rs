//! Easy and typed abstraction on Cloud Foundry Environment variables
//!
//! It allows you to get:
//! - Services, optionally filtered by name or type
//! - Application infos
//! - Cloud Foundry app instance set variables, most of the times starting with `CF_`
//!
//! It's meant to get you away from boilerplating and get you started with typed variables while keeping simplicity
#![deny(clippy::all, clippy::cargo)]
#![forbid(unsafe_code)]

pub mod constants;
pub mod enums;
pub mod models;

#[doc(hidden)]
pub use constants::*;
#[doc(hidden)]
pub use enums::*;
#[doc(hidden)]
pub use models::*;

use guid_create::GUID;
use http::Uri;
use locale_types::Locale;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::panic;
use std::path::PathBuf;
use std::str::FromStr;

/// Checks if `VCAP_APPLICATION` is defined, if so uses as the indicator that the app is running in a Cloud Foundry Environment.
///
/// Use this with caution. To use the flexibility of cargo and optimization of rust and llvm you should only use this if there is no other way. One other possible way would be to use features flags for your binaries and use them to identify for which environment you build.
pub fn is_cf_env() -> bool {
    env::var(VCAP_APPLICATION).is_ok()
}

/// Get's the value from `CF_INSTANCE_ADDR` as a typed SocketAddr
pub fn get_instance_address() -> Result<SocketAddr, Error<'static>> {
    match env::var(CF_INSTANCE_ADDR) {
        Ok(addr_string) => match addr_string.parse::<SocketAddr>() {
            Ok(socket) => Ok(socket),
            Err(_) => Err(Error::EnvMalformed(
                CF_INSTANCE_ADDR.to_string(),
                "Doesn't match the format of addr:ip".to_string(),
            )),
        },
        Err(_) => Err(Error::EnvNotSet(CF_INSTANCE_ADDR)),
    }
}

/// Get's the value from `CF_INSTANCE_GUID` as a typed GUID
pub fn get_instance_guid() -> Result<GUID, Error<'static>> {
    match env::var(CF_INSTANCE_GUID) {
        Ok(guid_string) => match GUID::parse(&guid_string) {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::EnvMalformed(
                CF_INSTANCE_GUID.to_string(),
                "Isn't a valid guid".to_string(),
            )),
        },
        Err(_) => Err(Error::EnvNotSet(CF_INSTANCE_GUID)),
    }
}

/// Get's the value from `CF_INSTANCE_INDEX` as a typed u128
pub fn get_instance_index() -> Result<u128, Error<'static>> {
    match env::var(CF_INSTANCE_INDEX) {
        Ok(index_string) => match index_string.parse::<u128>() {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::EnvMalformed(
                CF_INSTANCE_INDEX.to_string(),
                "Ins't a valid positive (u128) number".to_string(),
            )),
        },
        Err(_) => Err(Error::EnvNotSet(CF_INSTANCE_INDEX)),
    }
}

/// Get's the value from `CF_INSTANCE_IP` as a typed IpAddr
pub fn get_instance_ip() -> Result<IpAddr, Error<'static>> {
    match env::var(CF_INSTANCE_IP) {
        Ok(ip_string) => match ip_string.parse::<IpAddr>() {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::EnvMalformed(
                CF_INSTANCE_IP.to_string(),
                "Ins't a valid ip address".to_string(),
            )),
        },
        Err(_) => Err(Error::EnvNotSet(CF_INSTANCE_IP)),
    }
}

/// Get's the value from `CF_INSTANCE_INTERNAL_IP` as a typed IpAddr
pub fn get_instance_internal_ip() -> Result<IpAddr, Error<'static>> {
    match env::var(CF_INSTANCE_INTERNAL_IP) {
        Ok(ip_string) => match ip_string.parse::<IpAddr>() {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::EnvMalformed(
                CF_INSTANCE_INTERNAL_IP.to_string(),
                "Ins't a valid ip address".to_string(),
            )),
        },
        Err(_) => Err(Error::EnvNotSet(CF_INSTANCE_INTERNAL_IP)),
    }
}

/// Get's the value from `CF_INSTANCE_PORT` as a typed u16
pub fn get_instance_port() -> Result<u16, Error<'static>> {
    match env::var(CF_INSTANCE_PORT) {
        Ok(index_string) => match index_string.parse::<u16>() {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::EnvMalformed(
                CF_INSTANCE_PORT.to_string(),
                "Ins't a valid positive (u16) number".to_string(),
            )),
        },
        Err(_) => Err(Error::EnvNotSet(CF_INSTANCE_PORT)),
    }
}

/// Get's the value from `DATABASE_URL` as a typed Uri
pub fn get_database_url() -> Result<Uri, Error<'static>> {
    match env::var(DATABASE_URL) {
        Ok(index_string) => match index_string.parse::<Uri>() {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::EnvMalformed(
                DATABASE_URL.to_string(),
                "Ins't a valid uri".to_string(),
            )),
        },
        Err(_) => Err(Error::EnvNotSet(DATABASE_URL)),
    }
}

/// Get's the value from `HOME` as a typed PathBuf
pub fn get_home() -> Result<PathBuf, Error<'static>> {
    match env::var(HOME) {
        Ok(home_string) => Ok(PathBuf::from(home_string)),
        Err(_) => Err(Error::EnvNotSet(HOME)),
    }
}

/// Get's the value from `LANG` as a typed Locale
pub fn get_lang() -> Result<Locale, Error<'static>> {
    match env::var(LANG) {
        Ok(lang_string) => {
            let parse_result = panic::catch_unwind(|| Locale::from_str(&lang_string));

            if parse_result.is_err() {
                return Err(Error::EnvMalformed(
                    LANG.to_string(),
                    "Ins't a valid locale".to_string(),
                ));
            }

            match parse_result.unwrap() {
                Ok(result) => Ok(result),
                Err(_) => Err(Error::EnvMalformed(
                    LANG.to_string(),
                    "Ins't a valid locale".to_string(),
                )),
            }
        }
        Err(_) => Err(Error::EnvNotSet(LANG)),
    }
}

/// Get's the value from `MEMORY_LIMIT` as a typed MemoryLimit
pub fn get_memory_limit() -> Result<MemoryLimit, Error<'static>> {
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
        Err(_) => Err(Error::EnvNotSet(MEMORY_LIMIT)),
    }
}

/// Get's the value from `PORT` as a typed 16
pub fn get_port() -> Result<u16, Error<'static>> {
    match env::var(PORT) {
        Ok(port_string) => match port_string.parse::<u16>() {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::EnvMalformed(
                PORT.to_string(),
                "Ins't a valid positive (u16) number".to_string(),
            )),
        },
        Err(_) => Err(Error::EnvNotSet(PORT)),
    }
}

/// Get's the value from `PWD` as a typed PathBuf
pub fn get_pwd() -> Result<PathBuf, Error<'static>> {
    match env::var(PWD) {
        Ok(pwd_string) => Ok(PathBuf::from(pwd_string)),
        Err(_) => Err(Error::EnvNotSet(PWD)),
    }
}

/// Get's the value from `TMPDIR` as a typed PathBuf
pub fn get_tmp_dir() -> Result<PathBuf, Error<'static>> {
    match env::var(TMPDIR) {
        Ok(tmp_dir) => Ok(PathBuf::from(tmp_dir)),
        Err(_) => Err(Error::EnvNotSet(TMPDIR)),
    }
}

/// Get's the value from `USER`
pub fn get_user() -> Result<String, Error<'static>> {
    match env::var(USER) {
        Ok(user_string) => Ok(user_string),
        Err(_) => Err(Error::EnvNotSet(USER)),
    }
}

type ServiceMap = HashMap<String, Vec<Service>>;

/// Get's the value from `VCAP_SERVICES` as a typed HashMap of Strings and a list of Services
pub fn get_services() -> Result<ServiceMap, Error<'static>> {
    match env::var(VCAP_SERVICES) {
        Ok(services) => match serde_json::from_str::<ServiceMap>(&services) {
            Ok(value) => Ok(value),
            Err(_err) => Err(Error::JsonMalformed(VCAP_SERVICES.to_string())),
        },
        Err(_) => Err(Error::EnvNotSet(VCAP_SERVICES)),
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
                                    service.name.to_owned()
                                )))
                            }
                        }
                    }
                }
            }
            Err(Error::ServiceNotPresent(name))
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
                    Err(_err) => return Err(Error::JsonMalformed(format!("<{}>.credentials", type_name))),
                }
            }
            Err(Error::ServiceTypeNotPresent(type_name))
        }
        Err(e) => Err(e),
    }
}

/// Get's you the information from `VCAP_APPLICATION` as a typed Application
pub fn get_application_info() -> Result<Application, Error<'static>> {
    match env::var(VCAP_APPLICATION) {
        Ok(application) => match serde_json::from_str::<Application>(&application) {
            Ok(value) => Ok(value),
            Err(_err) => Err(Error::JsonMalformed(VCAP_APPLICATION.to_string())),
        },
        Err(_) => Err(Error::EnvNotSet(VCAP_APPLICATION)),
    }
}

#[cfg(test)]
mod tests {

    const SERVICE_DATA: &str = "
    {
        \"secure-auth\": [
          {
            \"label\": \"secure-auth\",
            \"provider\": null,
            \"plan\": \"beta\",
            \"name\": \"my-app-backend-auth\",
            \"tags\": [
              \"oidc\",
              \"jwt\"
            ],
            \"instance_guid\": \"720a4210-3ea0-44e0-b3e3-63ad833191a9\",
            \"instance_name\": \"my-app-backend-auth\",
            \"binding_guid\": \"8d2b186f-22a6-48a8-bb38-df5320987812\",
            \"binding_name\": null,
            \"credentials\": {
              \"authorizationEndpoint\": \"https://authority.example.io/intern/authorize\",
              \"clientId\": \"VEO7igWPLpRIzB9IDoUAXhbsowklLn8u93hYyOYSBbPANMiLE5UGo0wpqdasHTDNdP\",
              \"clientSecret\": \"null\",
              \"accessTokenValidity\": \"900\",
              \"tokenEndpoint\": \"https://authority.example.io/intern/token\",
              \"userInfoEndpoint\": \"https://authority.example.io/userinfo\",
              \"logoutEndpoint\": \"https://authority.example.io/logout\",
              \"redirectUris\": \"https://memory-service.example.io/oidc/callback,http://localhost:8080/oidc/callback\",
              \"grantTypes\": \"client_credentials\"
            },
            \"syslog_drain_url\": null,
            \"volume_mounts\": []
          }
        ],
        \"mongodb\": [
          {
            \"label\": \"mongodb\",
            \"provider\": null,
            \"plan\": \"huge\",
            \"name\": \"my-db\",
            \"tags\": [
              \"mongodb\",
              \"mongo\"
            ],
            \"instance_guid\": \"2b6e08f9-3174-46ff-999d-183dc4c4964d\",
            \"instance_name\": \"lpn-db\",
            \"binding_guid\": \"3290823d-ab9f-4d72-b414-2438144ea9dc\",
            \"binding_name\": null,
            \"credentials\": {
              \"host\": \"b4386ed6-2770-444e-9a5d-727855f758fa.services.intern\",
              \"port\": \"27801\",
              \"database\": \"rs_e01562b7-04e7-46fb-b348-03fb70d442f8\",
              \"username\": \"598bd0f7-9b35-42df-a504-e1964f2698f4\",
              \"password\": \"olskjlkjasf09823ja0a\",
              \"database_uri\": \"mongodb://598bd0f7-9b35-42df-a504-e1964f2698f4:olskjlkjasf09823ja0a@b4386ed6-2770-444e-9a5d-727855f758fa.services.intern:37831\",
              \"uri\": \"mongodb://598bd0f7-9b35-42df-a504-e1964f2698f4:olskjlkjasf09823ja0a@b4386ed6-2770-444e-9a5d-727855f758fa.services.intern:37831\",
              \"replica_set\": \"rs_66a04559-c503-44f8-a2ef-28644b3cbce4\"
            },
            \"syslog_drain_url\": null,
            \"volume_mounts\": []
          }
        ]
      }";

    const APP_DATA: &str = "
      {
        \"cf_api\": \"https://api.example.io\",
        \"limits\": {
          \"fds\": 12384,
          \"mem\": 512,
          \"disk\": 1024
        },
        \"application_name\": \"my-backend\",
        \"application_uris\": [
          \"backend.example.io\"
        ],
        \"name\": \"my-backend\",
        \"space_name\": \"production\",
        \"space_id\": \"29349a46-ff0c-447e-bde0-db1be814f564\",
        \"organization_id\": \"26421037-ab23-4b51-99f8-9f5a6251fd26\",
        \"organization_name\": \"Project_One\",
        \"uris\": [
          \"backend.example.io\"
        ],
        \"users\": null,
        \"process_id\": \"d8304a62-2df7-41d5-9211-0917c2253591\",
        \"process_type\": \"web\",
        \"application_id\": \"d8304a62-2df7-41d5-9211-0917c2253591\",
        \"version\": \"9fe9fe07-c7b7-415b-afa3-75fef5258d47\",
        \"application_version\": \"9fe9fe07-c7b7-415b-afa3-75fef5258d47\"
      }";

    #[test]
    fn get_instance_addr_valid() {
        std::env::set_var("CF_INSTANCE_ADDR", "10.24.8.2:8080");
        let instance_addr_result = crate::get_instance_address();

        assert!(instance_addr_result.is_ok());

        let instance_addr = instance_addr_result.unwrap();
        assert_eq!(instance_addr.to_string(), "10.24.8.2:8080".to_string());
        assert_eq!(
            instance_addr.ip(),
            "10.24.8.2".parse::<std::net::IpAddr>().unwrap()
        );
        assert_eq!(instance_addr.port(), 8080);
    }

    #[test]
    fn get_instance_addr_invalid_port() {
        std::env::set_var("CF_INSTANCE_ADDR", "10.24.8.2:port");
        let instance_addr_result = crate::get_instance_address();

        assert!(!instance_addr_result.is_ok());
    }

    #[test]
    fn get_instance_addr_invalid_not_defined() {
        std::env::remove_var(crate::CF_INSTANCE_ADDR);
        let instance_addr_result = crate::get_instance_address();

        assert!(!instance_addr_result.is_ok());
    }

    #[test]
    fn get_instance_addr_invalid_host() {
        std::env::set_var("CF_INSTANCE_ADDR", "host:8080");
        let instance_addr_result = crate::get_instance_address();

        assert!(!instance_addr_result.is_ok());
    }

    #[test]
    fn get_guid_valid() {
        std::env::set_var("CF_INSTANCE_GUID", "046463bc-1ba9-4046-bf5a-bd95672ee871");
        let guid_result = crate::get_instance_guid();

        assert!(guid_result.is_ok());
        assert_eq!(
            guid_result.unwrap(),
            guid_create::GUID::parse("046463bc-1ba9-4046-bf5a-bd95672ee871").unwrap()
        );
    }

    #[test]
    fn get_guid_invalid() {
        std::env::set_var("CF_INSTANCE_GUID", "046463bc-1ba9-4046-bf5a-bd95672ee81");
        let guid_result = crate::get_instance_guid();

        assert!(!guid_result.is_ok());
    }

    #[test]
    fn get_guid_invalid_not_defined() {
        std::env::remove_var(crate::CF_INSTANCE_GUID);
        let guid_result = crate::get_instance_guid();

        assert!(!guid_result.is_ok());
    }

    #[test]
    fn get_instance_index_valid() {
        std::env::set_var("CF_INSTANCE_INDEX", "8");
        let index_result = crate::get_instance_index();

        assert!(index_result.is_ok());
        assert_eq!(index_result.unwrap(), 8);
    }

    #[test]
    fn get_instance_index_invalid_negative() {
        std::env::set_var("CF_INSTANCE_INDEX", "-1");
        let index_result = crate::get_instance_index();

        assert!(!index_result.is_ok());
    }

    #[test]
    fn get_instance_index_invalid_not_defined() {
        std::env::remove_var(crate::CF_INSTANCE_INDEX);
        let index_result = crate::get_instance_index();

        assert!(!index_result.is_ok());
    }

    #[test]
    fn get_instance_index_invalid_non_number() {
        std::env::set_var("CF_INSTANCE_INDEX", "hello");
        let index_result = crate::get_instance_index();

        assert!(!index_result.is_ok());
    }

    #[test]
    fn get_instance_ip_invalid_domain() {
        std::env::set_var("CF_INSTANCE_IP", "me.com");
        let index_result = crate::get_instance_ip();

        assert!(!index_result.is_ok());
    }

    #[test]
    fn get_instance_ip_invalid_not_defined() {
        std::env::remove_var(crate::CF_INSTANCE_IP);
        let index_result = crate::get_instance_ip();

        assert!(!index_result.is_ok());
    }

    #[test]
    fn get_instance_ip_invalid_ip() {
        std::env::set_var("CF_INSTANCE_IP", "670.120.01.94");
        let index_result = crate::get_instance_ip();

        assert!(!index_result.is_ok());
    }

    #[test]
    fn get_instance_ip_valid() {
        std::env::set_var("CF_INSTANCE_IP", "192.168.2.3");
        let index_result = crate::get_instance_ip();

        assert!(index_result.is_ok());
        assert_eq!(index_result.unwrap().to_string(), "192.168.2.3".to_string());
    }

    #[test]
    fn get_instance_internal_ip_invalid_domain() {
        std::env::set_var("CF_INSTANCE_INTERNAL_IP", "me.com");
        let index_result = crate::get_instance_internal_ip();

        assert!(!index_result.is_ok());
    }

    #[test]
    fn get_instance_internal_ip_invalid_not_defined() {
        std::env::remove_var(crate::CF_INSTANCE_INTERNAL_IP);
        let index_result = crate::get_instance_internal_ip();

        assert!(!index_result.is_ok());
    }

    #[test]
    fn get_instance_internal_ip_invalid_ip() {
        std::env::set_var("CF_INSTANCE_INTERNAL_IP", "670.120.01.94");
        let index_result = crate::get_instance_internal_ip();

        assert!(!index_result.is_ok());
    }

    #[test]
    fn get_instance_internal_ip_valid() {
        std::env::set_var("CF_INSTANCE_INTERNAL_IP", "192.168.2.3");
        let index_result = crate::get_instance_internal_ip();

        assert!(index_result.is_ok());
        assert_eq!(index_result.unwrap().to_string(), "192.168.2.3".to_string());
    }

    #[test]
    fn get_instance_port_valid() {
        std::env::set_var("CF_INSTANCE_PORT", "8080");
        let port_result = crate::get_instance_port();

        assert!(port_result.is_ok());
        assert_eq!(port_result.unwrap(), 8080);
    }

    #[test]
    fn get_instance_port_invalid() {
        std::env::set_var("CF_INSTANCE_PORT", "hello");
        let port_result = crate::get_instance_port();

        assert!(!port_result.is_ok());
    }

    #[test]
    fn get_instance_port_invalid_not_defined() {
        std::env::remove_var(crate::CF_INSTANCE_PORT);
        let port_result = crate::get_instance_port();

        assert!(!port_result.is_ok());
    }

    #[test]
    fn get_port_valid() {
        std::env::set_var("PORT", "8080");
        let port_result = crate::get_port();

        assert!(port_result.is_ok());
        assert_eq!(port_result.unwrap(), 8080);
    }

    #[test]
    fn get_port_invalid() {
        std::env::set_var("PORT", "hello");
        let port_result = crate::get_port();

        assert!(!port_result.is_ok());
    }

    #[test]
    fn get_port_invalid_not_defined() {
        std::env::remove_var(crate::PORT);
        let port_result = crate::get_port();

        assert!(!port_result.is_ok());
    }

    #[test]
    fn get_lang_valid() {
        std::env::set_var("LANG", "en_US.UTF-8");
        let lang_result = crate::get_lang();

        assert!(lang_result.is_ok());
        assert_eq!(lang_result.unwrap().to_string(), "en_US.UTF-8".to_string());
    }

    #[test]
    fn get_lang_invalid() {
        std::env::set_var("LANG", "hello");
        let lang_result = crate::get_lang();

        assert!(!lang_result.is_ok());
    }

    #[test]
    fn get_lang_invalid_not_defined() {
        std::env::remove_var(crate::LANG);
        let lang_result = crate::get_lang();

        assert!(!lang_result.is_ok());
    }

    #[test]
    fn get_user_valid() {
        std::env::set_var(crate::USER, "vcap");
        let user_result = crate::get_user();

        assert!(user_result.is_ok());
        assert_eq!(user_result.unwrap(), "vcap".to_string());
    }

    #[test]
    fn get_user_invalid_not_defined() {
        std::env::remove_var(crate::USER);
        let user_result = crate::get_user();

        assert!(!user_result.is_ok());
    }

    #[test]
    fn get_memory_limit_invalid_unit() {
        std::env::set_var("MEMORY_LIMIT", "512K");
        let memory_limit_result = crate::get_memory_limit();

        assert!(!memory_limit_result.is_ok());
    }

    #[test]
    fn get_memory_limit_invalid_size() {
        std::env::set_var("MEMORY_LIMIT", "-512M");
        let memory_limit_result = crate::get_memory_limit();

        assert!(!memory_limit_result.is_ok());
    }

    #[test]
    fn get_memory_limit_invalid_not_defined() {
        std::env::remove_var(crate::MEMORY_LIMIT);
        let memory_limit_result = crate::get_memory_limit();

        assert!(!memory_limit_result.is_ok());
    }

    #[test]
    fn get_memory_limit_valid() {
        std::env::set_var("MEMORY_LIMIT", "512M");
        let memory_limit_result = crate::get_memory_limit();

        assert!(memory_limit_result.is_ok());
        assert_eq!(
            memory_limit_result.unwrap(),
            crate::MemoryLimit {
                unit: crate::ByteUnit::Megabyte,
                size: 512,
            }
        )
    }

    #[test]
    fn get_app_info_valid() {
        std::env::set_var("VCAP_APPLICATION", APP_DATA);
        let app_info_result = crate::get_application_info();

        assert!(app_info_result.is_ok());
        assert_eq!(
            app_info_result.unwrap(),
            serde_json::from_str::<crate::Application>(APP_DATA).unwrap()
        );
    }

    #[test]
    fn get_app_info_invalid() {
        let mut data = APP_DATA.to_string();
        data.pop();
        std::env::set_var("VCAP_APPLICATION", data);
        let app_info_result = crate::get_application_info();

        assert!(!app_info_result.is_ok());
    }

    #[test]
    fn get_app_info_invalid_not_set() {
        std::env::remove_var("VCAP_APPLICATION");
        let app_info_result = crate::get_application_info();

        assert!(!app_info_result.is_ok());
    }

    #[test]
    fn get_services_valid() {
        use std::collections::HashMap;
        std::env::set_var("VCAP_SERVICES", SERVICE_DATA);
        let service_info = crate::get_services();

        assert!(service_info.is_ok());
        assert_eq!(
            service_info.unwrap(),
            serde_json::from_str::<HashMap<String, Vec<crate::Service>>>(SERVICE_DATA).unwrap()
        );
    }

    #[test]
    fn get_services_invalid_not_set() {
        std::env::remove_var("VCAP_SERVICES");
        let service_info = crate::get_services();

        assert!(!service_info.is_ok());
    }

    #[test]
    fn get_services_invalid_mal_formed() {
        let mut data = SERVICE_DATA.to_string();
        data.pop();

        std::env::set_var("VCAP_SERVICES", data);

        let service_info = crate::get_services();

        assert!(!service_info.is_ok());
    }

    #[test]
    fn get_services_by_name_valid() {
        std::env::set_var("VCAP_SERVICES", SERVICE_DATA);

        #[derive(serde::Serialize, serde::Deserialize)]
        struct DbCredentials {
            host: String,
            port: String,
            database: String,
            username: String,
            password: String,
            database_uri: String,
            uri: String,
            replica_set: String,
        }
        let service_info = crate::get_service_by_name::<DbCredentials>("my-db");

        assert!(service_info.is_ok());
        assert_eq!(service_info.unwrap().name, "my-db");
    }

    #[test]
    fn get_services_by_name_invalid_creds_schema() {
        std::env::set_var("VCAP_SERVICES", SERVICE_DATA);

        #[derive(serde::Serialize, serde::Deserialize)]
        struct DbCredentials {
            host: String,
            port: String,
            database: String,
            username: String,
            password: String,
            database_uri: String,
            uri: String,
            replica_set: String,
            not_avaiable: String,
        }
        let service_info = crate::get_service_by_name::<DbCredentials>("my-db");

        assert!(!service_info.is_ok());
    }

    #[test]
    fn get_services_by_name_invalid() {
        std::env::set_var("VCAP_SERVICES", SERVICE_DATA);

        let service_info = crate::get_service_by_name::<serde_json::Value>("the-db");

        assert!(!service_info.is_ok());
    }

    #[test]
    fn get_services_by_name_invalid_not_set() {
        std::env::remove_var("VCAP_SERVICES");

        let service_info = crate::get_service_by_name::<serde_json::Value>("the-db");

        assert!(!service_info.is_ok());
    }

    #[test]
    fn get_services_by_type_valid() {
        std::env::set_var("VCAP_SERVICES", SERVICE_DATA);

        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        struct DbCredentials {
            pub host: String,
            pub port: String,
            pub database: String,
            pub username: String,
            pub password: String,
            pub database_uri: String,
            pub uri: String,
            pub replica_set: String,
        }
        let service_info = crate::get_services_by_type::<DbCredentials>("mongodb");

        assert!(service_info.is_ok());

        let data = service_info.unwrap();
        let data: &crate::Service<DbCredentials> = data.get(0).unwrap();

        assert_eq!(data.name, "my-db");
        assert_eq!(data.label, "mongodb");
    }

    #[test]
    fn get_services_by_type_invalid_creds_schema() {
        std::env::set_var("VCAP_SERVICES", SERVICE_DATA);

        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        struct DbCredentials {
            pub host: String,
            pub port: String,
            pub database: String,
            pub username: String,
            pub password: String,
            pub database_uri: String,
            pub uri: String,
            pub replica_set: String,
            pub not_avaiable: String,
        }
        let service_info = crate::get_services_by_type::<DbCredentials>("mongodb");

        assert!(!service_info.is_ok());
    }

    #[test]
    fn get_services_by_type_invalid() {
        std::env::set_var("VCAP_SERVICES", SERVICE_DATA);

        let service_info = crate::get_services_by_type::<serde_json::Value>("some-type");

        assert!(!service_info.is_ok());
    }

    #[test]
    fn get_services_by_type_invalid_not_set() {
        std::env::remove_var("VCAP_SERVICES");

        let service_info = crate::get_services_by_type::<serde_json::Value>("some-type");

        assert!(!service_info.is_ok());
    }

    #[test]
    fn get_database_url_valid() {
        std::env::set_var("DATABASE_URL", "mysql://root:root@192.168.2.3:3098");
        let database_url_result = crate::get_database_url();

        assert!(database_url_result.is_ok());
        assert_eq!(
            database_url_result.unwrap().to_string(),
            "mysql://root:root@192.168.2.3:3098/".to_string()
        );
    }

    #[test]
    fn get_database_url_invalid() {
        std::env::set_var("DATABASE_URL", "mysql:/root@root@192.168.2.3:3098");
        let database_url_result = crate::get_database_url();

        assert!(!database_url_result.is_ok());
    }

    #[test]
    fn get_database_url_invalid_not_set() {
        std::env::remove_var(crate::DATABASE_URL);
        let database_url_result = crate::get_database_url();

        assert!(!database_url_result.is_ok());
    }

    #[test]
    fn get_home_valid() {
        std::env::set_var("HOME", "/home/vcap");
        let home_result = crate::get_home();

        assert!(home_result.is_ok());
        assert_eq!(home_result.unwrap().to_str().unwrap(), "/home/vcap");
    }

    #[test]
    fn get_home_not_set() {
        std::env::remove_var("HOME");
        let home_result = crate::get_home();

        assert!(!home_result.is_ok());
    }

    #[test]
    fn get_pwd_valid() {
        std::env::set_var("PWD", "/home/vcap");
        let pwd_result = crate::get_pwd();

        assert!(pwd_result.is_ok());
        assert_eq!(pwd_result.unwrap().to_str().unwrap(), "/home/vcap");
    }

    #[test]
    fn get_pwd_not_set() {
        std::env::remove_var("PWD");
        let pwd_result = crate::get_pwd();

        assert!(!pwd_result.is_ok());
    }

    #[test]
    fn get_tmp_dir_valid() {
        std::env::set_var("TMPDIR", "/tmp");
        let tmp_dir_result = crate::get_tmp_dir();

        assert!(tmp_dir_result.is_ok());
        assert_eq!(tmp_dir_result.unwrap().to_str().unwrap(), "/tmp");
    }

    #[test]
    fn get_tmp_dir_not_set() {
        std::env::remove_var("TMPDIR");
        let tmp_dir_result = crate::get_tmp_dir();

        assert!(!tmp_dir_result.is_ok());
    }

    #[test]
    fn is_cf_env_valid() {
        std::env::set_var("VCAP_APPLICATION", "test");
        let cf_env_result = crate::is_cf_env();

        assert!(cf_env_result);
    }

    #[test]
    fn is_cf_env_invalid() {
        std::env::remove_var("VCAP_APPLICATION");
        let cf_env_result = crate::is_cf_env();

        assert!(!cf_env_result);
    }
}
