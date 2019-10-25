use crate::hosts::{AllowHosts,Host};
use crate::cors::{AccessControlAllowHeaders,AccessControlAllowOrigin};

#[derive(Clone)]
pub struct AccessControl {
    pub allow_hosts: AllowHosts, 
    pub cors_allow_origin: Option<Vec<AccessControlAllowOrigin>>,
    pub cors_max_age: Option<u32>,
    pub cors_allow_headers: AccessControlAllowHeaders,
    pub continue_on_invalid_cors: bool,
}

impl Default for AccessControl {
    fn default() -> Self {
        Self {
            allow_hosts: AllowHosts::Any,
            cors_allow_origin: None,
            cors_max_age: None,
            cors_allow_headers: AccessControlAllowHeaders::Any,
            continue_on_invalid_cors: false,
        }
    }
}

pub struct AccessControlBuilder {
    allow_hosts: AllowHosts, 
    cors_allow_origin: Option<Vec<AccessControlAllowOrigin>>,
    cors_max_age: Option<u32>,
    cors_allow_headers: AccessControlAllowHeaders,
    continue_on_invalid_cors: bool,
}

impl AccessControlBuilder {
    pub fn new() -> Self {
        AccessControlBuilder {
            allow_hosts: AllowHosts::Any,
            cors_allow_origin: None,
            cors_max_age: None,
            cors_allow_headers: AccessControlAllowHeaders::Any,
            continue_on_invalid_cors: false,
        }
    }

    pub fn allow_host(mut self, host: Host) -> Self {
        let allow_hosts = match self.allow_hosts {
            AllowHosts::Any => {                
                vec![host]
            }
            AllowHosts::Only(mut allow_hosts) => {
                allow_hosts.push(host);
                allow_hosts                
            }
        };
        self.allow_hosts = AllowHosts::Only(allow_hosts);
        self
    }

    pub fn cors_allow_origin(mut self, allow_origin: AccessControlAllowOrigin) -> Self {
        let cors_allow_origin = match self.cors_allow_origin {
            Some(mut cors_allow_origin) => {
                cors_allow_origin.push(allow_origin);
                cors_allow_origin
            }
            None => {
                vec![allow_origin]
            }
        };
        self.cors_allow_origin = Some(cors_allow_origin);
        self
    }

    pub fn cors_max_age(mut self, max_age: u32) -> Self {
        self.cors_max_age = Some(max_age);
        self
    }

    pub fn cors_allow_header(mut self, header: String) -> Self {
        let allow_headers = match self.cors_allow_headers {
            AccessControlAllowHeaders::Any => vec![header],
            AccessControlAllowHeaders::Only(mut allow_headers) => {
                allow_headers.push(header);
                allow_headers
            }
        };
        self.cors_allow_headers = AccessControlAllowHeaders::Only(allow_headers);
        self
    }

    pub fn continue_on_invalid_cors(mut self, continue_on_invalid_cors: bool) -> Self {
        self.continue_on_invalid_cors = continue_on_invalid_cors;
        self
    }

    pub fn build(self) -> AccessControl {
        AccessControl {
            allow_hosts: self.allow_hosts, 
            cors_allow_origin: self.cors_allow_origin,
            cors_max_age: self.cors_max_age,
            cors_allow_headers: self.cors_allow_headers,
            continue_on_invalid_cors: self.continue_on_invalid_cors,
        }
    }
}