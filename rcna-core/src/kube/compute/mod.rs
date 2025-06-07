#[derive(Debug, Clone)]
pub struct ContainerSpec {
    pub name: String,
    pub image: String,
    pub args: Vec<String>,
    pub env: Vec<EnvVar>,
}

#[derive(Debug, Clone)]
pub struct EnvVar {
    pub name: String,
    pub value: Option<String>,
    pub value_from: Option<EnvVarSource>,
}

#[derive(Debug, Clone)]
pub struct EnvVarSource {
    pub config_map_key_ref: Option<ConfigMapKeySelector>,
}

#[derive(Debug, Clone)]
pub struct ConfigMapKeySelector {
    pub name: String,
    pub key: String,
}
