#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use opentelemetry_sdk::Resource;
    use tracing::Value;
    use opentelemetry::{KeyValue, StringValue};
    use opentelemetry_sdk::export;
    use serde_json::json;

    use crate::mapping::{get_monitored_resource, MonitoredResourceData};

    use super::*;

    fn to_labels(kv: serde_json::Value) -> HashMap<String, String> {
        kv.as_object().unwrap().iter().map(|(k, v)| (k.to_string(), v.as_str().unwrap().to_string())).collect()
    }

    #[test]
    fn test_get_monitored_resource_k8s_container() {
        let res = Resource::new(vec![
            KeyValue::new("cloud.platform", "gcp_kubernetes_engine"),
            KeyValue::new("cloud.availability_zone", "myavailzone"),
            KeyValue::new("k8s.cluster.name", "mycluster"),
            KeyValue::new("k8s.namespace.name", "myns"),
            KeyValue::new("k8s.pod.name", "mypod"),
            KeyValue::new("k8s.container.name", "mycontainer"),
        ]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "k8s_container".to_string(),
            labels: to_labels(json!({
                "cluster_name": "mycluster",
                "container_name": "mycontainer",
                "location": "myavailzone",
                "namespace_name": "myns",
                "pod_name": "mypod",
            })),
        };
    }
    
    #[test]
    fn test_get_monitored_resource_k8s_container_region_fallback() {
        let res = Resource::new(vec![
            KeyValue::new("cloud.platform", "gcp_kubernetes_engine"),
            KeyValue::new("cloud.region", "myregion"),
            KeyValue::new("k8s.cluster.name", "mycluster"),
            KeyValue::new("k8s.namespace.name", "myns"),
            KeyValue::new("k8s.pod.name", "mypod"),
            KeyValue::new("k8s.container.name", "mycontainer"),
        ]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "k8s_container".to_string(),
            labels: to_labels(json!({
                "cluster_name": "mycluster",
                "container_name": "mycontainer",
                "location": "myregion",
                "namespace_name": "myns",
                "pod_name": "mypod",
            })),
        };
    }
    

    #[test]
    fn test_get_monitored_resource_k8s_pod() {
        let res = Resource::new(vec![
            KeyValue::new("cloud.platform", "gcp_kubernetes_engine"),
            KeyValue::new("cloud.availability_zone", "myavailzone"),
            KeyValue::new("k8s.cluster.name", "mycluster"),
            KeyValue::new("k8s.namespace.name", "myns"),
            KeyValue::new("k8s.pod.name", "mypod"),
        ]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "k8s_pod".to_string(),
            labels: to_labels(json!({
                "cluster_name": "mycluster",
                "location": "myavailzone",
                "namespace_name": "myns",
                "pod_name": "mypod",
            })),
        };
    }

    #[test]
    fn test_get_monitored_resource_k8s_pod_region_fallback() {
        let res = Resource::new(vec![
            KeyValue::new("cloud.platform", "gcp_kubernetes_engine"),
            KeyValue::new("cloud.region", "myregion"),
            KeyValue::new("k8s.cluster.name", "mycluster"),
            KeyValue::new("k8s.namespace.name", "myns"),
            KeyValue::new("k8s.pod.name", "mypod"),
        ]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "k8s_pod".to_string(),
            labels: to_labels(json!({
                "cluster_name": "mycluster",
                "location": "myregion",
                "namespace_name": "myns",
                "pod_name": "mypod",
            })),
        };
    }

    #[test]
    fn test_get_monitored_resource_k8s_node() {
        let res = Resource::new(vec![
            KeyValue::new("cloud.platform", "gcp_kubernetes_engine"),
            KeyValue::new("cloud.availability_zone", "myavailzone"),
            KeyValue::new("k8s.cluster.name", "mycluster"),
            KeyValue::new("k8s.namespace.name", "myns"),
            KeyValue::new("k8s.node.name", "mynode"),
        ]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "k8s_node".to_string(),
            labels: to_labels(json!({
                "cluster_name": "mycluster",
                "location": "myavailzone",
                "node_name": "mynode",
            })),
        };
    }

    #[test]
    fn test_get_monitored_resource_k8s_node_region_fallback() {
        let res = Resource::new(vec![
            KeyValue::new("cloud.platform", "gcp_kubernetes_engine"),
            KeyValue::new("cloud.region", "myregion"),
            KeyValue::new("k8s.cluster.name", "mycluster"),
            KeyValue::new("k8s.namespace.name", "myns"),
            KeyValue::new("k8s.node.name", "mynode"),
        ]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "k8s_node".to_string(),
            labels: to_labels(json!({
                "cluster_name": "mycluster",
                "location": "myregion",
                "node_name": "mynode",
            })),
        };
    }

    #[test]
    fn test_get_monitored_resource_k8s_cluster() {
        let res = Resource::new(vec![
            KeyValue::new("cloud.platform", "gcp_kubernetes_engine"),
            KeyValue::new("cloud.availability_zone", "myavailzone"),
            KeyValue::new("k8s.cluster.name", "mycluster"),
            KeyValue::new("k8s.namespace.name", "myns"),
        ]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "k8s_cluster".to_string(),
            labels: to_labels(json!({
                "cluster_name": "mycluster",
                "location": "myavailzone",
            })),
        };
    }

    #[test]
    fn test_get_monitored_resource_k8s_cluster_region_fallback() {
        let res = Resource::new(vec![
            KeyValue::new("cloud.platform", "gcp_kubernetes_engine"),
            KeyValue::new("cloud.region", "myregion"),
            KeyValue::new("k8s.cluster.name", "mycluster"),
            KeyValue::new("k8s.namespace.name", "myns"),
        ]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "k8s_cluster".to_string(),
            labels: to_labels(json!({
                "cluster_name": "mycluster",
                "location": "myregion",
            })),
        };
    }

    #[test]
    fn test_get_monitored_resource_aws_ec2() {
        let res = Resource::new(vec![
            KeyValue::new("cloud.platform", "aws_ec2"),
            KeyValue::new("cloud.availability_zone", "myavailzone"),
            KeyValue::new("host.id", "myhostid"),
            KeyValue::new("cloud.account.id", "myawsaccount"),
        ]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "aws_ec2_instance".to_string(),
            labels: to_labels(json!({
                "aws_account": "myawsaccount",
                "instance_id": "myhostid",
                "region": "myavailzone",
            })),
        };
    }

    #[test]
    fn test_get_monitored_resource_aws_ec2_region_fallback() {
        let res = Resource::new(vec![
            KeyValue::new("cloud.platform", "aws_ec2"),
            KeyValue::new("cloud.region", "myregion"),
            KeyValue::new("host.id", "myhostid"),
            KeyValue::new("cloud.account.id", "myawsaccount"),
        ]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "aws_ec2_instance".to_string(),
            labels: to_labels(json!({
                "aws_account": "myawsaccount",
                "instance_id": "myhostid",
                "region": "myregion",
            })),
        };
    }

    #[test]
    fn test_get_monitored_resource_generic_task() {
        let res = Resource::new(vec![
            KeyValue::new("cloud.availability_zone", "myavailzone"),
            KeyValue::new("service.namespace", "servicens"),
            KeyValue::new("service.name", "servicename"),
            KeyValue::new("service.instance.id", "serviceinstanceid"),
        ]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "generic_task".to_string(),
            labels: to_labels(json!({
                "job": "servicename",
                "location": "myavailzone",
                "namespace": "servicens",
                "task_id": "serviceinstanceid",
            })),
        };
    }

    #[test]
    fn test_get_monitored_resource_generic_task_fallback_region() {
        let res = Resource::new(vec![
            KeyValue::new("cloud.region", "myregion"),
            KeyValue::new("service.namespace", "servicens"),
            KeyValue::new("service.name", "servicename"),
            KeyValue::new("service.instance.id", "serviceinstanceid"),
        ]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "generic_task".to_string(),
            labels: to_labels(json!({
                "job": "servicename",
                "location": "myregion",
                "namespace": "servicens",
                "task_id": "serviceinstanceid",
            })),
        };
    }

    #[test]
    fn test_get_monitored_resource_generic_task_fallback_global() {
        let res = Resource::new(vec![
            KeyValue::new("service.namespace", "servicens"),
            KeyValue::new("service.name", "servicename"),
            KeyValue::new("service.instance.id", "serviceinstanceid"),
        ]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "generic_task".to_string(),
            labels: to_labels(json!({
                "job": "servicename",
                "location": "global",
                "namespace": "servicens",
                "task_id": "serviceinstanceid",
            })),
        };
    }

    #[test]
    fn test_get_monitored_resource_generic_task_faas() {
        let res = Resource::new(vec![
            KeyValue::new("service.name", "unknown_service"),
            KeyValue::new("cloud.region", "myregion"),
            KeyValue::new("service.namespace", "servicens"),
            KeyValue::new("faas.name", "faasname"),
            KeyValue::new("faas.instance", "faasinstance"),
        ]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "generic_task".to_string(),
            labels: to_labels(json!({
                "job": "faasname",
                "location": "myregion",
                "namespace": "servicens",
                "task_id": "faasinstance",
            })),
        };
    }

    #[test]
    fn test_get_monitored_resource_generic_task_faas_fallback() {
        let res = Resource::new(vec![
            KeyValue::new("service.name", "unknown_service"),
            KeyValue::new("cloud.region", "myregion"),
            KeyValue::new("service.namespace", "servicens"),
            KeyValue::new("faas.instance", "faasinstance"),
        ]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "generic_task".to_string(),
            labels: to_labels(json!({
                "job": "unknown_service",
                "location": "myregion",
                "namespace": "servicens",
                "task_id": "faasinstance",
            })),
        };
    }

    #[test]
    fn test_get_monitored_resource_generic_node() {
        let res = Resource::new(vec![
            KeyValue::new("cloud.availability_zone", "myavailzone"),
            KeyValue::new("service.namespace", "servicens"),
            KeyValue::new("service.name", "servicename"),
            KeyValue::new("host.id", "hostid"),
        ]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "generic_node".to_string(),
            labels: to_labels(json!({
                "location": "myavailzone",
                "namespace": "servicens",
                "node_id": "hostid",
            })),
        };
    }

    #[test]
    fn test_get_monitored_resource_generic_node_fallback_region() {
        let res = Resource::new(vec![
            KeyValue::new("cloud.region", "myregion"),
            KeyValue::new("service.namespace", "servicens"),
            KeyValue::new("service.name", "servicename"),
            KeyValue::new("host.id", "hostid"),
        ]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "generic_node".to_string(),
            labels: to_labels(json!({
                "location": "myregion",
                "namespace": "servicens",
                "node_id": "hostid",
            })),
        };
    }

    #[test]
    fn test_get_monitored_resource_generic_node_fallback_global() {
        let res = Resource::new(vec![
            KeyValue::new("service.namespace", "servicens"),
            KeyValue::new("service.name", "servicename"),
            KeyValue::new("host.id", "hostid"),
        ]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "generic_node".to_string(),
            labels: to_labels(json!({
                "location": "global",
                "namespace": "servicens",
                "node_id": "hostid",
            })),
        };
    }

    #[test]
    fn test_get_monitored_resource_generic_node_fallback_host_name() {
        let res = Resource::new(vec![
            KeyValue::new("service.namespace", "servicens"),
            KeyValue::new("service.name", "servicename"),
            KeyValue::new("host.id", "hostid"),
        ]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "generic_node".to_string(),
            labels: to_labels(json!({
                "location": "global",
                "namespace": "servicens",
                "node_id": "hostname",
            })),
        };
    }

    #[test]
    fn test_get_monitored_resource_fallback_generic_node() {
        let res = Resource::new(vec![
            KeyValue::new("foo", "bar"),
            KeyValue::new("no.useful", "resourceattribs"),
        ]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "generic_node".to_string(),
            labels: to_labels(json!({
                "location": "global",
                "namespace": "",
                "node_id": "",
            })),
        };
    }

    #[test]
    fn test_get_monitored_resource_empty() {
        let res = Resource::new(vec![]);
        let monitored_resource = get_monitored_resource(res);
        assert!(monitored_resource.is_some());
        let export_monitored_resource = MonitoredResourceData {
            r#type: "generic_node".to_string(),
            labels: to_labels(json!({
                "location": "global",
                "namespace": "",
                "node_id": "",
            })),
        };
    }

    #[test]
    fn test_non_string_values() {
        fn non_string_expect<V: Into<opentelemetry::Value>>(value: V, expect: Option<String>) {
            let res = Resource::new(vec![KeyValue::new("host.id", value)]);
            let monitored_resource = get_monitored_resource(res);
            assert!(monitored_resource.is_some());
            if let Some(monitored_resource) = monitored_resource {
                assert_eq!(monitored_resource.labels.get("node_id"), expect.as_ref());
            }
        }
        
        non_string_expect(123, Some("123".to_string()));
        non_string_expect(123.4, Some("123.4".to_string()));
        non_string_expect(opentelemetry::Value::Array(vec![1, 2, 3, 4].into()), Some("[1,2,3,4]".to_string()));
        non_string_expect(opentelemetry::Value::Array(vec![1.1, 2.2, 3.3, 4.4].into()), Some("[1.1,2.2,3.3,4.4]".to_string()));
        non_string_expect(opentelemetry::Value::Array(vec![StringValue::from("a"), StringValue::from("b"), StringValue::from("c"), StringValue::from("d")].into()), Some("[\"a\",\"b\",\"c\",\"d\"]".to_string()));
    }


}