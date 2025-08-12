use crate::application::ports::EventRepository;
use crate::domain::models::PdfEventMessage;
use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::{Client, Collection};

const MONGO_URI: &str = "mongodb://localhost:27017";
const DB_NAME: &str = "signature_extractor_db";
const COLLECTION_NAME: &str = "signed_events";

pub struct MongoDbRepository {
    collection: Collection<PdfEventMessage>,
}

impl MongoDbRepository {
    pub async fn new() -> Result<Self> {
        let client = Client::with_uri_str(MONGO_URI)
            .await
            .context("Falha ao criar cliente MongoDB")?;

        let database = client.database(DB_NAME);
        let collection = database.collection::<PdfEventMessage>(COLLECTION_NAME);

        println!("Conectado ao MongoDB e à collection '{COLLECTION_NAME}'");

        Ok(Self { collection })
    }
}

#[async_trait]
impl EventRepository for MongoDbRepository {
    async fn save(&self, event_message: &PdfEventMessage) -> Result<()> {
        self.collection
            .insert_one(event_message, None)
            .await
            .context("Falha ao inserir documento no MongoDB")?;

        println!(
            "Mensagem com client_id '{}' salva no MongoDB.",
            event_message.client_id
        );

        Ok(())
    }
    async fn get_all(&self) -> Result<Vec<PdfEventMessage>> {
        let cursor = self
            .collection
            .find(None, None)
            .await
            .context("Falha ao buscar documentos do MongoDB")?;

        let events: Vec<PdfEventMessage> = cursor.try_collect().await?;
        Ok(events)
    }
}
