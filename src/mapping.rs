use std::collections::HashMap;
use opentelemetry_sdk::Resource;
use tracing::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonitoredResourceData {
    pub r#type: String,
    pub labels: HashMap<String, String>,
}

pub fn get_monitored_resource(res: Resource) -> Option<MonitoredResourceData> {
    unimplemented!()
}



