# UM Blog Backend

个人博客系统的后端服务，使用 Rust 构建的高性能 Web API。

## 技术栈

### 核心框架
- **Rust** (Edition 2024) - 系统编程语言，提供内存安全和高性能
- **Axum 0.8.3** - 现代异步 Web 框架，基于 tokio 和 hyper
- **Tokio 1.44.2** - 异步运行时，支持高并发处理

### 数据库
- **MongoDB 2.7.0** - NoSQL 文档数据库
- **MongoDB Atlas** - 云端托管的 MongoDB 服务

### 中间件和工具
- **Tower 0.5.2** - 服务抽象层，提供中间件支持
- **Tower-HTTP 0.6.2** - HTTP 中间件集合，包含 CORS 支持
- **Serde 1.0.219** - 序列化/反序列化框架
- **dotenv 0.15.0** - 环境变量管理

## 项目结构

```
um-blog-backend/
├── src/
│   ├── main.rs          # 应用入口点，服务器配置和路由
│   ├── db.rs            # 数据库连接和数据模型
│   └── bin/
│       └── test_db.rs   # 数据库连接测试工具
├── Cargo.toml           # 项目配置和依赖管理
├── .env.example         # 环境变量模板
├── .env                 # 环境变量配置（不提交到版本控制）
├── .gitignore           # Git 忽略规则
├── README.md            # 项目文档
└── DEVELOPMENT.md       # 开发文档
```

## 核心功能

### 当前实现

#### 1. Web 服务器
- **端口**: 8080 (可配置)
- **地址**: 127.0.0.1 (可配置)
- **协议**: HTTP/1.1
- **并发**: 基于 Tokio 的异步处理

#### 2. API 接口

| 方法 | 路径 | 功能 | 状态 |
|------|------|------|------|
| GET | `/` | 健康检查 | ✅ 已实现 |
| GET | `/api/posts` | 获取文章列表 | ✅ 已实现（模拟数据）|

#### 3. 数据模型

**Post 文章模型：**
```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub title: String,
    pub content: String,
    pub created_at: String,
}
```

#### 4. 跨域支持 (CORS)
- 允许来源: `http://localhost:3000` (Next.js 前端)
- 允许方法: GET, POST, PUT, DELETE
- 允许头部: 所有头部

### 计划功能

#### 1. 文章管理
- [ ] 创建文章 (POST `/api/posts`)
- [ ] 更新文章 (PUT `/api/posts/:id`)
- [ ] 删除文章 (DELETE `/api/posts/:id`)
- [ ] 文章详情 (GET `/api/posts/:id`)
- [ ] 文章搜索和过滤

#### 2. 用户系统
- [ ] 用户注册和登录
- [ ] JWT 身份验证
- [ ] 权限管理

#### 3. 评论系统
- [ ] 文章评论 CRUD
- [ ] 评论嵌套回复
- [ ] 评论审核

#### 4. 文件上传
- [ ] 图片上传
- [ ] 文件存储（本地/云存储）

## 环境配置

### 必需环境变量

```env
# MongoDB 连接
MONGODB_URI=mongodb+srv://username:password@cluster.mongodb.net/?retryWrites=true&w=majority
DB_NAME=um_blog

# 服务器配置
PORT=8080
HOST=127.0.0.1
```

### 开发环境设置

1. **安装 Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **克隆项目**
   ```bash
   git clone <repository-url>
   cd um-blog-backend
   ```

3. **配置环境变量**
   ```bash
   cp .env.example .env
   # 编辑 .env 文件，填入实际的 MongoDB 连接信息
   ```

4. **运行项目**
   ```bash
   cargo run --bin backend
   ```

## 部署

### 本地部署

```bash
# 构建发布版本
cargo build --release

# 运行发布版本
./target/release/backend
```

### Docker 部署 (计划)

```dockerfile
# 待实现 Dockerfile
FROM rust:1.70 as builder
# ... 构建步骤

FROM debian:bookworm-slim
# ... 运行时配置
```

## 性能特性

### 优势
- **内存安全**: Rust 的所有权系统防止内存泄漏和数据竞争
- **零成本抽象**: 编译时优化，运行时性能接近 C/C++
- **异步处理**: 基于 Tokio 的高并发处理能力
- **类型安全**: 编译时类型检查，减少运行时错误

### 性能指标 (预期)
- **并发连接**: 10,000+ (基于 Tokio)
- **响应时间**: < 10ms (简单查询)
- **内存占用**: < 50MB (基础服务)
- **CPU 使用**: 高效的异步处理

## 开发工具

### 测试工具
```bash
# 数据库连接测试
cargo run --bin test_db

# API 测试
curl http://127.0.0.1:8080/
curl http://127.0.0.1:8080/api/posts
```

### 代码质量
```bash
# 代码格式化
cargo fmt

# 代码检查
cargo clippy

# 运行测试
cargo test
```

## 安全考虑

### 已实现
- 环境变量管理（`.env` 文件不提交）
- CORS 配置限制跨域访问
- MongoDB 连接字符串加密传输

### 计划实现
- [ ] JWT 身份验证
- [ ] API 速率限制
- [ ] 输入验证和清理
- [ ] HTTPS 支持
- [ ] 日志记录和监控

## 监控和日志

### 当前日志
- 服务器启动状态
- MongoDB 连接状态
- 基本请求日志

### 计划改进
- [ ] 结构化日志 (JSON 格式)
- [ ] 日志级别配置
- [ ] 性能指标收集
- [ ] 错误追踪

## 贡献指南

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启 Pull Request

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 联系方式

- 项目维护者: [Your Name]
- 邮箱: [your.email@example.com]
- 项目链接: [https://github.com/yourusername/um-blog-backend]