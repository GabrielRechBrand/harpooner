use std::error::Error;
use mongodb::{Client, Database};
use mongodb::options::{ClientOptions, ResolverConfig};

pub async fn create_client() -> Result<Database, Box<dyn Error>> {
    let client_uri = std::env::var("DATABASE_URL")?;

    let mut options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;

    options.app_name = Some("My App".to_string());
    options.retry_writes = Some(true);
    options.direct_connection = Some(false);
    options.server_selection_timeout = Some(std::time::Duration::from_secs(5));
    options.max_pool_size = Some(10);

    // Set the database name
    options.default_database = Some("harpooner_dev".to_string());

    let client = Client::with_options(options)?.database(&std::env::var("DATABASE_NAME").unwrap());
    Ok(client)
}
