use std::sync::Arc;

use crate::application::ports::{EventConsumer, MessageHandler};
use anyhow::{Context, Result};
use async_nats::jetstream;
use async_trait::async_trait;
use futures::StreamExt;

const NATS_URL: &str = "nats://192.168.2.107:4222";
const STREAM_NAME: &str = "PDF_PROCESSING";
const SUBJECT_NAME: &str = "pdf.events.started";

pub struct NatsEventConsumer {
    client: async_nats::Client,
    jetstream: jetstream::Context,
}

impl NatsEventConsumer {
    pub async fn new() -> Result<Self> {
        let client = async_nats::connect(NATS_URL)
            .await
            .context("Falha ao conectar ao servidor NATS")?;
        let jetstream = jetstream::new(client.clone());
        Ok(Self { client, jetstream })
    }
}

#[async_trait]
impl EventConsumer for NatsEventConsumer {
    async fn consume_events(&self, handler: Box<Arc<dyn MessageHandler>>) -> Result<()> {
        let stream = self
            .jetstream
            .get_or_create_stream(jetstream::stream::Config {
                name: STREAM_NAME.to_string(),
                subjects: vec![format!("{}_STREAM", SUBJECT_NAME)],
                ..Default::default()
            })
            .await
            .context("Falha ao obter ou criar o stream")?;

        println!("Ouvindo mensagens no assunto: '{}'", SUBJECT_NAME);

        let consumer = stream
            .create_consumer(jetstream::consumer::pull::Config {
                durable_name: Some("pdf-events-processor".to_string()),
                filter_subject: SUBJECT_NAME.to_string(),
                ..Default::default()
            })
            .await
            .context("Falha ao criar o consumidor")?;

        let mut messages = consumer.messages().await?.take(100);

        while let Some(Ok(message)) = messages.next().await {
            match serde_json::from_slice::<crate::domain::models::PdfEventMessage>(&message.payload)
            {
                Ok(pdf_event) => {
                    if let Err(e) = handler.handle(pdf_event).await {
                        eprintln!("Erro ao processar a mensagem: {e:?}");
                        if let Err(e) = message.ack().await {
                            eprintln!("Falha ao enviar ACK da mensagem: {e:?}");
                        }
                    } else if let Err(e) = message.ack().await {
                        eprintln!("Falha ao enviar ACK da mensagem: {e:?}");
                    }
                }
                Err(e) => {
                    eprintln!("Erro de desserialização: Payload inválido. Erro: {e:?}");
                    if let Err(e) = message.ack().await {
                        eprintln!("Falha ao enviar ACK da mensagem inválida: {e:?}");
                    }
                }
            }
        }

        Ok(())
    }
}
