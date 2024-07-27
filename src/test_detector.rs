
use crate::error::OpenTelemetryError;
use opentelemetry::KeyValue;
use opentelemetry_sdk::Resource;
use once_cell::sync::Lazy;
use std::sync::Mutex;
static THE_RESOURCE: Lazy<Mutex<()>> = Lazy::new(Mutex::default);


#[cfg(test)]
mod gce_gke_tests {

    use super::*;
    use once_cell::sync::Lazy;
    // use pretty_assertions::{assert_eq, assert_ne};
    use pretty_assertions_sorted::{assert_eq, assert_eq_sorted};
    use regex::Regex;
    use crate::{get_gce_resources, get_gke_resources, test_envs::TestEnvs};


    #[test]
    fn test_get_gce_resources() {
        let metadata = serde_json::json!({
            "project": {"projectId": "fakeProject"},
            "instance": {
                "name": "fakeName",
                "id": "fakeId",
                "machineType": "fakeMachineType",
                "zone": "projects/233510669999/zones/us-east4-b",
            },
        });
        let resources = get_gce_resources(&metadata).unwrap();
        let res_default = Resource::default();
        let res = Resource::new(resources);
        // let res = res.merge(&res_default);
        // res.iter().for_each(|kv| println!("{:?} : {:?}", kv.0, kv.1));
        let res_sould_be = Resource::new(vec![
            KeyValue::new("cloud.account.id", "fakeProject".to_string()),
            KeyValue::new("cloud.availability_zone", "us-east4-b".to_string()),
            KeyValue::new("cloud.platform", "gcp_compute_engine"),
            KeyValue::new("cloud.provider", "gcp"),
            KeyValue::new("cloud.region", "us-east4".to_string()),
            KeyValue::new("host.id", "fakeId".to_string()),
            KeyValue::new("host.name", "fakeName".to_string()),
            KeyValue::new("host.type", "fakeMachineType".to_string()),
            // KeyValue::new("service.name", "unknown_service"),
            // KeyValue::new("telemetry.sdk.language", "rust"),
            // KeyValue::new("telemetry.sdk.name", "opentelemetry"),
            // KeyValue::new("telemetry.sdk.version", "0.23.0"),
        ]);
        assert_eq_sorted!(res, res_sould_be);
    }

    #[test]
    fn test_get_gke_resources_regional() {
        let resources = {  
            let _m = THE_RESOURCE.lock().unwrap();
            let _e = TestEnvs::new(vec!["KUBERNETES_SERVICE_HOST","POD_NAME","HOSTNAME","NAMESPACE"]);

            TestEnvs::set_var("KUBERNETES_SERVICE_HOST", "10.0.0.1");
            TestEnvs::set_var("NAMESPACE", "namespace");
            TestEnvs::set_var("HOSTNAME", "host_name");
            TestEnvs::set_var("POD_NAME", "pod_name");

            let metadata = serde_json::json!({
                "instance": {
                    "id": "instance_id",
                    "zone": "projects/233510669999/zones/us-east4-b",
                    "attributes": {
                        "cluster-name": "cluster_name",
                        "cluster-location": "us-east4",
                    },
                },
                "project": {"projectId": "project_id"},
            });
            get_gke_resources(&metadata).unwrap()
        };
        let res_default = Resource::default();
        let res = Resource::new(resources);
        // let res = res.merge(&res_default);
        // res.iter().for_each(|kv| println!("{:?} : {:?}", kv.0, kv.1));
        
        let res_sould_be = Resource::new(vec![
            KeyValue::new("cloud.account.id", "project_id".to_string()),
            KeyValue::new("k8s.cluster.name", "cluster_name".to_string()),
            KeyValue::new("k8s.namespace.name", "namespace".to_string()),
            KeyValue::new("host.id", "instance_id".to_string()),
            KeyValue::new("k8s.pod.name", "pod_name".to_string()),
            KeyValue::new("cloud.region", "us-east4".to_string()),
            KeyValue::new("cloud.zone", "us-east4-b".to_string()),
            KeyValue::new("cloud.platform", "gcp_kubernetes_engine"),
            KeyValue::new("cloud.provider", "gcp"),
            KeyValue::new("gcp.resource_type", "gke_container"),
        ]);
        assert_eq_sorted!(res, res_sould_be);
    }

    #[test]
    fn test_get_gke_resources_zone() {
        let resources = {  
            let _m = THE_RESOURCE.lock().unwrap();
            let _e = TestEnvs::new(vec!["KUBERNETES_SERVICE_HOST","POD_NAME","HOSTNAME","NAMESPACE"]);

            TestEnvs::set_var("KUBERNETES_SERVICE_HOST", "10.0.0.1");
            TestEnvs::set_var("NAMESPACE", "namespace");
            TestEnvs::set_var("HOSTNAME", "host_name");
            TestEnvs::set_var("POD_NAME", "pod_name");

            let metadata = serde_json::json!({
                "instance": {
                    "id": "instance_id",
                    "zone": "projects/233510669999/zones/us-east4-b",
                    "attributes": {
                        "cluster-name": "cluster_name",
                        "cluster-location": "us-east4-b",
                    },
                },
                "project": {"projectId": "project_id"},
            });
            get_gke_resources(&metadata).unwrap()
        };
        let res_default = Resource::default();
        let res = Resource::new(resources);
        // let res = res.merge(&res_default);
        // res.iter().for_each(|kv| println!("{:?} : {:?}", kv.0, kv.1));
        
        let res_sould_be = Resource::new(vec![
            KeyValue::new("cloud.account.id", "project_id".to_string()),
            KeyValue::new("k8s.cluster.name", "cluster_name".to_string()),
            KeyValue::new("k8s.namespace.name", "namespace".to_string()),
            KeyValue::new("host.id", "instance_id".to_string()),
            KeyValue::new("k8s.pod.name", "pod_name".to_string()),
            KeyValue::new("cloud.availability_zone", "us-east4-b".to_string()),
            KeyValue::new("cloud.platform", "gcp_kubernetes_engine"),
            KeyValue::new("cloud.provider", "gcp"),
            KeyValue::new("cloud.zone", "us-east4-b".to_string()),
            KeyValue::new("gcp.resource_type", "gke_container"),
        ]);
        assert_eq_sorted!(res, res_sould_be);
    }
}

// @patch_env
// @mock.patch(
//     "opentelemetry.resourcedetector.gcp_resource_detector.requests.get",
//     **{"return_value.json.return_value": GKE_RESOURCES_JSON_STRING}
// )
// class TestGKEResourceFinder(unittest.TestCase):

//     # pylint: disable=unused-argument
//     def test_not_running_on_gke(self, getter):
//         pop_environ_key(KUBERNETES_SERVICE_HOST)
//         found_resources = get_gke_resources()
//         self.assertEqual(found_resources, {})

//     # pylint: disable=unused-argument
//     def test_missing_container_name(self, getter):
//         os.environ[KUBERNETES_SERVICE_HOST] = "10.0.0.1"
//         pop_environ_key(CONTAINER_NAME)
//         found_resources = get_gke_resources()
//         self.assertEqual(
//             found_resources,
//             {
//                 "cloud.account.id": "project_id",
//                 "k8s.cluster.name": "cluster_name",
//                 "k8s.namespace.name": "",
//                 "host.id": "instance_id",
//                 "k8s.pod.name": "",
//                 "cloud.zone": "zone",
//                 "cloud.provider": "gcp",
//                 "gcp.resource_type": "gke_container",
//             },
//         )

//     # pylint: disable=unused-argument
//     def test_environment_empty_strings(self, getter):
//         os.environ[KUBERNETES_SERVICE_HOST] = "10.0.0.1"
//         os.environ[CONTAINER_NAME] = ""
//         os.environ[NAMESPACE] = ""
//         found_resources = get_gke_resources()
//         self.assertEqual(
//             found_resources,
//             {
//                 "cloud.account.id": "project_id",
//                 "k8s.cluster.name": "cluster_name",
//                 "k8s.namespace.name": "",
//                 "host.id": "instance_id",
//                 "k8s.pod.name": "",
//                 "container.name": "",
//                 "cloud.zone": "zone",
//                 "cloud.provider": "gcp",
//                 "gcp.resource_type": "gke_container",
//             },
//         )

//     def test_missing_namespace_file(self, getter):
//         os.environ[KUBERNETES_SERVICE_HOST] = "10.0.0.1"
//         os.environ[CONTAINER_NAME] = "container_name"
//         found_resources = get_gke_resources()
//         self.assertEqual(
//             found_resources,
//             {
//                 "cloud.account.id": "project_id",
//                 "k8s.cluster.name": "cluster_name",
//                 "k8s.namespace.name": "",
//                 "host.id": "instance_id",
//                 "k8s.pod.name": "",
//                 "container.name": "container_name",
//                 "cloud.zone": "zone",
//                 "cloud.provider": "gcp",
//                 "gcp.resource_type": "gke_container",
//             },
//         )

//     def test_finding_gke_resources(self, getter):
//         os.environ[KUBERNETES_SERVICE_HOST] = "10.0.0.1"
//         os.environ[NAMESPACE] = "namespace"
//         os.environ[CONTAINER_NAME] = "container_name"
//         os.environ[HOSTNAME] = "host_name"
//         found_resources = get_gke_resources()
//         self.assertEqual(getter.call_args_list[0][0][0], _GCP_METADATA_URL)
//         self.assertEqual(
//             found_resources,
//             {
//                 "cloud.account.id": "project_id",
//                 "k8s.cluster.name": "cluster_name",
//                 "k8s.namespace.name": "namespace",
//                 "host.id": "instance_id",
//                 "k8s.pod.name": "host_name",
//                 "container.name": "container_name",
//                 "cloud.zone": "zone",
//                 "cloud.provider": "gcp",
//                 "gcp.resource_type": "gke_container",
//             },
//         )

//     def test_finding_gke_resources_with_pod_name(self, getter):
//         os.environ[KUBERNETES_SERVICE_HOST] = "10.0.0.1"
//         os.environ[NAMESPACE] = "namespace"
//         os.environ[CONTAINER_NAME] = "container_name"
//         os.environ[HOSTNAME] = "host_name"
//         os.environ[POD_NAME] = "pod_name"
//         found_resources = get_gke_resources()
//         self.assertEqual(getter.call_args_list[0][0][0], _GCP_METADATA_URL)
//         self.assertEqual(
//             found_resources,
//             {
//                 "cloud.account.id": "project_id",
//                 "k8s.cluster.name": "cluster_name",
//                 "k8s.namespace.name": "namespace",
//                 "host.id": "instance_id",
//                 "k8s.pod.name": "pod_name",
//                 "container.name": "container_name",
//                 "cloud.zone": "zone",
//                 "cloud.provider": "gcp",
//                 "gcp.resource_type": "gke_container",
//             },
//         )

#[cfg(test)]
mod gke_tests {

    use std::{fs, io::Write, path::Path};

    use super::*;
    use once_cell::sync::Lazy;
    // use pretty_assertions::{assert_eq, assert_ne};
    use pretty_assertions_sorted::{assert_eq, assert_eq_sorted};
    use regex::Regex;
    use crate::{get_gke_resources, test_envs::TestEnvs};
    
    static GKE_RESOURCES_JSON_STRING: Lazy<serde_json::Value> = Lazy::new(|| {
        serde_json::json!({
            "instance": {
                "id": "instance_id",
                "zone": "projects/123/zones/zone",
                "attributes": {"cluster-name": "cluster_name"},
            },
            "project": {"projectId": "project_id"},
        })
    });

    #[test]
    fn test_not_running_on_cloudrun() {
        let _m = THE_RESOURCE.lock().unwrap();
        let metadata = serde_json::json!({});
        TestEnvs::remove_var("KUBERNETES_SERVICE_HOST");
        let resources = get_gke_resources(&metadata);
        assert_eq!(resources.is_err(), true);
    }

    #[test]
    fn test_missing_container_name() {
        let resources = {  
            let _m = THE_RESOURCE.lock().unwrap();
            let _e = TestEnvs::new(vec!["KUBERNETES_SERVICE_HOST","CONTAINER_NAME"]);

            TestEnvs::set_var("KUBERNETES_SERVICE_HOST", "10.0.0.1");
            TestEnvs::remove_var("CONTAINER_NAME");

            let metadata = GKE_RESOURCES_JSON_STRING.clone();
            get_gke_resources(&metadata).unwrap()
        };
        let res_default = Resource::default();
        let res = Resource::new(resources);
        // let res = res.merge(&res_default);
        // res.iter().for_each(|kv| println!("{:?} : {:?}", kv.0, kv.1));
        
        let res_sould_be = Resource::new(vec![
            KeyValue::new("cloud.account.id", "project_id".to_string()),
            KeyValue::new("k8s.cluster.name", "cluster_name".to_string()),
            KeyValue::new("k8s.namespace.name", "".to_string()),
            KeyValue::new("host.id", "instance_id".to_string()),
            KeyValue::new("k8s.pod.name", "".to_string()),
            KeyValue::new("cloud.zone", "zone".to_string()),
            KeyValue::new("cloud.provider", "gcp"),
            KeyValue::new("cloud.platform", "gcp_kubernetes_engine"),
            KeyValue::new("gcp.resource_type", "gke_container"),
        ]);
        assert_eq_sorted!(res, res_sould_be);
    }

    #[test]
    fn test_environment_empty_strings() {
        let resources = {  
            let _m = THE_RESOURCE.lock().unwrap();
            let _e = TestEnvs::new(vec!["KUBERNETES_SERVICE_HOST","CONTAINER_NAME", "NAMESPACE"]);

            TestEnvs::set_var("KUBERNETES_SERVICE_HOST", "10.0.0.1");
            TestEnvs::set_var("CONTAINER_NAME", "");
            TestEnvs::set_var("NAMESPACE", "");

            let metadata = GKE_RESOURCES_JSON_STRING.clone();
            get_gke_resources(&metadata).unwrap()
        };
        let res_default = Resource::default();
        let res = Resource::new(resources);
        // let res = res.merge(&res_default);
        // res.iter().for_each(|kv| println!("{:?} : {:?}", kv.0, kv.1));
        
        let res_sould_be = Resource::new(vec![
            KeyValue::new("cloud.account.id", "project_id".to_string()),
            KeyValue::new("k8s.cluster.name", "cluster_name".to_string()),
            KeyValue::new("k8s.namespace.name", "".to_string()),
            KeyValue::new("host.id", "instance_id".to_string()),
            KeyValue::new("k8s.pod.name", "".to_string()),
            KeyValue::new("container.name", "".to_string()),
            KeyValue::new("cloud.zone", "zone".to_string()),
            KeyValue::new("cloud.provider", "gcp"),
            KeyValue::new("cloud.platform", "gcp_kubernetes_engine"),
            KeyValue::new("gcp.resource_type", "gke_container"),
        ]);
        assert_eq_sorted!(res, res_sould_be);
    }

    #[test]
    fn test_missing_namespace_file() {
        let resources = {  
            let _m = THE_RESOURCE.lock().unwrap();
            let _e = TestEnvs::new(vec!["KUBERNETES_SERVICE_HOST","CONTAINER_NAME", "NAMESPACE"]);

            TestEnvs::set_var("KUBERNETES_SERVICE_HOST", "10.0.0.1");
            TestEnvs::set_var("CONTAINER_NAME", "container_name");

            let metadata = GKE_RESOURCES_JSON_STRING.clone();
            get_gke_resources(&metadata).unwrap()
        };
        let res_default = Resource::default();
        let res = Resource::new(resources);
        // let res = res.merge(&res_default);
        // res.iter().for_each(|kv| println!("{:?} : {:?}", kv.0, kv.1));
        
        let res_sould_be = Resource::new(vec![
            KeyValue::new("cloud.account.id", "project_id".to_string()),
            KeyValue::new("k8s.cluster.name", "cluster_name".to_string()),
            KeyValue::new("k8s.namespace.name", "".to_string()),
            KeyValue::new("host.id", "instance_id".to_string()),
            KeyValue::new("k8s.pod.name", "".to_string()),
            KeyValue::new("container.name", "container_name".to_string()),
            KeyValue::new("cloud.zone", "zone".to_string()),
            KeyValue::new("cloud.provider", "gcp"),
            KeyValue::new("cloud.platform", "gcp_kubernetes_engine"),
            KeyValue::new("gcp.resource_type", "gke_container"),
        ]);
        assert_eq_sorted!(res, res_sould_be);
    }

    #[test]
    #[ignore]
    fn test_exist_namespace_file() {
        // sudo cargo test -- test_detector::gke_tests::test_exist_namespace_file
        let dir_path = Path::new("/var/run/secrets/kubernetes.io/serviceaccount");
        let file_path = dir_path.join("namespace");

        fs::create_dir_all(&dir_path).unwrap();
    
        // Create and write to the file
        let mut file = fs::File::create(file_path).unwrap();
        file.write_all(b"namespace").unwrap();
        let resources = {  
            let _m = THE_RESOURCE.lock().unwrap();
            let _e = TestEnvs::new(vec!["KUBERNETES_SERVICE_HOST","CONTAINER_NAME", "NAMESPACE"]);

            TestEnvs::set_var("KUBERNETES_SERVICE_HOST", "10.0.0.1");
            TestEnvs::set_var("CONTAINER_NAME", "container_name");

            let metadata = GKE_RESOURCES_JSON_STRING.clone();
            get_gke_resources(&metadata).unwrap()
        };
        fs::remove_file("/var/run/secrets/kubernetes.io/serviceaccount/namespace").unwrap();
        let res_default = Resource::default();
        let res = Resource::new(resources);
        // let res = res.merge(&res_default);
        // res.iter().for_each(|kv| println!("{:?} : {:?}", kv.0, kv.1));
        
        let res_sould_be = Resource::new(vec![
            KeyValue::new("cloud.account.id", "project_id".to_string()),
            KeyValue::new("k8s.cluster.name", "cluster_name".to_string()),
            KeyValue::new("k8s.namespace.name", "namespace".to_string()),
            KeyValue::new("host.id", "instance_id".to_string()),
            KeyValue::new("k8s.pod.name", "".to_string()),
            KeyValue::new("container.name", "container_name".to_string()),
            KeyValue::new("cloud.zone", "zone".to_string()),
            KeyValue::new("cloud.provider", "gcp"),
            KeyValue::new("cloud.platform", "gcp_kubernetes_engine"),
            KeyValue::new("gcp.resource_type", "gke_container"),
        ]);
        assert_eq_sorted!(res, res_sould_be);
    }

    #[test]
    fn test_finding_gke_resources() {
        let resources = {  
            let _m = THE_RESOURCE.lock().unwrap();
            let _e = TestEnvs::new(vec!["KUBERNETES_SERVICE_HOST","CONTAINER_NAME", "NAMESPACE", "HOSTNAME"]);

            TestEnvs::set_var("KUBERNETES_SERVICE_HOST", "10.0.0.1");
            TestEnvs::set_var("NAMESPACE", "namespace");
            TestEnvs::set_var("CONTAINER_NAME", "container_name");
            TestEnvs::set_var("HOSTNAME", "host_name");

            let metadata = GKE_RESOURCES_JSON_STRING.clone();
            get_gke_resources(&metadata).unwrap()
        };
        let res_default = Resource::default();
        let res = Resource::new(resources);
        // let res = res.merge(&res_default);
        // res.iter().for_each(|kv| println!("{:?} : {:?}", kv.0, kv.1));
        
        let res_sould_be = Resource::new(vec![
            KeyValue::new("cloud.account.id", "project_id".to_string()),
            KeyValue::new("k8s.cluster.name", "cluster_name".to_string()),
            KeyValue::new("k8s.namespace.name", "namespace".to_string()),
            KeyValue::new("host.id", "instance_id".to_string()),
            KeyValue::new("k8s.pod.name", "host_name".to_string()),
            KeyValue::new("container.name", "container_name".to_string()),
            KeyValue::new("cloud.zone", "zone".to_string()),
            KeyValue::new("cloud.provider", "gcp"),
            KeyValue::new("cloud.platform", "gcp_kubernetes_engine"),
            KeyValue::new("gcp.resource_type", "gke_container"),
        ]);
        assert_eq_sorted!(res, res_sould_be);
    }

    #[test]
    fn test_finding_gke_resources_with_pod_name() {
        let resources = {  
            let _m = THE_RESOURCE.lock().unwrap();
            let _e = TestEnvs::new(vec!["KUBERNETES_SERVICE_HOST","CONTAINER_NAME", "NAMESPACE", "HOSTNAME", "POD_NAME"]);

            TestEnvs::set_var("KUBERNETES_SERVICE_HOST", "10.0.0.1");
            TestEnvs::set_var("NAMESPACE", "namespace");
            TestEnvs::set_var("CONTAINER_NAME", "container_name");
            TestEnvs::set_var("HOSTNAME", "host_name");
            TestEnvs::set_var("POD_NAME", "pod_name");

            let metadata = GKE_RESOURCES_JSON_STRING.clone();
            get_gke_resources(&metadata).unwrap()
        };
        let res_default = Resource::default();
        let res = Resource::new(resources);
        // let res = res.merge(&res_default);
        // res.iter().for_each(|kv| println!("{:?} : {:?}", kv.0, kv.1));
        
        let res_sould_be = Resource::new(vec![
            KeyValue::new("cloud.account.id", "project_id".to_string()),
            KeyValue::new("k8s.cluster.name", "cluster_name".to_string()),
            KeyValue::new("k8s.namespace.name", "namespace".to_string()),
            KeyValue::new("host.id", "instance_id".to_string()),
            KeyValue::new("k8s.pod.name", "pod_name".to_string()),
            KeyValue::new("container.name", "container_name".to_string()),
            KeyValue::new("cloud.zone", "zone".to_string()),
            KeyValue::new("cloud.provider", "gcp"),
            KeyValue::new("cloud.platform", "gcp_kubernetes_engine"),
            KeyValue::new("gcp.resource_type", "gke_container"),
        ]);
        assert_eq_sorted!(res, res_sould_be);
    }

}

#[cfg(test)]
mod cloudfunctions_tests {

    use super::*;
    use once_cell::sync::Lazy;
    // use pretty_assertions::{assert_eq, assert_ne};
    use pretty_assertions_sorted::{assert_eq, assert_eq_sorted};
    use regex::Regex;
    use crate::{get_cloudfunctions_resources, get_cloudrun_resources, test_envs::TestEnvs};
    
    static CLOUDFUNCTIONS_RESOURCES_JSON_STRING: Lazy<serde_json::Value> = Lazy::new(|| {
        serde_json::json!({
            "instance": {
                "id": "instance_id",
                "zone": "projects/123/zones/zone",
                "region": "projects/123/regions/region",
            },
            "project": {"projectId": "project_id"},
        })
    });

    #[test]
    fn test_not_running_on_cloudrun() {
        let _m = THE_RESOURCE.lock().unwrap();
        let metadata = serde_json::json!({});
        let resources = get_cloudrun_resources(&metadata);
        assert_eq!(resources.is_err(), true);
    }

    #[test]
    fn test_missing_service_name() {
        let resources = {  
            let _m = THE_RESOURCE.lock().unwrap();
            let _e = TestEnvs::new(vec!["FUNCTION_TARGET","K_SERVICE","K_REVISION"]);

            TestEnvs::set_var("FUNCTION_TARGET", "function");
            TestEnvs::remove_var("K_SERVICE");
            TestEnvs::remove_var("K_REVISION");

            let metadata = CLOUDFUNCTIONS_RESOURCES_JSON_STRING.clone();
            get_cloudfunctions_resources(&metadata).unwrap()
        };
        let res_default = Resource::default();
        let res = Resource::new(resources);
        // let res = res.merge(&res_default);
        // res.iter().for_each(|kv| println!("{:?} : {:?}", kv.0, kv.1));
        
        let res_sould_be = Resource::new(vec![
            KeyValue::new("cloud.account.id", "project_id".to_string()),
            KeyValue::new("cloud.platform", "gcp_cloud_functions"),
            KeyValue::new("cloud.region", "region".to_string()),
            KeyValue::new("faas.instance", "instance_id".to_string()),
            KeyValue::new("cloud.zone", "zone".to_string()),
            KeyValue::new("cloud.provider", "gcp"),
            KeyValue::new("gcp.resource_type", "cloud_functions"),
        ]);
        assert_eq_sorted!(res, res_sould_be);
    }

    #[test]
    fn test_environment_empty_strings() {
        let resources = {  
            let _m = THE_RESOURCE.lock().unwrap();
            let _e = TestEnvs::new(vec!["FUNCTION_TARGET","K_SERVICE","K_REVISION"]);

            TestEnvs::set_var("FUNCTION_TARGET", "function");
            TestEnvs::set_var("K_SERVICE","");
            TestEnvs::set_var("K_REVISION","");

            let metadata = CLOUDFUNCTIONS_RESOURCES_JSON_STRING.clone();
            get_cloudfunctions_resources(&metadata).unwrap()
        };
        let res_default = Resource::default();
        let res = Resource::new(resources);
        // let res = res.merge(&res_default);
        // res.iter().for_each(|kv| println!("{:?} : {:?}", kv.0, kv.1));
        
        let res_sould_be = Resource::new(vec![
            KeyValue::new("cloud.account.id", "project_id".to_string()),
            KeyValue::new("cloud.platform", "gcp_cloud_functions"),
            KeyValue::new("cloud.region", "region".to_string()),
            KeyValue::new("faas.instance", "instance_id".to_string()),
            KeyValue::new("faas.name", "".to_string()),
            KeyValue::new("faas.version", "".to_string()),
            KeyValue::new("cloud.zone", "zone".to_string()),
            KeyValue::new("cloud.provider", "gcp"),
            KeyValue::new("gcp.resource_type", "cloud_functions"),
        ]);
        assert_eq_sorted!(res, res_sould_be);
    }

    #[test]
    fn test_finding_cloudfunctions_resources() {
        let resources = {  
            let _m = THE_RESOURCE.lock().unwrap();
            let _e = TestEnvs::new(vec!["FUNCTION_TARGET","K_SERVICE","K_REVISION"]);

            TestEnvs::set_var("FUNCTION_TARGET", "function");
            TestEnvs::set_var("K_SERVICE","service");
            TestEnvs::set_var("K_REVISION","revision");

            let metadata = CLOUDFUNCTIONS_RESOURCES_JSON_STRING.clone();
            get_cloudfunctions_resources(&metadata).unwrap()
        };
        let res_default = Resource::default();
        let res = Resource::new(resources);
        // let res = res.merge(&res_default);
        // res.iter().for_each(|kv| println!("{:?} : {:?}", kv.0, kv.1));
        
        let res_sould_be = Resource::new(vec![
            KeyValue::new("cloud.account.id", "project_id".to_string()),
            KeyValue::new("cloud.platform", "gcp_cloud_functions"),
            KeyValue::new("cloud.region", "region".to_string()),
            KeyValue::new("faas.instance", "instance_id".to_string()),
            KeyValue::new("faas.name", "service".to_string()),
            KeyValue::new("faas.version", "revision".to_string()),
            KeyValue::new("cloud.zone", "zone".to_string()),
            KeyValue::new("cloud.provider", "gcp"),
            KeyValue::new("gcp.resource_type", "cloud_functions"),
        ]);
        assert_eq_sorted!(res, res_sould_be);
    }

}

#[cfg(test)]
mod cloudrun_tests {

    use super::*;
    use once_cell::sync::Lazy;
    // use pretty_assertions::{assert_eq, assert_ne};
    use pretty_assertions_sorted::{assert_eq, assert_eq_sorted};
    use regex::Regex;
    use crate::{get_cloudrun_resources, test_envs::TestEnvs};
    static CLOUDRUN_RESOURCES_JSON_STRING: Lazy<serde_json::Value> = Lazy::new(|| {
        serde_json::json!({
            "instance": {
                "id": "instance_id",
                "zone": "projects/123/zones/zone",
                "region": "projects/123/regions/region",
            },
            "project": {"projectId": "project_id"},
        })
    });

    #[test]
    fn test_not_running_on_cloudrun() {
        let _m = THE_RESOURCE.lock().unwrap();
        let metadata = serde_json::json!({});
        let resources = get_cloudrun_resources(&metadata);
        assert_eq!(resources.is_err(), true);
    }

    #[test]
    fn test_missing_service_name() {
        let resources = {  
            let _m = THE_RESOURCE.lock().unwrap();
            let _e = TestEnvs::new(vec!["K_CONFIGURATION","K_SERVICE","K_REVISION"]);

            TestEnvs::set_var("K_CONFIGURATION", "cloudrun_config");
            TestEnvs::remove_var("K_SERVICE");
            TestEnvs::remove_var("K_REVISION");

            let metadata = CLOUDRUN_RESOURCES_JSON_STRING.clone();
            get_cloudrun_resources(&metadata).unwrap()
        };
        let res_default = Resource::default();
        let res = Resource::new(resources);
        // let res = res.merge(&res_default);
        // res.iter().for_each(|kv| println!("{:?} : {:?}", kv.0, kv.1));
        
        let res_sould_be = Resource::new(vec![
            KeyValue::new("cloud.account.id", "project_id".to_string()),
            KeyValue::new("cloud.platform", "gcp_cloud_run"),
            KeyValue::new("cloud.region", "region".to_string()),
            KeyValue::new("faas.instance", "instance_id".to_string()),
            KeyValue::new("cloud.zone", "zone".to_string()),
            KeyValue::new("cloud.provider", "gcp"),
            KeyValue::new("gcp.resource_type", "cloud_run"),
        ]);
        assert_eq_sorted!(res, res_sould_be);
        
    }

    #[test]
    fn test_environment_empty_strings() {
        let resources = {  
            let _m = THE_RESOURCE.lock().unwrap();
            let _e = TestEnvs::new(vec!["K_CONFIGURATION","K_SERVICE","K_REVISION"]);

            TestEnvs::set_var("K_CONFIGURATION", "cloudrun_config");
            TestEnvs::set_var("K_SERVICE", "");
            TestEnvs::set_var("K_REVISION", "");

            let metadata = CLOUDRUN_RESOURCES_JSON_STRING.clone();
            get_cloudrun_resources(&metadata).unwrap()
        };
        let res_default = Resource::default();
        let res = Resource::new(resources);
        // let res = res.merge(&res_default);
        // res.iter().for_each(|kv| println!("{:?} : {:?}", kv.0, kv.1));
        
        let res_sould_be = Resource::new(vec![
            KeyValue::new("cloud.account.id", "project_id".to_string()),
            KeyValue::new("cloud.platform", "gcp_cloud_run"),
            KeyValue::new("cloud.region", "region".to_string()),
            KeyValue::new("faas.instance", "instance_id".to_string()),
            KeyValue::new("faas.name", "".to_string()),
            KeyValue::new("faas.version", "".to_string()),
            KeyValue::new("cloud.zone", "zone".to_string()),
            KeyValue::new("cloud.provider", "gcp"),
            KeyValue::new("gcp.resource_type", "cloud_run"),
        ]);
        assert_eq_sorted!(res, res_sould_be);
        
    }

    #[test]
    fn test_finding_cloudrun_resources() {
        let resources = {  
            let _m = THE_RESOURCE.lock().unwrap();
            let _e = TestEnvs::new(vec!["K_CONFIGURATION","K_SERVICE","K_REVISION"]);

            TestEnvs::set_var("K_CONFIGURATION", "cloudrun_config");
            TestEnvs::set_var("K_SERVICE", "service");
            TestEnvs::set_var("K_REVISION", "revision");

            let metadata = CLOUDRUN_RESOURCES_JSON_STRING.clone();
            get_cloudrun_resources(&metadata).unwrap()
        };
        let res_default = Resource::default();
        let res = Resource::new(resources);
        // let res = res.merge(&res_default);
        // res.iter().for_each(|kv| println!("{:?} : {:?}", kv.0, kv.1));
        
        let res_sould_be = Resource::new(vec![
            KeyValue::new("cloud.account.id", "project_id".to_string()),
            KeyValue::new("cloud.platform", "gcp_cloud_run"),
            KeyValue::new("cloud.region", "region".to_string()),
            KeyValue::new("faas.instance", "instance_id".to_string()),
            KeyValue::new("faas.name", "service".to_string()),
            KeyValue::new("faas.version", "revision".to_string()),
            KeyValue::new("cloud.zone", "zone".to_string()),
            KeyValue::new("cloud.provider", "gcp"),
            KeyValue::new("gcp.resource_type", "cloud_run"),
        ]);
        assert_eq_sorted!(res, res_sould_be);
        
    }
}
        
