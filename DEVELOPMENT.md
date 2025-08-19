# UM Blog Backend 开发文档

## MongoDB Atlas 连接配置与测试

### 连接配置过程

#### 1. 环境变量配置

在项目根目录创建 `.env` 文件，配置 MongoDB Atlas 连接信息：

```env
# MongoDB Atlas 连接配置
MONGODB_URI=mongodb+srv://<username>:<password>@cluster0.vkbsgmx.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0
DB_NAME=um_blog

# 服务器配置
PORT=8080
HOST=127.0.0.1
```

**注意事项：**
- 将 `<username>` 和 `<password>` 替换为实际的 Atlas 凭据
- 如果密码包含特殊字符（如 @:/?&=），需要进行 URL 百分号编码
- `.env` 文件已添加到 `.gitignore` 中，确保不会被提交到版本控制

#### 2. Atlas 端配置要求

在 MongoDB Atlas 控制台完成以下配置：

1. **创建 Cluster**：选择免费层或付费层集群
2. **数据库用户**：在 Database Access 中创建用户，记录用户名和密码
3. **网络访问**：在 Network Access 中添加 IP 白名单
   - 开发环境可临时使用 `0.0.0.0/0`（不推荐生产环境）
   - 生产环境应添加具体的服务器 IP
4. **连接字符串**：复制标准驱动程序连接字符串

### 连接实现原理

#### 1. 代码结构

**主要文件：**
- `src/main.rs`：应用入口，服务器启动和路由配置
- `src/db.rs`：数据库连接和状态管理
- `.env`：环境变量配置

#### 2. 连接流程

```rust
// 1. 加载环境变量
dotenv().ok();

// 2. 初始化数据库连接
let app_state = match db::AppState::init().await {
    Ok(state) => {
        println!("Successfully connected to MongoDB");
        state
    },
    Err(e) => {
        eprintln!("Failed to connect to MongoDB: {}", e);
        panic!("MongoDB connection required");
    }
};
```

#### 3. AppState 设计

```rust
#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub posts_collection: Collection<Post>,
}
```

**设计原理：**
- 使用 `Arc<AppState>` 在多个请求处理器间共享数据库连接
- 预先获取常用集合的引用，避免重复查找
- 实现 `Clone` trait 以支持 Axum 的状态共享机制

### 测试方法

#### 1. 编译和启动

```bash
# 进入后端目录
cd um-blog-backend

# 编译并运行（指定二进制文件）
cargo run --bin backend
```

#### 2. 连接状态验证

启动成功后，控制台应显示：
```
Starting server...
Connecting to MongoDB...
Successfully connected to MongoDB
Server listening on 127.0.0.1:8080
```

#### 3. API 接口测试

**健康检查接口：**
```bash
curl --noproxy '*' http://127.0.0.1:8080/ && echo
# 预期输出：Backend is healthy!
```

**文章列表接口：**
```bash
curl --noproxy '*' http://127.0.0.1:8080/api/posts && echo
# 预期输出：JSON 格式的文章列表
```

**注意：** 如果系统配置了代理，需要使用 `--noproxy '*'` 参数绕过代理。

### 常见问题排查

#### 1. 认证失败
- 检查用户名和密码是否正确
- 确认密码中的特殊字符已正确编码
- 验证数据库用户权限设置

#### 2. 网络连接问题
- 检查 Atlas Network Access 白名单配置
- 确认本地网络可以访问外网
- 验证防火墙设置

#### 3. DNS 解析问题
- MongoDB Atlas 使用 SRV 记录，确保 DNS 解析正常
- 可以尝试使用标准连接字符串替代 SRV 格式

#### 4. 编译错误
- Axum 0.8 版本移除了 `Server`，使用 `tokio::net::TcpListener` 和 `axum::serve`
- 确保所有依赖版本兼容

### 安全最佳实践

1. **环境变量管理**
   - 永远不要将 `.env` 文件提交到版本控制
   - 生产环境使用更安全的密钥管理服务

2. **网络安全**
   - 生产环境避免使用 `0.0.0.0/0` IP 白名单
   - 定期轮换数据库凭据

3. **连接池配置**
   - 根据应用负载调整连接池大小
   - 设置合适的连接超时时间

### 性能优化建议

1. **连接复用**
   - MongoDB 驱动自动管理连接池
   - 避免频繁创建新连接

2. **索引优化**
   - 为常用查询字段创建索引
   - 定期分析查询性能

3. **数据模型设计**
   - 根据查询模式设计文档结构
   - 合理使用嵌入文档和引用