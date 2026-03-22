# TODO.md - Another Man 项目任务追踪

> 使用原则：当此文件更新时，重新评估并调整工作计划

---

## 当前阶段：Phase 1 - MVP (4周) ✅ Week 1 完成

### Week 1: 基础架构 + HTTP 请求 ✅ 完成

| # | 任务 | 状态 | 验收条件 |
|---|------|------|---------|
| 1.1.1 | 添加后端依赖 (reqwest, rusqlite) | ✅ 完成 | Cargo.toml 更新 |
| 1.1.2 | 创建数据模型 (Request, Response) | ✅ 完成 | Rust 结构体定义 |
| 1.1.3 | 实现 HTTP 客户端核心 | ✅ 完成 | 能发送 GET/POST |
| 1.1.4 | 创建 Tauri Commands | ✅ 完成 | 前端可调用的 API |
| 1.1.5 | 实现基础请求构建器 UI | ✅ 完成 | URL输入 + Method选择 |

### Week 2: 响应展示 + 数据持久化 🔄 进行中

| # | 任务 | 状态 | 验收条件 |
|---|------|------|---------|
| 1.2.1 | JSON/XML/HTML 响应格式化 | ⏳ 待执行 | 响应正确格式化 |
| 1.2.2 | Headers/Cookies 展示 | ⏳ 待执行 | 响应头表格展示 |
| 1.2.3 | SQLite 数据库初始化 | ⏳ 待执行 | 数据库连接成功 |
| 1.2.4 | 历史记录存储 | ⏳ 待执行 | 请求自动保存 |
| 1.2.5 | 历史记录列表展示 | ⏳ 待执行 | 侧边栏历史列表 |

---

## 待确认事项

暂无

---

## 已完成

### Phase 0 ✅
- [x] 创建 GitHub 仓库
- [x] 推送设计文档
- [x] 初始化 Tauri + Vue 3 项目
- [x] 创建 dev 分支
- [x] 创建 Phase 1-4 Issues
- [x] 设置 CI/CD

### Week 1 ✅
- [x] 后端依赖 (reqwest, rusqlite, tokio, chrono, uuid, anyhow)
- [x] 数据模型 (HttpRequest, HttpResponse, HttpMethod, Header, Param, RequestBody)
- [x] HTTP 客户端 (async send, 支持 7 种方法, headers/query/body)
- [x] Tauri Command (send_http_request)
- [x] 前端 API 封装 (src/api/http.js)
- [x] RequestBuilder 组件 (URL, Method, Headers, Body, Response)

---

最后更新：2026-03-22 16:15