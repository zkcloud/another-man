# Postman Local - 完整设计文档

> 使用 Rust + Tauri 2.0 构建的本地化 API 测试工具，复刻 Postman 核心功能，去除云同步依赖，保留 Git 版本控制，追求极致本地响应速度。

---

## 一、Postman 深度洞察

### 1.1 功能全景分析

Postman 功能层次：
- **协作层** → 去除，替换为 Git
- **自动化层** → 本地实现
- **扩展层** → 本地 Mock + Markdown
- **核心层** → 完全保留
- **数据层** → SQLite + 文件系统

### 1.2 界面交互深度分析

**关键交互模式：**
1. 三栏布局：Sidebar + Main + Response
2. Tab 导航：Params/Auth/Headers/Body/Scripts
3. 键值对编辑器：支持批量编辑
4. 响应分栏：Body/Cookies/Headers/Test Results
5. 环境切换器

### 1.3 后台架构痛点分析

| 问题 | 影响 | 本地化解决方案 |
|------|------|---------------|
| Electron 臃肿 | 300MB+ 内存 | Tauri 2.0 (<50MB) |
| 云端同步阻塞 | 离线不可用 | 本地 SQLite |
| 网络依赖 | 弱网体验差 | 零网络依赖 |
| 复杂状态管理 | Redux 层层传递 | Rust 端集中管理 |

### 1.4 生态对比

本方案定位：极致本地、Rust 性能、Git 原生

---

## 二、本地化设计策略

### 2.1 云功能替代方案

| Postman 云功能 | 替代方案 |
|---------------|---------|
| Cloud Sync | Git 版本控制 |
| Cloud History | 本地 SQLite |
| Cloud Mock Server | 本地 Axum Mock |
| Cloud Documentation | Markdown 导出 |

### 2.2 性能优化策略

三层架构：前端层(虚拟滚动) → 通信层(批量传输) → 后端层(连接池)

---

## 三、技术架构详设

### 3.1 技术栈

**前端：** Vue 3 + TailwindCSS + Monaco Editor
**后端：** Rust + Tauri 2.0 + SQLite + QuickJS

### 3.2 核心依赖

```toml
reqwest = { version = "0.12", features = ["json", "multipart"] }
rusqlite = { version = "0.32", features = ["bundled"] }
rquickjs = "0.6"
git2 = "0.19"
axum = "0.7"
tokio = { version = "1.40", features = ["full"] }
```

### 3.3 目录结构

```
postman-local/
├── src/
│   ├── commands/      # Tauri Commands
│   ├── core/          # 核心业务逻辑
│   ├── models/        # 数据模型
│   └── utils/         # 工具函数
├── frontend/          # Vue 3 前端
└── docs/              # 文档
```

---

## 四、数据模型设计

### 4.1 核心实体

**Collection** - 集合
- id, name, description, parent_id
- variables, sort_order
- created_at, updated_at

**Request** - 请求
- id, method, url
- headers, query_params, path_params
- body, auth, scripts

**Environment** - 环境
- id, name, variables
- is_global

### 4.2 文件存储结构

```
~/.postman-local/
├── config.json
├── database.sqlite
├── collections/
├── environments/
└── scripts/
```

---

## 五、功能模块设计

### 5.1 请求构建器
- 智能 URL 补全
- Method 快速切换
- Headers 自动补全
- Body 编辑器（支持多种格式）
- 变量注入 `{{variable}}`

### 5.2 响应展示
- Pretty/Raw/Preview 三种模式
- JSON/XML/HTML 格式化
- Cookie 管理器
- 响应时间/大小统计

### 5.3 脚本引擎 (QuickJS)

**Pre-request Script：**
```javascript
pm.environment.set("timestamp", Date.now());
```

**Test Script：**
```javascript
pm.test("Status is 200", () => {
    pm.response.to.have.status(200);
});
```

### 5.4 本地 Mock 服务
- 基于 Axum 的本地 HTTP 服务器
- 支持动态响应
- 延迟模拟

---

## 六、Git 同步方案

### 6.1 工作流

```
Collections/ → Git Repo → Remote (GitHub/GitLab)
     ↑
   Commit/Push/Pull
```

### 6.2 特性
- 集合导出为 JSON
- 内置 Git 操作
- 变更可视化

---

## 七、开发路线图

### Phase 1: MVP (4周)
- [ ] 基础 HTTP 请求/响应
- [ ] 集合管理
- [ ] 环境变量
- [ ] 历史记录

### Phase 2: 核心功能 (4周)
- [ ] 脚本引擎 (QuickJS)
- [ ] 认证方式 (OAuth2/JWT等)
- [ ] 文件上传/下载
- [ ] 导入/导出

### Phase 3: 高级功能 (4周)
- [ ] Git 同步
- [ ] Mock 服务
- [ ] Collection Runner
- [ ] 文档生成

### Phase 4: 优化 (2周)
- [ ] 性能优化
- [ ] 插件系统
- [ ] 主题定制

---

## 八、关键技术决策

### 8.1 脚本引擎选择
**选择 QuickJS 而非 Deno Core：**
- 更轻量 (<1MB vs >30MB)
- 启动更快
- 内存占用更低
- 足够满足测试脚本需求

### 8.2 数据库选择
**SQLite 而非 PostgreSQL/MySQL：**
- 零配置
- 单文件存储
- 性能足够
- 易于备份

### 8.3 HTTP 客户端
**Reqwest 而非 Hyper 裸用：**
- 更友好的 API
- 内置连接池
- 支持 HTTP/2
- 成熟的生态系统

---

## 九、性能目标

| 指标 | Postman | 本方案目标 |
|------|---------|-----------|
| 启动时间 | 3-5s | <1s |
| 内存占用 | 300MB+ | <50MB |
| 请求响应 | 依赖网络 | 本地零延迟 |
| 大数据渲染 | 卡顿 | 虚拟滚动流畅 |

---

文档完成时间: 2026-03-21
