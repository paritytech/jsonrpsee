use crate::hosts::AllowedHosts;
use crate::cors::{AccessControlAllowHeaders,AccessControlAllowOrigin};

#[derive(Clone)]
pub struct AccessControl {
    pub allowed_hosts: AllowedHosts, 
    pub cors_allow_origin: Option<Vec<AccessControlAllowOrigin>>,
    pub cors_max_age: Option<u32>,
    pub cors_allow_headers: AccessControlAllowHeaders,
    pub continue_on_invalid_cors: bool,
}

impl Default for AccessControl {
    fn default() -> Self {
        Self {
            allowed_hosts: AllowedHosts::Any,
            cors_allow_origin: None,
            cors_max_age: None,
            cors_allow_headers: AccessControlAllowHeaders::Any,
            continue_on_invalid_cors: false,
        }
    }
}

