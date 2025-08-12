use crate::application::ports::{EventRepository, MessageHandler};
use crate::domain::models::PdfEventMessage;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct PrintEventHandler;
pub struct SaveEventHandler<R: EventRepository> {
    repository: Arc<R>,
}
impl<R: EventRepository> SaveEventHandler<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

impl PrintEventHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl<R: EventRepository> MessageHandler for SaveEventHandler<R> {
    async fn handle(&self, message: PdfEventMessage) -> Result<()> {
        println!("--- Nova Mensagem Consumida ---");
        println!("{:#?}", message);
        println!("--------------------------------\n");
        self.repository.save(&message).await?;
        println!(
            "Mensagem com client_id '{}' salva no repositório.",
            message.client_id
        );
        Ok(())
    }
}

#[async_trait]
impl MessageHandler for PrintEventHandler {
    async fn handle(&self, message: PdfEventMessage) -> Result<()> {
        println!("--- Nova Mensagem Consumida ---");
        println!("{:#?}", message);
        println!("--------------------------------\n");
        Ok(())
    }
}
