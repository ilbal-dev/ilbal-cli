pub fn images(key: &str) -> &'static str {
    match key {
        "postgresql" => "docker.io/ilbal/ilbal-postgresql:latest",
        "ingest" => "docker.io/ilbal/ilbal-ingest:latest",
        // "dev" => "ilbal-dev:0.1.0", // to contain pgbranch and pgroll
        "pgadmin4" => "docker.io/dpage/pgadmin4:latest",
        "pgdog" => "ghcr.io/pgdogdev/pgdog:latest",
        "martin" => "ghcr.io/maplibre/martin:1.13.0",
        "sequin" => "docker.io/sequin/sequin:latest",
        "redis" => "docker.io/library/redis:latest",
        _ => panic!("unknown image: {key}"),
    }
}
