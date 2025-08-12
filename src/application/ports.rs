use crate::domain::models::PdfEventMessage;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait EventConsumer {
    async fn consume_events(&self, handler: Box<Arc<dyn MessageHandler>>) -> Result<()>;
}

#[async_trait]
pub trait MessageHandler: Send + Sync {
    async fn handle(&self, message: PdfEventMessage) -> Result<()>;
}

#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn save(&self, event_message: &PdfEventMessage) -> Result<()>;
    async fn get_all(&self) -> Result<Vec<PdfEventMessage>>;
}
