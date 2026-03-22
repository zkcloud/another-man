# Work Log - Another Man 项目工作日志

> 记录关键操作、决策逻辑和进展

---

## 2026-03-22

### 15:41 - 开始 Phase 1 Week 1

**规划：**
Week 1 目标：基础架构 + HTTP 请求
- 1.1.1 添加后端依赖
- 1.1.2 创建数据模型
- 1.1.3 实现 HTTP 客户端
- 1.1.4 创建 Tauri Commands
- 1.1.5 实现前端 UI

### 15:45 - 添加后端依赖

**操作：**
- 更新 Cargo.toml
- 添加 reqwest (HTTP 客户端)
- 添加 rusqlite (数据库)
- 添加 tokio (异步运行时)
- 添加 chrono, uuid, anyhow (工具库)

### 15:50 - 创建数据模型

**文件：**
- `src/models/request.rs` - HttpRequest, HttpMethod, Header, Param, RequestBody
- `src/models/response.rs` - HttpResponse, ResponseError, ErrorKind
- `src/models/mod.rs` - 模块导出

### 16:00 - 实现 HTTP 客户端

**文件：** `src/core/http_client.rs`

**功能：**
- async send 方法
- 支持 7 种 HTTP 方法
- Headers 处理
- Query params 处理
- Body 支持 (JSON/Text/FormData)
- 响应时间统计
- 错误处理

### 16:15 - 创建 Tauri Commands

**文件：** `src/lib.rs`

**添加：**
```rust
#[tauri::command]
async fn send_http_request(request: HttpRequest) -> Result<HttpResponse, ResponseError>
```

### 16:30 - 构建验证

**问题：**
- 缺少 OpenSSL 开发库
- main.rs 中库名未更新

**解决：**
- 安装 libssl-dev, pkg-config
- 修复 main.rs: another_man_tmp_lib → another_man_lib

**结果：** ✅ cargo check passed

### 16:45 - 提交代码

**提交：** `feat(week1): add HTTP client backend with reqwest`

**变更：**
- 10 files changed
- 5971 insertions(+), 35 deletions(-)

---

## 下一步

Week 1 剩余任务：
- [ ] 1.1.5 实现基础请求构建器 UI

---