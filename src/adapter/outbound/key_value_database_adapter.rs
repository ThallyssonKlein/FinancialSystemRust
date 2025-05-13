use redis::{Client, AsyncCommands};
use serde::{Serialize, Deserialize};
use std::error::Error;
use crate::adapter::inbound::receive_user_extract_adapter::Transaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct OutboundKeyValueDatabaseAdapter {
    client: Client,
}

impl OutboundKeyValueDatabaseAdapter {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get_last_item(&self, user_id: &str, file_type: &str) -> Result<Option<Transaction>, Box<dyn Error>> {
        let mut conn = self.client.get_async_connection().await?;
        let key = format!("{}:{}", user_id, file_type);
        let data: Option<String> = conn.get(&key).await?;
        
        match data {
            Some(json) => Ok(serde_json::from_str(&json)?),
            None => Ok(None),
        }
    }

    pub async fn save_last_item(&self, user_id: &str, transaction: &Transaction, file_type: &str) -> Result<(), Box<dyn Error>> {
        let mut conn = self.client.get_async_connection().await?;
        let key = format!("{}:{}", user_id, file_type);
        let json = serde_json::to_string(transaction)?;
        conn.set(&key, json).await?;
        Ok(())
    }
} 