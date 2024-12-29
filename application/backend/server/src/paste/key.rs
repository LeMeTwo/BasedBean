use crate::common::InternalServerError;
use actix_web::http::uri;
use awc::{http::StatusCode, Client};
use log::info;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct KegKeyRespData {
    key: String,
}

#[derive(Deserialize)]
struct KegKeyResp {
    data: KegKeyRespData,
}

#[derive(Clone)]
pub struct KeyClient {
    scheme: uri::Scheme,
    ip: String,
    port: String,
}

impl KeyClient {
    pub fn new() -> Self {
        info!("Init KeyClient.");
        Self {
            scheme: env::var("KEG_SCHEME").unwrap().parse().unwrap(),
            ip: env::var("KEG_IP").unwrap(),
            port: env::var("KEG_PORT").unwrap(),
        }
    }

    pub async fn get_key(&self) -> Result<String, InternalServerError> {
        let uri = self.get_uri("/v1/key");
        info!("Send key request to URI: {}.", uri);

        let mut resp = Client::new()
            .get(&uri)
            .send()
            .await
            .map_err(|e| InternalServerError::ServerComponentError(e.to_string()))?;

        self.is_resp_status_valid(&resp.status())?;

        match resp.json::<KegKeyResp>().await {
            Ok(keg_data) => {
                info!("Generated key {}.", keg_data.data.key);
                Ok(keg_data.data.key)
            }
            Err(e) => Err(InternalServerError::KegInvalidData(e.to_string())),
        }
    }

    fn get_uri(&self, path: &str) -> uri::Uri {
        uri::Builder::new()
            .scheme(self.scheme.clone())
            .authority([self.ip.as_str(), self.port.as_str()].join(":"))
            .path_and_query(path)
            .build()
            .unwrap()
    }

    fn is_resp_status_valid(&self, status: &StatusCode) -> Result<(), InternalServerError> {
        if status.is_client_error() {
            Err(InternalServerError::KegClientErrorResponse(
                status.to_string(),
            ))
        } else if status.is_server_error() {
            Err(InternalServerError::KegServerErrorResponse(
                status.to_string(),
            ))
        } else {
            Ok(())
        }
    }

    pub async fn delete_key(&self, key: &String) -> Result<(), InternalServerError> {
        let uri = self.get_uri(&["/v1/key", key.as_str()].join("/"));
        info!("Send delete request to URI: {}.", uri);

        let resp = Client::new()
            .delete(&uri)
            .send()
            .await
            .map_err(|e| InternalServerError::ServerComponentError(e.to_string()))?;

        self.is_resp_status_valid(&resp.status())
    }
}
