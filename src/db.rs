use mongodb::{Client, Collection, Database};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub title: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub posts_collection: Collection<Post>,
}

impl AppState {
    pub async fn init() -> Result<Arc<Self>, mongodb::error::Error> {
        // 这里应该从环境变量或配置文件中读取连接字符串
        // 在实际项目中使用dotenv或类似工具加载.env文件
        let mongodb_uri = env::var("MONGODB_URI")
            .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
        
        let db_name = env::var("DB_NAME").unwrap_or_else(|_| "um_blog".to_string());
        
        println!("Connecting to MongoDB...");
        let client = Client::with_uri_str(&mongodb_uri).await?;
        let db = client.database(&db_name);
        
        let posts_collection = db.collection::<Post>("posts");
        
        Ok(Arc::new(Self {
            db,
            posts_collection,
        }))
    }
}

// 这个函数在后续实现中将从数据库获取帖子
// 现在先返回模拟数据
pub async fn get_posts(_app_state: Arc<AppState>) -> Vec<Post> {
    // TODO: 从MongoDB获取数据
    // 目前返回模拟数据
    vec![
        Post {
            id: None,
            title: "Hello Axum".to_string(),
            content: "This is a test post from Axum backend".to_string(),
            created_at: "2025-05-01".to_string(),
        },
        Post {
            id: None,
            title: "Learning Rust".to_string(),
            content: "Rust is an amazing language for building backend services".to_string(),
            created_at: "2025-05-02".to_string(),
        },
    ]
} 