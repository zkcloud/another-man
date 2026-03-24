# Another Man

HTTP API 客户端工具，支持 Postman 类似的界面和功能。

## 功能特性

- ✅ **HTTP 请求**: GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS
- ✅ **认证**: Basic Auth, Bearer Token, API Key, OAuth2
- ✅ **环境变量**: 多环境支持，变量替换
- ✅ **脚本引擎**: Pre-request Script, Test Script
- ✅ **断言库**: 状态码、Header、Body、JSON 断言
- ✅ **变量提取**: JSONPath, Regex, Header 提取
- ✅ **批量执行**: Collection Runner 支持迭代和延迟
- ✅ **依赖管理**: 请求依赖拓扑排序
- ✅ **测试套件**: 测试用例管理和运行
- ✅ **报告生成**: HTML/JSON 测试报告
- ✅ **CLI 模式**: 命令行支持 CI/CD

## 技术栈

- **前端**: Vue 3 + Vite + TypeScript
- **后端**: Rust + Tauri
- **数据库**: SQLite (rusqlite)

## 开发

### 前端
```bash
npm install
npm run dev
```

### 后端
```bash
cd src-tauri
cargo build
```

### 测试
```bash
cd src-tauri
cargo test --lib
```

## CLI 用法

```bash
# 查看状态
another-man status

# 运行 Collection
another-man run <collection-id>

# 生成报告
another-man report <suite-id> --format html
```

## 项目结构

```
another-man/
├── src/                    # Vue 前端
│   ├── components/         # Vue 组件
│   ├── api/               # API 调用
│   └── App.vue            # 主应用
├── src-tauri/             # Rust 后端
│   └── src/
│       ├── core/          # 核心模块
│       ├── db/            # 数据库
│       ├── models/        # 数据模型
│       └── script/         # 脚本引擎
└── README.md
```

## License

MIT
