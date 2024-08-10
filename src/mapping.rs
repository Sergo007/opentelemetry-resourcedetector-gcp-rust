use std::{collections::HashMap, fmt::format};
use once_cell::sync::Lazy;
use opentelemetry_sdk::Resource;
use tracing::Value;

use crate::constants::{self, ResourceAttributes};

struct MapConfig<'a> {
    /// OTel resource keys to try and populate the resource label from. For entries with multiple
    /// OTel resource keys, the keys' values will be coalesced in order until there is a non-empty
    /// value.
    pub otel_keys: Vec<&'a str>,
    /// If none of the otelKeys are present in the Resource, fallback to this literal value
    pub fallback: &'a str,
}

impl <'a>MapConfig<'a> {
    pub fn new(otel_keys: Vec<&'a str>) -> Self {
        Self {
            otel_keys,
            fallback: "",
        }
    }
}

/// Mappings of GCM resource label keys onto mapping config from OTel resource for a given
/// monitored resource type. Copied from Go impl:
/// https://github.com/GoogleCloudPlatform/opentelemetry-operations-go/blob/v1.8.0/internal/resourcemapping/resourcemapping.go#L51
static MAPPINFS: Lazy<HashMap<&str, HashMap<&str, MapConfig>>> = Lazy::new(|| {
    HashMap::from([
        (
            constants::GCE_INSTANCE,
            HashMap::from([
                (
                    constants::ZONE,
                    MapConfig::new(vec![ResourceAttributes::CLOUD_AVAILABILITY_ZONE]),
                ),
                (
                    constants::INSTANCE_ID,
                    MapConfig::new(vec![ResourceAttributes::HOST_ID]),
                ),
            ]),
        ),
        (
            constants::K8S_CONTAINER,
            HashMap::from([
                (
                    constants::LOCATION,
                    MapConfig::new(vec![
                        ResourceAttributes::CLOUD_AVAILABILITY_ZONE, 
                        ResourceAttributes::CLOUD_REGION
                    ]),
                ),
                (
                    constants::CLUSTER_NAME,
                    MapConfig::new(vec![ResourceAttributes::K8S_CLUSTER_NAME]),
                ),
                (
                    constants::NAMESPACE_NAME,
                    MapConfig::new(vec![ResourceAttributes::K8S_NAMESPACE_NAME]),
                ),
                (
                    constants::POD_NAME,
                    MapConfig::new(vec![ResourceAttributes::K8S_POD_NAME]),
                ),
                (
                    constants::CONTAINER_NAME,
                    MapConfig::new(vec![ResourceAttributes::K8S_CONTAINER_NAME]),
                ),
            ]),
        ),
        (
            constants::K8S_POD,
            HashMap::from([
                (
                    constants::LOCATION,
                    MapConfig::new(vec![
                        ResourceAttributes::CLOUD_AVAILABILITY_ZONE, 
                        ResourceAttributes::CLOUD_REGION
                    ]),
                ),
                (
                    constants::CLUSTER_NAME,
                    MapConfig::new(vec![ResourceAttributes::K8S_CLUSTER_NAME]),
                ),
                (
                    constants::NAMESPACE_NAME,
                    MapConfig::new(vec![ResourceAttributes::K8S_NAMESPACE_NAME]),
                ),
                (
                    constants::POD_NAME,
                    MapConfig::new(vec![ResourceAttributes::K8S_POD_NAME]),
                ),
            ]),
        ),
        (
            constants::K8S_NODE,
            HashMap::from([
                (
                    constants::LOCATION,
                    MapConfig::new(vec![
                        ResourceAttributes::CLOUD_AVAILABILITY_ZONE, 
                        ResourceAttributes::CLOUD_REGION
                    ]),
                ),
                (
                    constants::CLUSTER_NAME,
                    MapConfig::new(vec![ResourceAttributes::K8S_CLUSTER_NAME]),
                ),
                (
                    constants::NODE_NAME,
                    MapConfig::new(vec![ResourceAttributes::K8S_NODE_NAME]),
                ),
            ]),
        ),
        (
            constants::K8S_CLUSTER,
            HashMap::from([
                (
                    constants::LOCATION,
                    MapConfig::new(vec![
                        ResourceAttributes::CLOUD_AVAILABILITY_ZONE, 
                        ResourceAttributes::CLOUD_REGION
                    ]),
                ),
                (
                    constants::CLUSTER_NAME,
                    MapConfig::new(vec![ResourceAttributes::K8S_CLUSTER_NAME]),
                ),
            ]),
        ),
        (
            constants::AWS_EC2_INSTANCE,
            HashMap::from([
                (
                    constants::INSTANCE_ID,
                    MapConfig::new(vec![ResourceAttributes::HOST_ID]),
                ),
                (
                    constants::REGION,
                    MapConfig::new(vec![
                        ResourceAttributes::CLOUD_AVAILABILITY_ZONE, 
                        ResourceAttributes::CLOUD_REGION
                    ]),
                ),
                (
                    constants::AWS_ACCOUNT,
                    MapConfig::new(vec![ResourceAttributes::CLOUD_ACCOUNT_ID]),
                ),
            ]),
        ),
        (
            constants::GENERIC_TASK,
            HashMap::from([
                (
                    constants::LOCATION,
                    MapConfig {
                        otel_keys: vec![
                            ResourceAttributes::CLOUD_AVAILABILITY_ZONE,
                            ResourceAttributes::CLOUD_REGION,
                        ],
                        fallback: "global",
                    },
                ),
                (
                    constants::NAMESPACE,
                    MapConfig::new(vec![
                        ResourceAttributes::SERVICE_NAMESPACE,
                    ]),
                ),
                (
                    constants::JOB,
                    MapConfig::new(vec![
                        ResourceAttributes::SERVICE_NAME,
                        ResourceAttributes::FAAS_NAME,
                    ]),
                ),
                (
                    constants::TASK_ID,
                    MapConfig::new(vec![
                        ResourceAttributes::SERVICE_INSTANCE_ID,
                        ResourceAttributes::FAAS_INSTANCE,
                    ]),
                ),
            ]),
        ),
        (
            constants::GENERIC_NODE,
            HashMap::from([
                (
                    constants::LOCATION,
                    MapConfig {
                        otel_keys: vec![
                            ResourceAttributes::CLOUD_AVAILABILITY_ZONE,
                            ResourceAttributes::CLOUD_REGION,
                        ],
                        fallback: "global",
                    },
                ),
                (
                    constants::NAMESPACE,
                    MapConfig::new(vec![ResourceAttributes::SERVICE_NAMESPACE]),
                ),
                (
                    constants::NODE_ID,
                    MapConfig::new(vec![
                        ResourceAttributes::HOST_ID,
                        ResourceAttributes::HOST_NAME,
                    ]),
                ),
            ]),
        ),
    ])
});


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonitoredResourceData {
    pub r#type: String,
    pub labels: HashMap<String, String>,
}

fn create_monitored_resource(
    monitored_resource_type: &str, 
    resource_attrs: HashMap<String, opentelemetry::Value>
) -> MonitoredResourceData {
    let mapping = MAPPINFS.get(monitored_resource_type).unwrap();
    let mut labels = HashMap::new();

    for (mr_key, map_config) in mapping.iter() {
        let mut mr_value = None;
        for otel_key in map_config.otel_keys.iter() {
            if let Some(v) = resource_attrs.get(*otel_key) {
                let v_string = v.to_string();
                if !v_string.starts_with(constants::UNKNOWN_SERVICE_PREFIX) {
                    mr_value = Some(v);
                    break;
                }
            }
        }

        if mr_value.is_none() && map_config.otel_keys.contains(&ResourceAttributes::SERVICE_NAME) {
            // The service name started with unknown_service, and was ignored above.
            mr_value = resource_attrs.get(ResourceAttributes::SERVICE_NAME);
        }

        let mr_value = match mr_value {
            Some(v) => v.clone(),
            None => opentelemetry::Value::String(map_config.fallback.into()),
        };


        // OTel attribute values can be any of str, bool, int, float, or Sequence of any of
        // them. Encode any non-strings as json string
        let mr_value_str: String = match mr_value {
            opentelemetry::Value::String(s) => s.to_string(),
            _ => format!("{}", mr_value),
        };

        labels.insert(mr_key.to_string(), mr_value_str);
    }

    MonitoredResourceData {
        r#type: monitored_resource_type.to_string(),
        labels,
    }
}



/// Add Google resource specific information (e.g. instance id, region).
/// 
/// See
/// https://cloud.google.com/monitoring/custom-metrics/creating-metrics#custom-metric-resources
/// for supported types
/// Args:
///         resource: OTel resource
/// 
pub fn get_monitored_resource(
    resource: Resource,
) -> Option<MonitoredResourceData> {
    let res_atters = resource.iter().map(|(k,v)| {
        (k.to_string(), v.clone())
    });
    let attrs: HashMap<String, opentelemetry::Value> = HashMap::from_iter(res_atters);
    let platform = attrs.get(ResourceAttributes::CLOUD_PLATFORM_KEY).map(|v| v.to_string());
    let platform = platform.as_ref().map(|v| v.as_str());
    let mr = match platform {
        Some(ResourceAttributes::GCP_COMPUTE_ENGINE) => {
            create_monitored_resource(constants::GCE_INSTANCE, attrs)
        },
        Some(ResourceAttributes::GCP_KUBERNETES_ENGINE) => {
            if attrs.get(ResourceAttributes::K8S_CONTAINER_NAME).is_some() {
                create_monitored_resource(constants::K8S_CONTAINER, attrs)
            } else if attrs.get(ResourceAttributes::K8S_POD_NAME).is_some() {
                create_monitored_resource(constants::K8S_POD, attrs)
            } else if attrs.get(ResourceAttributes::K8S_NODE_NAME).is_some() {
                create_monitored_resource(constants::K8S_NODE, attrs)
            } else {
                create_monitored_resource(constants::K8S_CLUSTER, attrs)
            }
        },
        Some(ResourceAttributes::AWS_EC2) => {
            create_monitored_resource(constants::AWS_EC2_INSTANCE, attrs)
        },
        _ => {
            // fallback to generic_task
            if( attrs.get(ResourceAttributes::SERVICE_NAME).is_some() || attrs.get(ResourceAttributes::FAAS_NAME).is_some()) && (attrs.get(ResourceAttributes::SERVICE_INSTANCE_ID).is_some() || attrs.get(ResourceAttributes::FAAS_INSTANCE).is_some()) {
                create_monitored_resource(constants::GENERIC_TASK, attrs)
            } else {
                create_monitored_resource(constants::GENERIC_NODE, attrs)
            }
        }
    };

    Some(mr)
}
