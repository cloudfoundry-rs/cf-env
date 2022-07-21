# Cloud Foundry Environment
[![codecov](https://codecov.io/gh/somehowchris/cf-env/branch/main/graph/badge.svg?token=8RVBW5DIMN)](https://codecov.io/gh/somehowchris/cf-env) ![Crates.io](https://img.shields.io/crates/v/cf-env) 

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/somehowchris/cf-env) 

A small library to get you going in no time with typed environment variables for you Cloud Foundry envirnoment.

Goals:
 - Remove boilerplate code for projects using cf
 - Use the power of rust to get the environment variables in a typed way
 - Give you the flexiblity to define custom values such as credential structures to meet your needs


## Usage

Getting this crate is easy as adding it to your dependencies
```toml
[dependencies]
cf-env = "0.1.7"
```

After that, just check what you need and get it, you may wanna check out the [docs.rs page](https://docs.rs/cf-env/). For example `CF_INSTANCE_INDEX`
```rust
let instance_index = cf_env::get_instance_index().unwrap();
```

Or for example if you need to get some credentials:
```rust
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct CustomCredentials {
    pub password: String,
    pub username: String,
    pub host: String,
    pub port: u16,
}

// After that you can use it
let service = cf_env::get_service_by_name::<CustomCredentials>("my_db").unwrap();
```

There is no need for typed credentials if you would like to parse it anyway and then deal with the `Value` enum from `serde_json`
```rust
use serde_json::Value;
use cf_env::Service;

let service: Service<Value> = cf_env::get_service_by_name("my_db").unwrap();

let uri = service.credentials["uri"].as_str().unwrap();
```
