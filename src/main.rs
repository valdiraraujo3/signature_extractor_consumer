pub mod application;
pub mod domain;
pub mod infra;

use anyhow::Result;
use application::ports::EventConsumer;
use application::use_cases::PrintEventHandler;
use application::use_cases::SaveEventHandler;
use infra::adapter::api::run_api_server;
use infra::adapter::mongo::MongoDbRepository;
use infra::adapter::nats::NatsEventConsumer;
use std::sync::Arc;

/* Função principal que inicia o consumidor NATS e o servidor API.
   O MongoDbRepository é usado para salvar os eventos processados.
   O NatsEventConsumer é responsável por consumir eventos do NATS e passar para o handler.
   O PrintEventHandler imprime os eventos no console.
   O SaveEventHandler salva os eventos no MongoDB.
   As tarefas são executadas concorrentemente usando tokio::spawn e tokio::select.
*/

#[tokio::main]
async fn main() -> Result<()> {
    let nats_consumer = NatsEventConsumer::new().await?;
    let mongo_repo = Arc::new(MongoDbRepository::new().await?);
    let event_handler = Arc::new(SaveEventHandler::new(mongo_repo.clone()));
    // let event_handler = Arc::new(PrintEventHandler::new());
    let nats_task = tokio::spawn(async move {
        if let Err(e) = nats_consumer
            .consume_events(Box::new(event_handler.clone()))
            .await
        {
            eprintln!("O consumidor NATS encontrou um erro fatal: {e:?}");
        }
    });

    let api_task = tokio::spawn(async {
        run_api_server(mongo_repo).await;
    });

    tokio::select! {
        _ = nats_task => println!("Tarefa NATS finalizada."),
        _ = api_task => println!("Tarefa API finalizada."),
    }

    Ok(())
}
