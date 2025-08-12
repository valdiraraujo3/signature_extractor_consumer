use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDetail {
    pub geolocation: String,
    pub ip_address: String,
    pub signed_at: String,
    pub user_agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfEventMessage {
    pub client_id: Uuid,
    pub events: Vec<EventDetail>,
}
