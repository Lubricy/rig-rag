use std::sync::Arc;
use anyhow::Result;
use rig::completion::Prompt;
use rig::Embed;
use serde::Serialize;
use rig::{
    client::builder::DynClientBuilder,
    embeddings::{embedding::EmbeddingModelDyn, EmbeddingModel, EmbeddingsBuilder},
    vector_store::in_memory_store::InMemoryVectorStore,
};
#[derive(Clone)]
pub struct EmbeddingModelHandle<'a> {
    inner: Arc<dyn EmbeddingModelDyn + 'a>,
}

impl EmbeddingModel for EmbeddingModelHandle<'_> {
    const MAX_DOCUMENTS: usize = 96;

    fn ndims(&self) -> usize {
        return self.inner.ndims();
    }

    fn embed_texts(
        &self,
        texts: impl IntoIterator<Item = String> + Send,
    ) -> impl std::future::Future<Output = std::result::Result<Vec<rig::embeddings::Embedding>, rig::embeddings::EmbeddingError>> + Send {
        return self.inner.embed_texts(texts.into_iter().collect())
    }
}

impl<'a> From<Box<dyn EmbeddingModelDyn + 'a>> for EmbeddingModelHandle<'a> {
    fn from(source: Box<dyn EmbeddingModelDyn + 'a>) -> Self {
        Self {
            inner: source.into()
        }
    }
}

#[derive(Embed, Serialize, Clone, Debug, Eq, PartialEq, Default)]
struct WordDefinition {
    id: String,
    word: String,
    #[embed]
    definitions: Vec<String>,
}
#[tokio::main]
async fn main() -> Result<()> {
    let client = DynClientBuilder::new();
    let embedding_model: EmbeddingModelHandle = client.embeddings("azure", std::env::var("AZURE_EMBEDDING_MODEL")?.as_str())?.into();
    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .documents(vec![
            WordDefinition {
                id: "doc0".to_string(),
                word: "flurbo".to_string(),
                definitions: vec![
                    "1. *flurbo* (name): A flurbo is a green alien that lives on cold planets.".to_string(),
                    "2. *flurbo* (name): A fictional digital currency that originated in the animated series Rick and Morty.".to_string()
                ]
            },
            WordDefinition {
                id: "doc1".to_string(),
                word: "glarb-glarb".to_string(),
                definitions: vec![
                    "1. *glarb-glarb* (noun): A glarb-glarb is a ancient tool used by the ancestors of the inhabitants of planet Jiro to farm the land.".to_string(),
                    "2. *glarb-glarb* (noun): A fictional creature found in the distant, swampy marshlands of the planet Glibbo in the Andromeda galaxy.".to_string()
                ]
            },
            WordDefinition {
                id: "doc2".to_string(),
                word: "linglingdong".to_string(),
                definitions: vec![
                    "1. *linglingdong* (noun): A term used by inhabitants of the far side of the moon to describe humans.".to_string(),
                    "2. *linglingdong* (noun): A rare, mystical instrument crafted by the ancient monks of the Nebulon Mountain Ranges on the planet Quarm.".to_string()
                ]
            },
        ])?
        .build()
        .await?;
    // Create vector store with the embeddings
    let vector_store = InMemoryVectorStore::from_documents(embeddings);

    // Create vector store index
    let index = vector_store.index(embedding_model.clone());
    let rag_agent = client.agent("azure", std::env::var("AZURE_LLM_MODEL")?.as_str())?
                          .preamble("
            You are a dictionary assistant here to assist the user in understanding the meaning of words.
            You will find additional non-standard word definitions that could be useful below.
        ")
        .dynamic_context(1, index)
        .build();

    // Prompt the agent and print the response
    let response = rag_agent.prompt("What does \"glarb-glarb\" mean?").await?;

    println!("{response}");
    Ok(())
}
