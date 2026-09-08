use tracing::Level;

use crate::startup::start_app;

mod api;
mod modes;
mod startup;
mod state;
#[cfg(test)]
mod tests_utils;
mod ui;
mod utils;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), color_eyre::eyre::Error> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    color_eyre::install()?;
    dotenvy::dotenv()?;
    let result = start_app().await;

    ratatui::restore();
    result?;
    Ok(())
}
