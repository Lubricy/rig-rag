mod settings;
use settings::Settings;
use anyhow::Result;
use rig::completion::Prompt;

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings::new()?;
    println!("{settings:?}");
    let agent = settings.agent();
    let resp = agent.prompt("hi").await?;
    println!("{resp}");
    Ok(())
}
