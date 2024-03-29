use crate::enums::{ByteUnit, Error};
use guid_create::GUID;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MemoryLimit {
    pub unit: ByteUnit,
    pub size: u128,
}

impl MemoryLimit {
    pub fn from_string(mut input: String, env_variable: String) -> Result<Self, Error<'static>> {
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
            Err(_) => Err(Error::UnknownMemoryUnit),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApplicationLimits {
    pub disk: u128,
    pub fds: u128,
    pub mem: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Application {
    pub application_id: GUID,
    pub application_name: String,
    pub application_uris: Vec<String>,
    pub application_version: GUID,
    pub cf_api: String,
    pub limits: ApplicationLimits,
    pub name: String,
    pub process_id: String,
    pub process_type: String,
    pub organization_id: GUID,
    pub organization_name: String,
    pub space_id: GUID,
    pub space_name: String,
    pub start: Option<String>,
    pub started_at: Option<String>,
    pub started_at_timestamp: Option<String>,
    pub state_timestamp: Option<String>,
    pub uris: Vec<String>,
    pub version: GUID,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ServiceVolumeMount {
    pub container_dir: String,
    pub device_type: String,
    pub mode: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Service<Credentials = Value> {
    pub binding_guid: GUID,
    pub binding_name: Option<String>,
    pub instance_guid: GUID,
    pub instance_name: String,
    pub name: String,
    pub label: String,
    pub tags: Vec<String>,
    pub plan: String,
    pub credentials: Credentials,
    pub syslog_drain_url: Option<String>,
    pub volume_mounts: Vec<ServiceVolumeMount>,
}
