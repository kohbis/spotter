mod aws;
mod cli;
mod display;

use clap::Parser;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    // Validate CLI arguments
    if let Err(e) = cli.validate() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    let client = Client::new();
    let advisor_data = aws::fetch_spot_advisor_data(&client).await?;
    let price_data = aws::fetch_spot_price_data(&client).await?;

    display::display_spot_data(
        &cli.region,
        cli.instance_type.as_deref(),
        &advisor_data,
        &price_data,
        cli.spot_price,
    )?;

    Ok(())
}
