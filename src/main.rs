mod settings;
use settings::Settings;
use anyhow::Result;
use rig::{agent::{Agent, AgentBuilder}, completion::{self, Prompt}, providers::azure, streaming::StreamingPrompt};
use futures::stream::StreamExt;

struct MyBuilder;

impl settings::Builder for MyBuilder {
    fn build<M: completion::CompletionModel>(self, builder: AgentBuilder<M>) -> AgentBuilder<M> {
        builder.preamble("you are a priate.")
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings::new()?;
    println!("{settings:?}");
    let agent = settings.agent(MyBuilder);
    let resp = agent.prompt("hi").await?;
    println!("{resp}");

    let mut resp = agent.stream_prompt("hi").await?;
    while let Some(event) = resp.next().await {
        print!("{}", event?);
    }
    Ok(())
}
