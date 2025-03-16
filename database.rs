use mongodb::{Client, Collection, Database};
use mongodb::bson::{doc, Document, oid::ObjectId};
use serde::{Serialize, Deserialize};
use tokio::sync::OnceCell;
use thiserror::Error;
use std::env;

static DB_CLIENT: OnceCell<Database> = OnceCell::const_new();

#[derive(Debug, Error)]
pub enum DbError {
    #[error("MongoDB error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    #[error("Connection string missing")]
    ConnectionStringMissing,
    #[error("Document not found")]
    NotFound,
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub sender_id: String,
    pub room_id: String,
    pub content: String,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub created_at: i64,
}

pub async fn init_db() -> Result<(), DbError> {
    let conn_str = env::var("MONGODB_URI").map_err(|_| DbError::ConnectionStringMissing)?;
    let client = Client::with_uri_str(conn_str).await?;
    let db = client.database("circle_chat");
    
    // Initialize the database client
    DB_CLIENT.set(db).expect("Failed to set database client");
    Ok(())
}

pub async fn get_db() -> &'static Database {
    DB_CLIENT.get().expect("Database not initialized")
}

// User operations
pub async fn create_user(user: User) -> Result<ObjectId, DbError> {
    let collection: Collection<User> = get_db().await.collection("users");
    let result = collection.insert_one(user, None).await?;
    Ok(result.inserted_id.as_object_id().unwrap())
}

pub async fn get_user_by_id(id: &str) -> Result<User, DbError> {
    let object_id = ObjectId::parse_str(id).map_err(|e| DbError::SerializationError(e.to_string()))?;
    let collection: Collection<User> = get_db().await.collection("users");
    let filter = doc! { "_id": object_id };
    collection.find_one(filter, None).await?.ok_or(DbError::NotFound)
}

// Message operations
pub async fn save_message(message: Message) -> Result<ObjectId, DbError> {
    let collection: Collection<Message> = get_db().await.collection("messages");
    let result = collection.insert_one(message, None).await?;
    Ok(result.inserted_id.as_object_id().unwrap())
}

pub async fn get_messages_by_room(room_id: &str, limit: i64) -> Result<Vec<Message>, DbError> {
    let collection: Collection<Message> = get_db().await.collection("messages");
    let filter = doc! { "room_id": room_id };
    let find_options = mongodb::options::FindOptions::builder()
        .sort(doc! { "timestamp": -1 })
        .limit(limit)
        .build();
    
    let cursor = collection.find(filter, find_options).await?;
    let messages: Vec<Message> = cursor.try_collect().await?;
    Ok(messages)
}

// Room operations
pub async fn create_room(room: Room) -> Result<ObjectId, DbError> {
    let collection: Collection<Room> = get_db().await.collection("rooms");
    let result = collection.insert_one(room, None).await?;
    Ok(result.inserted_id.as_object_id().unwrap())
}

pub async fn list_rooms() -> Result<Vec<Room>, DbError> {
    let collection: Collection<Room> = get_db().await.collection("rooms");
    let cursor = collection.find(None, None).await?;
    let rooms: Vec<Room> = cursor.try_collect().await?;
    Ok(rooms)
}