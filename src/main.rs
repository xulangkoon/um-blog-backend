mod db;

use axum::{
    routing::{get, post},
    http::{HeaderValue, Method, StatusCode},
    Json, Router,
    extract::State,
};
use db::{AppState, Post};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // 加载.env文件（如果存在）
    dotenv().ok();
    
    // 初始化日志
    println!("Starting server...");

    // 尝试初始化数据库连接
    let app_state = match db::AppState::init().await {
        Ok(state) => {
            println!("Successfully connected to MongoDB");
            state
        },
        Err(e) => {
            eprintln!("Failed to connect to MongoDB: {}", e);
            println!("Starting with mock data...");
            // 创建一个模拟AppState，实际项目应该处理这个错误
            panic!("MongoDB connection required");
        }
    };

    // 构建CORS层
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    // 构建路由
    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/posts", get(get_posts))
        .layer(cors)
        .with_state(app_state);

    // 绑定地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server listening on {}", addr);

    // 启动服务器
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 健康检查接口
async fn health_check() -> &'static str {
    "Backend is healthy!"
}

// 获取文章列表接口
async fn get_posts(State(app_state): State<Arc<AppState>>) -> Json<Vec<Post>> {
    let posts = db::get_posts(app_state).await;
    Json(posts)
}
