pub struct ResourceAttributes;

impl ResourceAttributes {
    pub const AWS_EC2: &'static str = "aws_ec2";
    pub const CLOUD_ACCOUNT_ID: &'static str = "cloud.account.id";
    pub const CLOUD_AVAILABILITY_ZONE: &'static str = "cloud.availability_zone";
    pub const CLOUD_PLATFORM_KEY: &'static str = "cloud.platform";
    pub const CLOUD_PROVIDER: &'static str = "cloud.provider";
    pub const CLOUD_REGION: &'static str = "cloud.region";
    pub const GCP_COMPUTE_ENGINE: &'static str = "gcp_compute_engine";
    pub const GCP_KUBERNETES_ENGINE: &'static str = "gcp_kubernetes_engine";
    pub const HOST_ID: &'static str = "host.id";
    pub const HOST_NAME: &'static str = "host.name";
    pub const HOST_TYPE: &'static str = "host.type";
    pub const K8S_CLUSTER_NAME: &'static str = "k8s.cluster.name";
    pub const K8S_CONTAINER_NAME: &'static str = "k8s.container.name";
    pub const K8S_NAMESPACE_NAME: &'static str = "k8s.namespace.name";
    pub const K8S_NODE_NAME: &'static str = "k8s.node.name";
    pub const K8S_POD_NAME: &'static str = "k8s.pod.name";
    pub const SERVICE_INSTANCE_ID: &'static str = "service.instance.id";
    pub const SERVICE_NAME: &'static str = "service.name";
    pub const SERVICE_NAMESPACE: &'static str = "service.namespace";
    pub const FAAS_INSTANCE: &'static str = "faas.instance";
    pub const FAAS_NAME: &'static str = "faas.name";
}

pub const AWS_ACCOUNT: &str = "aws_account";
pub const AWS_EC2_INSTANCE: &str = "aws_ec2_instance";
pub const CLUSTER_NAME: &str = "cluster_name";
pub const CONTAINER_NAME: &str = "container_name";
pub const GCE_INSTANCE: &str = "gce_instance";
pub const GENERIC_NODE: &str = "generic_node";
pub const GENERIC_TASK: &str = "generic_task";
pub const INSTANCE_ID: &str = "instance_id";
pub const JOB: &str = "job";
pub const K8S_CLUSTER: &str = "k8s_cluster";
pub const K8S_CONTAINER: &str = "k8s_container";
pub const K8S_NODE: &str = "k8s_node";
pub const K8S_POD: &str = "k8s_pod";
pub const LOCATION: &str = "location";
pub const NAMESPACE: &str = "namespace";
pub const NAMESPACE_NAME: &str = "namespace_name";
pub const NODE_ID: &str = "node_id";
pub const NODE_NAME: &str = "node_name";
pub const POD_NAME: &str = "pod_name";
pub const REGION: &str = "region";
pub const TASK_ID: &str = "task_id";
pub const ZONE: &str = "zone";
pub const UNKNOWN_SERVICE_PREFIX: &str = "unknown_service";
