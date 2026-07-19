use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Event {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub actor: Actor,
    pub repo: EventRepo,
    pub payload: serde_json::Value,
    pub created_at: String,
}

#[derive(Deserialize, Debug)]
pub struct Actor {
    pub login: String,
}

#[derive(Deserialize, Debug)]
pub struct EventRepo {
    pub name: String,
}
