use rig::{
    client::{builder::{ClientFactory, DynClientBuilder}, ProviderClient, ProviderValue},
    completion::Prompt,
    providers::{openrouter}
};

fn init_from_env() -> Box<dyn ProviderClient> {
    let api_key  = std::env::var("MODEL_API_KEY").expect("MODEL_API_KEY not set");
    let base_url = std::env::var("MODEL_API_BASE").expect("MODEL_BASE_URL not set");
    Box::new(openrouter::client::Client::from_url(&api_key, &base_url))
}

fn init_from_val(p: ProviderValue) -> Box<dyn ProviderClient> {
    todo!()
}

#[tokio::main]
async fn main() {
    let multi_client = DynClientBuilder::new()
        .register(ClientFactory::new(
            "custom",
            init_from_env,
            init_from_val,
        ));

    // set up Custom client
    let completion_openai = multi_client.agent("custom", "openai/Qwen/Qwen3-235B-A22B-FP8").unwrap();
    let agent_openai = completion_openai
        .preamble("You are a helpful assistant")
        .build();

    println!("Sending prompt: 'Hello world!'");

    let res_openai = agent_openai.prompt("Hello world!").await.unwrap();
    println!("Response from model: {res_openai}");
}
