pub fn images(key: &str) -> &'static str {
    match key {
        // "postgresql" => "codeberg.org/ilbal/ilbal-postgresql:latest",
        // "ingest" => "codeberg.org/ilbal/ilbal-ingest:latest",
        "postgresql" => "docker.io/ilbal/ilbal-postgresql:latest",
        "ingest" => "docker.io/ilbal/ilbal-ingest:latest",
        // "dev" => "ilbal-dev:0.1.0", // to contain pgbranch and pgroll
        "pgadmin4" => "docker.io/dpage/pgadmin4:latest",
        "pgdog" => "ghcr.io/pgdogdev/pgdog:latest",
        "sequin" => "docker.io/sequin/sequin:latest",
        "redis" => "docker.io/library/redis:latest",
        _ => panic!("unknown image: {key}"),
    }
}
