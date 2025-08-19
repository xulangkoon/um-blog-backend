use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};
use std::env;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
  // 从环境变量读取 MongoDB URI
  let mongodb_uri = env::var("MONGODB_URI")
    .expect("MONGODB_URI environment variable must be set");
  
  let mut client_options = ClientOptions::parse(&mongodb_uri).await?;

  // Set the server_api field of the client_options object to set the version of the Stable API on the client
  let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
  client_options.server_api = Some(server_api);

  // Get a handle to the cluster
  let client = Client::with_options(client_options)?;

  // Ping the server to see if you can connect to the cluster
  client
    .database("admin")
    .run_command(doc! {"ping": 1}, None)
    .await?;
  println!("Pinged your deployment. You successfully connected to MongoDB!");

  // 尝试检查sample_mflix数据库中的集合
  let db = client.database("sample_mflix");
  let collections = db.list_collection_names(None).await?;
  println!("在sample_mflix数据库中发现以下集合:");
  for collection in collections {
      println!("  - {}", collection);
  }

  Ok(())
}