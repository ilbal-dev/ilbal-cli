use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectConfig {
    pub name: String,
    pub description: String,
    pub authors: Vec<String>,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeType {
    Docker,
    Podman,
}

impl std::fmt::Display for RuntimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeType::Docker => write!(f, "Docker"),
            RuntimeType::Podman => write!(f, "Podman"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RuntimeConfig {
    pub runtime: RuntimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostgresqlConfig {
    pub pg_username: String,
    pub pg_password: String,
    pub pg_uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PgadminConfig {
    pub pgadmin_email: String,
    pub pgadmin_password: String,
    pub pgadmin_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeaweedfsConfig {
    pub seaweedfs_s3_url: String,
    pub seaweedfs_master_url: String,
    pub seaweedfs_volume_url: String,
    pub seaweedfs_filer_url: String,
    pub seaweedfs_webdav_url: String,
    pub seaweedfs_admin_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MartinConfig {
    pub martin_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SequinConfig {
    pub sequin_url: String,
    pub sequin_email: String,
    pub sequin_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PgdogConfig {
    pub pgdog_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RedisConfig {
    pub redis_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub project: ProjectConfig,
    pub runtime: RuntimeConfig,
    pub postgresql: PostgresqlConfig,
    pub pgadmin: PgadminConfig,
    pub martin: MartinConfig,
    pub seaweedfs: SeaweedfsConfig,
    pub sequin: SequinConfig,
    pub pgdog: PgdogConfig,
    pub redis: RedisConfig,
}
