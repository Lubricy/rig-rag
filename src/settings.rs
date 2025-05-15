use config::{Config, ConfigError, File};
use rig::{
    agent::{Agent, AgentBuilder},
    completion::{self, Chat, Prompt},
    providers::{anthropic, azure, openai}, streaming::{StreamingChat, StreamingPrompt},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[allow(unused)]
enum Provider {
    OpenAI {},
    Anthropic {},
    Azure {},
}


pub enum SAgent {
    OpenAI(Agent<openai::completion::CompletionModel>),
    Anthropic(Agent<anthropic::completion::CompletionModel>),
    Azure(Agent<azure::CompletionModel>),
}

impl Chat for SAgent {
    async fn chat(
        &self,
        prompt: impl Into<completion::Message> + Send,
        chat_history: Vec<completion::Message>,
    ) -> Result<String, completion::PromptError>
    {
        match self {
            SAgent::OpenAI(agent) => agent.chat(prompt, chat_history).await,
            SAgent::Anthropic(agent) => agent.chat(prompt, chat_history).await,
            SAgent::Azure(agent) => agent.chat(prompt, chat_history).await,
        }
    }
}

impl Prompt for SAgent {
    async fn prompt(
        &self,
        prompt: impl Into<completion::Message> + Send,
    ) -> Result<String, completion::PromptError> {
        match self {
            SAgent::OpenAI(agent) => agent.prompt(prompt).await,
            SAgent::Anthropic(agent) => agent.prompt(prompt).await,
            SAgent::Azure(agent) => agent.prompt(prompt).await,
        }
    }
}

impl StreamingPrompt for SAgent {
    async fn stream_prompt(
        &self,
        prompt: &str,
    ) -> Result<rig::streaming::StreamingResult, completion::CompletionError> {
        match self {
            SAgent::OpenAI(agent) => agent.stream_prompt(prompt).await,
            SAgent::Anthropic(agent) => agent.stream_prompt(prompt).await,
            SAgent::Azure(agent) => agent.stream_prompt(prompt).await,
        }
    }
}

impl StreamingChat for SAgent {
    async fn stream_chat(
        &self,
        prompt: &str,
        chat_history: Vec<completion::Message>,
    ) -> Result<rig::streaming::StreamingResult, completion::CompletionError> {
        match self {
            SAgent::OpenAI(agent) => agent.stream_chat(prompt, chat_history).await,
            SAgent::Anthropic(agent) => agent.stream_chat(prompt, chat_history).await,
            SAgent::Azure(agent) => agent.stream_chat(prompt, chat_history).await,
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    debug: bool,
    provider: Provider,
    model: String,
}

pub trait Builder {
    fn build<M: completion::CompletionModel>(self, builder: AgentBuilder<M>) -> AgentBuilder<M>;
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config"))
            .build()?;

        s.try_deserialize()
    }

    pub fn agent(&self, builder: impl Builder) -> SAgent {
        match &self.provider {
            Provider::OpenAI {  } => SAgent::OpenAI(builder.build(openai::Client::from_env().agent(&self.model)).build()),
            Provider::Anthropic {  } => SAgent::Anthropic(builder.build(anthropic::Client::from_env().agent(&self.model)).build()),
            Provider::Azure {  } => SAgent::Azure(builder.build(azure::Client::from_env().agent(&self.model)).build()),
        }
    }
}
