use crate::database::DatabaseHandler;
use crate::paste::KeyClient;
use log::info;


#[derive(Clone)]
pub struct AppState {
    db: DatabaseHandler,
    key_client: KeyClient,
}

impl AppState {
    pub async fn new() -> Self {
        info!("Init AppState.");
        Self {
            db: DatabaseHandler::new()
                .await
                .expect("Failed to initialize mongoDB"),
            key_client: KeyClient::new(),
        }
    }

    pub fn get_db(&self) -> &DatabaseHandler {
        &self.db
    }

    pub fn get_key_client(&self) -> &KeyClient {
        &self.key_client
    }
}
