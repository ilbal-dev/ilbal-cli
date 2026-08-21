use crate::typedefs::{Config, RuntimeType};
use crate::ui::gradient_text::gradient_text;
use console::style;
use std::fs;
use std::process::Command;

fn is_container_running(runtime: &RuntimeType, project: &str, service: &str) -> bool {
    let container_name = format!("{project}-{service}");
    let result = match runtime {
        RuntimeType::Docker => Command::new("docker")
            .args([
                "ps",
                "-f",
                &format!("name={container_name}"),
                "--format",
                "{{.Names}}",
            ])
            .output(),
        RuntimeType::Podman => Command::new("podman")
            .args([
                "ps",
                "-f",
                &format!("name={container_name}"),
                "--format",
                "{{.Names}}",
            ])
            .output(),
    };
    match result {
        Ok(out) => !out.stdout.is_empty(),
        Err(_) => false,
    }
}

fn container_icon(
    runtime: &RuntimeType,
    project: &str,
    service: &str,
) -> console::StyledObject<&'static str> {
    if is_container_running(runtime, project, service) {
        style("✅").green()
    } else {
        style("❌").red()
    }
}

pub fn run() -> anyhow::Result<()> {
    let config_toml = fs::read_to_string("ilbal/config.toml")?;
    let config: Config = toml::from_str(&config_toml)?;

    let postgresql_icon =
        container_icon(&config.runtime.runtime, &config.project.name, "postgresql");
    let pgadmin_icon = container_icon(&config.runtime.runtime, &config.project.name, "pgadmin");
    let martin_icon = container_icon(&config.runtime.runtime, &config.project.name, "martin");
    let seaweedfs_icon = container_icon(&config.runtime.runtime, &config.project.name, "seaweedfs");
    let sequin_icon = container_icon(&config.runtime.runtime, &config.project.name, "sequin");
    let pgdog_icon = container_icon(&config.runtime.runtime, &config.project.name, "pgdog");

    // PROJECT INFORMATION
    println!();
    println!(
        "{}",
        gradient_text("---------------------------------------------")
    );
    println!();

    println!(
        "  {title}\n    {name_label} {name}\n    {desc_label} {desc}\n    {authors_label} {authors}\n    {runtime_label} {runtime}",
        title = gradient_text("Project Information"),
        name_label = gradient_text("Project"),
        name = style(&config.project.name).bold(),
        desc_label = gradient_text("Description"),
        desc = config.project.description,
        authors_label = gradient_text("Authors"),
        authors = config.project.authors.join(", "),
        runtime_label = gradient_text("Runtime"),
        runtime = style(&config.runtime.runtime).bold(),
    );

    println!();
    println!(
        "{}",
        gradient_text("---------------------------------------------")
    );
    println!();

    // POSTGRESQL
    println!(
        "  {postgresql_icon} {title}\n    {user_label} {user}\n    {pass_label} {pass}\n    {uri_label} {uri}",
        title = style("PostgreSQL").bold().underlined(),
        user_label = style("Username").dim(),
        user = style(&config.postgresql.pg_username).bold(),
        pass_label = style("Password").dim(),
        pass = style(&config.postgresql.pg_password).bold(),
        uri_label = style("URI").dim(),
        uri = style(&config.postgresql.pg_uri).cyan(),
    );
    println!();
    println!("---------------------------------------------");
    println!();

    // pgAdmin4
    println!(
        "  {pgadmin_icon} {title}\n    {url_label} {url}\n    {email_label} {email}\n    {pass_label} {pass}",
        title = style("pgAdmin4 (PostgreSQL Admin Tool)")
            .bold()
            .underlined(),
        url_label = style("URL").dim(),
        url = style(&config.pgadmin.pgadmin_url).cyan(),
        email_label = style("Email").dim(),
        email = style(&config.pgadmin.pgadmin_email).bold(),
        pass_label = style("Password").dim(),
        pass = style(&config.pgadmin.pgadmin_password).bold(),
    );
    println!();
    println!("---------------------------------------------");
    println!();

    // Martin
    println!(
        "  {martin_icon} {title}\n    {url_label} {url}\n    {note_label} {note}",
        title = style("Martin (Map Tile Server)").bold().underlined(),
        url_label = style("URL").dim(),
        url = style(&config.martin.martin_url).cyan(),
        note_label = style("Note").dim(),
        note = "Uses authenticator role via pgDog",
    );
    println!();
    println!("---------------------------------------------");
    println!();

    // SeaweedFS
    println!(
        "  {seaweedfs_icon} {title}\n    {s3_label} {s3}\n    {master_label} {master}\n    {volume_label} {volume}\n    {filer_label} {filer}\n    {webdav_label} {webdav}\n    {admin_label} {admin}",
        title = style("SeaweedFS (S3 Object Storage)").bold().underlined(),
        s3_label = style("S3 API").dim(),
        s3 = style(&config.seaweedfs.seaweedfs_s3_url).cyan(),
        master_label = style("Master UI").dim(),
        master = style(&config.seaweedfs.seaweedfs_master_url).cyan(),
        volume_label = style("Volume Server").dim(),
        volume = style(&config.seaweedfs.seaweedfs_volume_url).cyan(),
        filer_label = style("Filer UI").dim(),
        filer = style(&config.seaweedfs.seaweedfs_filer_url).cyan(),
        webdav_label = style("WebDAV").dim(),
        webdav = style(&config.seaweedfs.seaweedfs_webdav_url).cyan(),
        admin_label = style("Admin UI").dim(),
        admin = style(&config.seaweedfs.seaweedfs_admin_url).cyan(),
    );
    println!();
    println!("---------------------------------------------");
    println!();

    // Sequin
    println!(
        "  {sequin_icon} {title}\n    {url_label} {url}\n    {email_label} {email}\n    {pass_label} {pass}\n    {note_label} {note}",
        title = style("Sequin (CDC / Event Streaming)").bold().underlined(),
        url_label = style("URL").dim(),
        url = style(&config.sequin.sequin_url).cyan(),
        email_label = style("Email").dim(),
        email = style(&config.sequin.sequin_email).bold(),
        pass_label = style("Password").dim(),
        pass = style(&config.sequin.sequin_password).bold(),
        note_label = style("Note").dim(),
        note = "Connects to PostgreSQL directly",
    );
    println!();
    println!("---------------------------------------------");
    println!();

    // pgDog
    println!(
        "  {pgdog_icon} {title}\n    {url_label} {url}\n    {note_label} {note}",
        title = style("pgDog (Connection Pooler)").bold().underlined(),
        url_label = style("URL").dim(),
        url = style(&config.pgdog.pgdog_url).cyan(),
        note_label = style("Note").dim(),
        note = "Routes connections to PostgreSQL",
    );
    println!();
    println!("---------------------------------------------");
    println!();

    Ok(())
}
