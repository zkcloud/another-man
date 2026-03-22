# TODO.md - Another Man 项目任务追踪

> 使用原则：当此文件更新时，重新评估并调整工作计划

---

## 当前阶段：Phase 1 - MVP (4周) ✅ 全部完成

### Week 1: 基础架构 + HTTP 请求 ✅ 完成
### Week 2: 响应展示 + 数据持久化 ✅ 完成
### Week 3: 集合管理 + 环境变量 ✅ 完成
### Week 4: 完善与优化 ✅ 完成

| # | 任务 | 状态 | 验收条件 |
|---|------|------|---------|
| 1.4.1 | 导出 Collection 为 JSON | ✅ 完成 | 文件下载功能 |
| 1.4.2 | 导入 Collection JSON | ✅ 完成 | 粘贴 JSON 导入 |
| 1.4.3 | 性能优化 | ✅ 完成 | 基础优化 |
| 1.4.4 | 错误处理完善 | ✅ 完成 | 友好错误提示 |
| 1.4.5 | 最终测试与文档 | ✅ 完成 | README 更新 |

---

## Phase 1 MVP 功能清单

### ✅ 核心功能
- [x] HTTP 请求发送 (GET/POST/PUT/DELETE/PATCH/HEAD/OPTIONS)
- [x] Headers 编辑
- [x] Body 编辑 (JSON/Text)
- [x] 响应格式化 (JSON/XML/HTML)
- [x] 响应状态/时间/大小显示
- [x] Headers/Cookies 展示

### ✅ 数据管理
- [x] SQLite 本地数据库
- [x] 历史记录自动保存
- [x] 历史记录列表展示

### ✅ 集合管理
- [x] Collection CRUD
- [x] Request CRUD
- [x] 树形结构展示
- [x] 导出为 JSON
- [x] 从 JSON 导入

### ✅ 环境变量
- [x] 环境变量 CRUD
- [x] 变量替换 {{var}}
- [x] 环境选择器
- [x] 全局环境设置

---

## Phase 2 准备

### 待规划
- [ ] 自动化测试 (Pre-request/Post-response 脚本)
- [ ] 批量请求执行
- [ ] 响应断言
- [ ] Git 集成

---

最后更新：2026-03-22 17:50