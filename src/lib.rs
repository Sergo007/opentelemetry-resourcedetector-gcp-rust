#![allow(dead_code, unused_imports, unused_variables)]
pub mod get_val;
#[cfg(test)]
mod test_envs;
#[cfg(test)]
mod test_detector;

use core::str;
use std::{env, fs::File, io::Read, sync::Arc, time::Duration};

use error::OpenTelemetryError;
use opentelemetry::KeyValue;
use opentelemetry_sdk::{resource::ResourceDetector, Resource};
use regex::Regex;
use serde::de::value::Error;
use tracing::{info, warn};
pub mod error;

struct Zone {
    region: String,
    zone: String,
}

fn parse_zone(text: &str) -> Zone {
    let zone_region_re = Regex::new(r"projects/\d+/zones/(?P<zone>(?P<region>\w+-\w+)-\w+)").unwrap();

    // Example usage
    // let text = "projects/123456/zones/us-central1-a";
    if let Some(captures) = zone_region_re.captures(text) {
        if let Some(zone) = captures.name("zone") {
            if let Some(region) = captures.name("region") {
                return Zone {
                    region: region.as_str().to_string(),
                    zone: zone.as_str().to_string(),
                };
            }

        }
    }
    Zone {
        region: "".to_string(),
        zone: "".to_string(),
    }
}

async fn get_metadata() ->  Result<serde_json::Value, OpenTelemetryError> {
    let client = reqwest::Client::builder().build().map_err(OpenTelemetryError::new)?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Metadata-Flavor", "Google".parse().unwrap());

    let request = client.request(reqwest::Method::GET, "http://metadata.google.internal/computeMetadata/v1/?recursive=true")
        .headers(headers);

    let response = request.send().await.map_err(OpenTelemetryError::new)?;
    let body = response.json::<serde_json::Value>().await.map_err(OpenTelemetryError::new)?;
    Ok(body)
}


fn get_metadata_resources(metadata: &serde_json::Value) -> Result<Vec<KeyValue>, OpenTelemetryError> {
     let project_id = if let Some(serde_json::Value::String(project_id)) = get_val::get_val(metadata, &["project", "projectId"], None) {
        project_id
     } else {
        Err(OpenTelemetryError::new("project id not found"))?
     };
     let zone = if let Some(serde_json::Value::String(zone)) = get_val::get_val(metadata, &["instance", "zone"], None) {
        zone
     } else {
        Err(OpenTelemetryError::new("zone not found"))?
     };
    let attrs = vec![
        KeyValue::new("cloud.account.id", project_id.clone()),
        KeyValue::new("cloud.provider", "gcp"),
    ];
    Ok(attrs)
}


/// Resource finder for common GCE attributes
/// 
/// See: https://cloud.google.com/compute/docs/storing-retrieving-metadata
fn get_gce_resources(metadata: &serde_json::Value) -> Result<Vec<KeyValue>, OpenTelemetryError> {

    let mut attrs = get_metadata_resources(metadata)?;
    let host_id = if let Some(serde_json::Value::String(host_id)) = get_val::get_val(metadata, &["instance", "id"], None) {
        host_id.clone()
    } else {
        Err(OpenTelemetryError::new("host id not found"))?
    };
    let machine_type = if let Some(serde_json::Value::String(machine_type)) = get_val::get_val(metadata, &["instance", "machineType"], None) {
        machine_type.clone()
    } else {
        Err(OpenTelemetryError::new("not gce resources"))?
    };
    let zone_and_region = if let Some(serde_json::Value::String(zone_and_region)) = get_val::get_val(metadata, &["instance", "zone"], None) {
        parse_zone(zone_and_region)
    } else {
        Err(OpenTelemetryError::new("not gce resources"))?
    };
    let host_name = if let Some(serde_json::Value::String(host_name)) = get_val::get_val(metadata, &["instance", "name"], None) {
        host_name.clone()
    } else {
        Err(OpenTelemetryError::new("not gce resources"))?
    };
    
    attrs.push(KeyValue::new("cloud.platform", "gcp_compute_engine"));
    attrs.push(KeyValue::new("cloud.availability_zone", zone_and_region.zone));
    attrs.push(KeyValue::new("cloud.region", zone_and_region.region));
    attrs.push(KeyValue::new("host.type", machine_type));
    attrs.push(KeyValue::new("host.id", host_id));
    attrs.push(KeyValue::new("host.name", host_name));
    Ok(attrs)
}

/// Resource finder for GKE attributes
fn get_gke_resources(metadata: &serde_json::Value) -> Result<Vec<KeyValue>, OpenTelemetryError> {
    if env::var("KUBERNETES_SERVICE_HOST").is_err() {
        Err(OpenTelemetryError::new("KUBERNETES_SERVICE_HOST not found"))?
    }


    let mut attrs = get_metadata_resources(metadata)?;
    if let Ok(container_name) = env::var("CONTAINER_NAME") {
        attrs.push(KeyValue::new("container.name", container_name));
    }


    let pod_namespace = if let Ok(pod_namespace) = env::var("NAMESPACE") {
        pod_namespace
    } else {
        let namespace_path = "/var/run/secrets/kubernetes.io/serviceaccount/namespace";
        match File::open(namespace_path) {
            Ok(mut file) => {
                let mut namespace: Vec<u8> = Vec::new();
                if file.read_to_end(&mut namespace).is_ok() {
                    let s = match str::from_utf8(&namespace) {
                        Ok(v) => v,
                        Err(e) => "",
                    };
                    s.trim().to_string()
                } else {
                    String::new()
                }
            }
            Err(_) => String::new(),
        }
    };
    attrs.push(KeyValue::new("k8s.namespace.name", pod_namespace.clone()));

    let pod_name = if let Ok(pod_name) = env::var("POD_NAME") {
        pod_name
    } else if let Ok(pod_name) = env::var("HOSTNAME") {
        pod_name
    } else {
        String::new()
    };
    attrs.push(KeyValue::new("k8s.pod.name", pod_name.clone()));

    let cluster_name = if let Some(serde_json::Value::String(cluster_name)) = get_val::get_val(metadata, &["instance", "attributes", "cluster-name"], None) {
        cluster_name
    } else {
        Err(OpenTelemetryError::new("cluster name not found"))?
    };
    attrs.push(KeyValue::new("k8s.cluster.name", cluster_name.clone()));

    if let Some(serde_json::Value::String(cluster_location)) = get_val::get_val(metadata, &["instance", "attributes", "cluster-location"], None) {
        let hyphen_count = cluster_location.split('-').count();
        if hyphen_count == 2 {
            attrs.push(KeyValue::new("cloud.region", cluster_location.clone()));
        }
        if hyphen_count == 3 {
            attrs.push(KeyValue::new("cloud.availability_zone", cluster_location.clone()));
        }
    }

    if let Some(serde_json::Value::String(zone)) = get_val::get_val(metadata, &["instance", "zone"], None) {
        let zone = zone.split('/').last();
        if let Some(r) = zone {
            attrs.push(KeyValue::new("cloud.zone", r.to_string()));
        } else {
            Err(OpenTelemetryError::new("zone not found"))?
        }
    } else {
        Err(OpenTelemetryError::new("zone not found"))?
    };

    let host_id = if let Some(serde_json::Value::String(host_id)) = get_val::get_val(metadata, &["instance", "id"], None) {
        host_id
    } else {
        Err(OpenTelemetryError::new("host id not found"))?
    };
    attrs.push(KeyValue::new("host.id", host_id.clone()));

    attrs.push(KeyValue::new("gcp.resource_type", "gke_container"));
    attrs.push(KeyValue::new("cloud.platform", "gcp_kubernetes_engine"));

    Ok(attrs)
    
}


/// Resource finder for Cloud Run attributes
fn get_cloudrun_resources(metadata: &serde_json::Value) -> Result<Vec<KeyValue>, OpenTelemetryError> {
    if env::var("K_CONFIGURATION").is_err() {
        Err(OpenTelemetryError::new("K_CONFIGURATION not found"))?
    }

    let mut attrs = get_metadata_resources(metadata)?;
    if let Ok(faas_name) = env::var("K_SERVICE") {
        attrs.push(KeyValue::new("faas.name", faas_name.clone()));
    }

    if let Ok(faas_version) = env::var("K_REVISION") {
        attrs.push(KeyValue::new("faas.version", faas_version.clone()));
    }

    if let Some(serde_json::Value::String(region)) = get_val::get_val(metadata, &["instance", "region"], None) {
        let region = region.split('/').last();
        if let Some(r) = region {
            attrs.push(KeyValue::new("cloud.region", r.to_string()));
        } else {
            Err(OpenTelemetryError::new("region not found"))?
        }
    } else {
        Err(OpenTelemetryError::new("region not found"))?
    };

    if let Some(serde_json::Value::String(zone)) = get_val::get_val(metadata, &["instance", "zone"], None) {
        let zone = zone.split('/').last();
        if let Some(r) = zone {
            attrs.push(KeyValue::new("cloud.zone", r.to_string()));
        } else {
            Err(OpenTelemetryError::new("zone not found"))?
        }
    } else {
        Err(OpenTelemetryError::new("zone not found"))?
    };

    let instance_id = if let Some(serde_json::Value::String(instance_id)) = get_val::get_val(metadata, &["instance", "id"], None) {
        instance_id
    } else {
        Err(OpenTelemetryError::new("instance id not found"))?
    };
    attrs.push(KeyValue::new("faas.instance", instance_id.clone()));

    attrs.push(KeyValue::new("cloud.platform", "gcp_cloud_run"));
    attrs.push(KeyValue::new("gcp.resource_type", "cloud_run"));
    Ok(attrs)
}


/// Resource finder for Cloud Functions attributes
fn get_cloudfunctions_resources(metadata: &serde_json::Value) -> Result<Vec<KeyValue>, OpenTelemetryError> {
    if env::var("FUNCTION_TARGET").is_err() {
        Err(OpenTelemetryError::new("FUNCTION_TARGET not found"))?
    }

    let mut attrs = get_metadata_resources(metadata)?;

    if let Ok(faas_name) = env::var("K_SERVICE") {
        attrs.push(KeyValue::new("faas.name", faas_name.clone()));
    };
    if let Ok(faas_version) = env::var("K_REVISION") {
        attrs.push(KeyValue::new("faas.version", faas_version.clone()));
    };

    if let Some(serde_json::Value::String(region)) = get_val::get_val(metadata, &["instance", "region"], None) {
        let region = region.split('/').last();
        if let Some(r) = region {
            attrs.push(KeyValue::new("cloud.region", r.to_string()));
        } else {
            Err(OpenTelemetryError::new("region not found"))?
        }
    } else {
        Err(OpenTelemetryError::new("region not found"))?
    };

    if let Some(serde_json::Value::String(zone)) = get_val::get_val(metadata, &["instance", "zone"], None) {
        let zone = zone.split('/').last();
        if let Some(r) = zone {
            attrs.push(KeyValue::new("cloud.zone", r.to_string()));
        } else {
            Err(OpenTelemetryError::new("zone not found"))?
        }
    } else {
        Err(OpenTelemetryError::new("zone not found"))?
    };

    let instance_id = if let Some(serde_json::Value::String(instance_id)) = get_val::get_val(metadata, &["instance", "id"], None) {
        instance_id
    } else {
        Err(OpenTelemetryError::new("instance id not found"))?
    };
    attrs.push(KeyValue::new("faas.instance", instance_id.clone()));

    attrs.push(KeyValue::new("cloud.platform", "gcp_cloud_functions"));
    attrs.push(KeyValue::new("gcp.resource_type", "cloud_functions"));
    Ok(attrs)
}


pub struct GoogleCloudResourceDetector {
    attrs: Arc<Vec<KeyValue>>,
}

impl GoogleCloudResourceDetector {
    pub async fn new() -> Self {
        let metadata = match get_metadata().await {
            Ok(metadata) => metadata,
            Err(e) => {
                warn!("Failed to get metadata: {:?}", e);
                return Self {
                    attrs: Arc::new(vec![]),
                };
            }
        };

        // Order here matters. Since a GKE_CONTAINER is a specialized type of GCE_INSTANCE
        // We need to first check if it matches the criteria for being a GKE_CONTAINER
        // before falling back and checking if its a GCE_INSTANCE.
        // This list should be sorted from most specialized to least specialized.
        let attrs = if let Ok(kv) = get_gke_resources(&metadata) {
            kv
        } else if let Ok(kv) = get_cloudrun_resources(&metadata) {
            kv
        } else if let Ok(kv) = get_cloudfunctions_resources(&metadata) {
            kv
        } else if let Ok(kv) = get_gce_resources(&metadata) {
            kv
        } else {
            warn!("No resource found");
            return Self {
                attrs: Arc::new(vec![]),
            };
        };
        Self {
            attrs: Arc::new(attrs),
        }
    }

    pub fn get_resource(&self) -> Resource {
        Resource::new(self.attrs.as_ref().clone())
    }
}

impl ResourceDetector for GoogleCloudResourceDetector {
    fn detect(&self, timeout: Duration) -> Resource {
        self.get_resource()
    }
}

