use crate::common::InternalServerError;
use crate::common::{KeyData, PasteData, UserData};
use actix_web::http::uri;
use log::{debug, info};
use mongodb::{bson::doc, options::ClientOptions, Client, Collection, Database};
use std::error::Error;
use std::env;

#[derive(Clone, Debug)]
pub struct DatabaseHandler {
    user_data_collection: Collection<UserData>,
    paste_data_collection: Collection<PasteData>,
    keys_data_collection: Collection<KeyData>,
}

impl DatabaseHandler {
    const DATABASE_NAME: &str = "db";
    const USERS_COLLECTION: &str = "users";
    const PASTES_COLLECTION: &str = "pastes";
    const KEYS_COLLECTION: &str = "keys";

    pub async fn new() -> Result<Self, InternalServerError> {
        info!("Init DatabaseHandler.");

        let db = Self::get_database()
            .await
            .map_err(|e| InternalServerError::MongoDbError(e.to_string()))?;
        let user_data_collection: Collection<UserData> = db.collection(Self::USERS_COLLECTION);
        let paste_data_collection: Collection<PasteData> = db.collection(Self::PASTES_COLLECTION);
        let keys_data_collection: Collection<KeyData> = db.collection(Self::KEYS_COLLECTION);

        Ok(Self {
            user_data_collection: user_data_collection.clone(),
            paste_data_collection: paste_data_collection.clone(),
            keys_data_collection: keys_data_collection.clone(),
        })
    }

    async fn get_database() -> Result<Database, Box<dyn Error>> {
        let scheme: uri::Scheme = env::var("MONGODB_SCHEME").unwrap().parse().unwrap();
        let ip = env::var("MONGODB_IP").unwrap();
        let port = env::var("MONGODB_PORT").unwrap();
        let mongodb_uri = uri::Builder::new()
            .scheme(scheme)
            .authority([ip, port].join(":").as_str())
            .path_and_query("")
            .build()
            .unwrap();

        info!("Connect to db at {}.", mongodb_uri);

        let client_options = ClientOptions::parse(mongodb_uri.to_string()).await?;
        let client = Client::with_options(client_options)?;
        Ok(client.database(Self::DATABASE_NAME))
    }

    pub async fn add_user_data(&self, user_data: &UserData) -> Result<(), InternalServerError> {
        info!(
            "Add user data id: {}, username: {}, password {}.",
            user_data.id, user_data.username, user_data.password
        );

        self.check_if_user_exist(&user_data.username).await?;
        match self.user_data_collection.insert_one(user_data).await {
            Ok(_) => Ok(()),
            Err(e) => Err(InternalServerError::MongoDbError(e.to_string())),
        }
    }

    async fn check_if_user_exist(&self, username: &String) -> Result<(), InternalServerError> {
        let user_data = self.get_user_data(&username).await?;
        match user_data {
            Some(_) => Err(InternalServerError::InvalidDbData(
                "User already exist.".to_string(),
            )),
            None => Ok(()),
        }
    }

    pub async fn get_user_data(
        &self,
        username: &String,
    ) -> Result<Option<UserData>, InternalServerError> {
        info!("Get user {} from db.", username);

        match self
            .user_data_collection
            .find_one(doc! {"username": username})
            .await
        {
            Ok(user_data) => Ok(user_data),
            Err(e) => Err(InternalServerError::MongoDbError(e.to_string())),
        }
    }

    pub async fn add_paste(&self, paste: &PasteData) -> Result<(), InternalServerError> {
        info!("Add paste with key {}.", paste.key);
        debug!("Add paste with content {}.", paste.content);

        self.check_if_paste_exist(&paste.key).await?;
        match self.paste_data_collection.insert_one(paste).await {
            Ok(_) => Ok(()),
            Err(e) => Err(InternalServerError::MongoDbError(e.to_string())),
        }
    }

    async fn check_if_paste_exist(&self, key: &String) -> Result<(), InternalServerError> {
        let paste_data = self.get_paste(&key).await?;
        match paste_data {
            Some(_) => Err(InternalServerError::InvalidDbData(
                "Paste already exist.".to_string(),
            )),
            None => Ok(()),
        }
    }

    pub async fn get_paste(&self, key: &String) -> Result<Option<PasteData>, InternalServerError> {
        info!("Get paste with key {}.", key);

        match self.paste_data_collection.find_one(doc! {"key": key}).await {
            Ok(paste_data) => Ok(paste_data),
            Err(e) => Err(InternalServerError::MongoDbError(e.to_string())),
        }
    }

    pub async fn delete_paste(&self, key: &String) -> Result<(), InternalServerError> {
        info!("Delete paste with key {}.", key);

        match self
            .paste_data_collection
            .delete_one(doc! {"key": key})
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(InternalServerError::MongoDbError(e.to_string())),
        }
    }

    pub async fn add_key(&self, id: &String, key: &String) -> Result<(), InternalServerError> {
        info!("Add key {} for user {}.", &key, &id);

        match self.get_keys(&id).await? {
            Some(_) => self.update_keys_for_user(&id, &key).await,
            None => self.create_keys_for_user(&id, &key).await,
        }
    }

    pub async fn update_keys_for_user(
        &self,
        id: &String,
        key: &String,
    ) -> Result<(), InternalServerError> {
        match self
            .keys_data_collection
            .update_one(doc! {"id": id}, doc! {"$push": {"keys": key}})
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(InternalServerError::MongoDbError(e.to_string())),
        }
    }

    pub async fn create_keys_for_user(
        &self,
        id: &String,
        key: &String,
    ) -> Result<(), InternalServerError> {
        let key_data = KeyData {
            id: id.clone(),
            keys: vec![key.clone()],
        };

        match self.keys_data_collection.insert_one(key_data).await {
            Ok(_) => Ok(()),
            Err(e) => Err(InternalServerError::MongoDbError(e.to_string())),
        }
    }

    pub async fn get_keys(&self, id: &String) -> Result<Option<KeyData>, InternalServerError> {
        info!("Get paste for user {}.", &id);

        match self.keys_data_collection.find_one(doc! {"id": id}).await {
            Ok(key_data) => Ok(key_data),
            Err(e) => Err(InternalServerError::MongoDbError(e.to_string())),
        }
    }

    pub async fn dekete_key(&self, id: &String, key: &String) -> Result<(), InternalServerError> {
        info!("Delete key {} for user {}.", &key, &id);

        match self
            .keys_data_collection
            .update_one(doc! {"id": id}, doc! {"$pull": {"keys": key}})
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(InternalServerError::MongoDbError(e.to_string())),
        }
    }
}
