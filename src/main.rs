use anyhow::Result;
use tracing::*;

use throwsterhouse_five::client::Client;
use throwsterhouse_five::config::Config;
use throwsterhouse_five::setting::Setting;
use throwsterhouse_five::terminator;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    info!("lift off");

    let terminator = terminator::new();
    let config = Config::new()?;
    let setting = Setting::new(config).unwrap();

    let client = Client::new(setting);
    let client = client.listen();

    tokio::select! {
        _ = terminator => info!("terminator is comming"),
        _ = client => info!("mission completed")
    }

    info!("I'll be back");
    Ok(())
}
