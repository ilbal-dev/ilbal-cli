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

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub project: ProjectConfig,
    pub runtime: RuntimeConfig,
    pub postgresql: PostgresqlConfig,
    pub pgadmin: PgadminConfig,
}
