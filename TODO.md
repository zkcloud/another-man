# TODO.md - Another Man 项目任务追踪

> 使用原则：当此文件更新时，重新评估并调整工作计划

---

## 当前阶段：Phase 1 - MVP (4周) ✅ Week 1-2 完成

### Week 1: 基础架构 + HTTP 请求 ✅ 完成
- ✅ 后端依赖 (reqwest, rusqlite)
- ✅ 数据模型 (HttpRequest, HttpResponse)
- ✅ HTTP 客户端 (async send)
- ✅ Tauri Commands
- ✅ 前端请求构建器 UI

### Week 2: 响应展示 + 数据持久化 ✅ 完成
- ✅ JSON/XML/HTML 响应格式化
- ✅ Headers/Cookies 展示
- ✅ SQLite 数据库初始化
- ✅ 历史记录存储
- ✅ 历史记录列表展示

### Week 3: 集合管理 + 环境变量 🔄 进行中

| # | 任务 | 状态 | 验收条件 |
|---|------|------|---------|
| 1.3.1 | Collection CRUD API | ⏳ 待执行 | 增删改查集合 |
| 1.3.2 | Request CRUD API | ⏳ 待执行 | 增删改查请求 |
| 1.3.3 | 集合树形展示 UI | ⏳ 待执行 | 树形结构展示 |
| 1.3.4 | 环境变量系统 | ⏳ 待执行 | 变量注入 {{var}} |
| 1.3.5 | 变量选择器 UI | ⏳ 待执行 | 下拉选择环境 |

### Week 4: 完善与优化 ⏳ 待开始
- 导入/导出功能
- 性能优化
- Bug 修复

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
- [x] HTTP 客户端后端
- [x] 请求构建器 UI

### Week 2 ✅
- [x] 响应格式化 (JSON/XML/HTML)
- [x] ResponseViewer 组件
- [x] SQLite 数据库
- [x] 历史记录存储和展示

---

最后更新：2026-03-22 17:00