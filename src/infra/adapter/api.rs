use crate::application::ports::EventRepository;
use crate::domain::models::PdfEventMessage;
use axum::{Router, extract::State, http::StatusCode, response::Json, routing::get};
use std::sync::Arc;
type AppState = Arc<dyn EventRepository>;

async fn health_check() -> &'static str {
    "OK"
}

async fn get_reports(
    State(repo): State<AppState>,
) -> Result<Json<Vec<PdfEventMessage>>, StatusCode> {
    match repo.get_all().await {
        Ok(events) => Ok(Json(events)),
        Err(e) => {
            eprintln!("Erro ao buscar relatórios: {e:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn run_api_server(repo: AppState) {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/reports", get(get_reports))
        .with_state(repo);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Servidor API rodando em http://localhost:8000");
    println!("Endpoint de relatórios disponível em http://localhost:8000/reports");

    axum::serve(listener, app).await.unwrap();
}
