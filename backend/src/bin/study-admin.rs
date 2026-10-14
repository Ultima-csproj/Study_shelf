use backend::config::Config;
use study_storage_postgres::{PostgresStorage, account::PostgresAccountRepository};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().map(String::as_str).unwrap_or("help");
    if command == "help" || command == "--help" {
        println!(
            "study-admin commands:\n  grant-platform-admin / revoke-platform-admin --user-id N\n  grant-institution-admin / revoke-institution-admin --user-id N --institution-id N\n  accept-membership / reject-membership --user-id N --institution-id N --role student|teacher\n  accept-contributor / reject-contributor --user-id N --contributor-id N\n  cleanup-sessions / migrate"
        );
        return Ok(());
    }
    let config = Config::from_env()?;
    let storage = PostgresStorage::open(
        &config.database_url,
        config.max_connections,
        config.acquire_timeout,
    )
    .await?;
    let repo = PostgresAccountRepository::new(storage.pool().clone());
    match command {
        "grant-platform-admin" | "revoke-platform-admin" => {
            repo.platform_admin(id(&args, "--user-id")?, command.starts_with("grant"))
                .await?
        }
        "grant-institution-admin" | "revoke-institution-admin" => {
            repo.institution_admin(
                id(&args, "--user-id")?,
                id(&args, "--institution-id")?,
                command.starts_with("grant"),
            )
            .await?
        }
        "accept-membership" | "reject-membership" => {
            repo.decide_membership(
                id(&args, "--user-id")?,
                id(&args, "--institution-id")?,
                value(&args, "--role")?,
                command.starts_with("accept"),
            )
            .await?
        }
        "accept-contributor" | "reject-contributor" => {
            repo.link_contributor(
                id(&args, "--user-id")?,
                id(&args, "--contributor-id")?,
                command.starts_with("accept"),
            )
            .await?
        }
        "cleanup-sessions" => println!("Removed {} sessions", repo.cleanup_sessions().await?),
        "migrate" => {}
        _ => anyhow::bail!("Unknown command; run --help"),
    }
    println!("Operation completed: {command}");
    storage.pool().close().await;
    Ok(())
}
fn value<'a>(args: &'a [String], name: &str) -> anyhow::Result<&'a str> {
    let position = args
        .iter()
        .position(|v| v == name)
        .ok_or_else(|| anyhow::anyhow!("Missing {name}"))?;
    args.get(position + 1)
        .map(String::as_str)
        .ok_or_else(|| anyhow::anyhow!("Missing value for {name}"))
}
fn id(args: &[String], name: &str) -> anyhow::Result<i64> {
    let id = value(args, name)?.parse()?;
    anyhow::ensure!(id > 0, "{name} must be positive");
    Ok(id)
}
