# Another Man API 文档

## Tauri Commands

### HTTP 请求

#### send_http_request
发送 HTTP 请求并获取响应。

```rust
#[tauri::command]
async fn send_http_request(request: HttpRequest) -> Result<HttpResponse, ResponseError>
```

**参数:**
- `request: HttpRequest` - HTTP 请求配置

**返回:** `HttpResponse` - HTTP 响应

---

### Collections

#### create_collection
创建新的 Collection。

```rust
#[tauri::command]
fn create_collection(
    name: String,
    description: Option<String>,
    parent_id: Option<String>,
) -> Result<Collection, String>
```

#### get_collections
获取所有 Collections。

```rust
#[tauri::command]
fn get_collections(state: tauri::State<'_, AppState>) -> Result<Vec<Collection>, String>
```

#### delete_collection
删除 Collection。

```rust
#[tauri::command]
fn delete_collection(id: String, state: tauri::State<'_, AppState>) -> Result<(), String>
```

---

### Saved Requests

#### create_saved_request
保存请求到 Collection。

```rust
#[tauri::command]
fn create_saved_request(
    request: CreateRequestInput,
    state: tauri::State<'_, AppState>
) -> Result<SavedRequest, String>
```

#### get_requests_by_collection
获取 Collection 中的所有请求。

```rust
#[tauri::command]
fn get_requests_by_collection(
    collection_id: String,
    state: tauri::State<'_, AppState>
) -> Result<Vec<SavedRequest>, String>
```

---

### Environments

#### create_environment
创建环境。

```rust
#[tauri::command]
fn create_environment(
    name: String,
    variables: HashMap<String, String>,
    is_global: bool,
    state: tauri::State<'_, AppState>
) -> Result<Environment, String>
```

#### get_environments
获取所有环境。

```rust
#[tauri::command]
fn get_environments(state: tauri::State<'_, AppState>) -> Result<Vec<Environment>, String>
```

---

### Test Suites

#### create_test_suite
创建测试套件。

```rust
#[tauri::command]
fn create_test_suite(
    name: String,
    description: Option<String>,
    collection_id: Option<String>,
    state: tauri::State<'_, AppState>
) -> Result<TestSuite, String>
```

#### get_test_suites
获取所有测试套件。

```rust
#[tauri::command]
fn get_test_suites(state: tauri::State<'_, AppState>) -> Result<Vec<TestSuite>, String>
```

#### get_test_cases
获取测试套件中的所有测试用例。

```rust
#[tauri::command]
fn get_test_cases(
    suite_id: String,
    state: tauri::State<'_, AppState>
) -> Result<Vec<TestCase>, String>
```

#### get_test_runs
获取测试套件的运行记录。

```rust
#[tauri::command]
fn get_test_runs(
    suite_id: String,
    state: tauri::State<'_, AppState>
) -> Result<Vec<TestRun>, String>
```

---

### Test Reports

#### generate_test_report
生成测试报告。

```rust
#[tauri::command]
fn generate_test_report(
    suite_id: String,
    format: String,
    state: tauri::State<'_, AppState>
) -> Result<String, String>
```

**参数:**
- `suite_id: String` - 测试套件 ID
- `format: String` - 报告格式 ("html" 或 "json")

**返回:** 报告内容字符串

---

### Variable Extraction

#### extract_variables
从响应中提取多个变量。

```rust
#[tauri::command]
fn extract_variables(
    body: String,
    headers: HashMap<String, String>,
    extractions: Vec<String>,
) -> Result<HashMap<String, String>, String>
```

**参数:**
- `body: String` - 响应体 (JSON)
- `headers: HashMap<String, String>` - 响应头
- `extractions: Vec<String>` - 提取规则，如 `"userId:jsonPath:data.user.id"`

**返回:** 提取的变量 Map

#### extract_json_path
从 JSON 中提取值。

```rust
#[tauri::command]
fn extract_json_path(body: String, path: String) -> Result<Option<String>, String>
```

**参数:**
- `body: String` - JSON 响应
- `path: String` - JSONPath 路径，如 `"data.user.name"`

#### extract_regex
使用正则表达式提取值。

```rust
#[tauri::command]
fn extract_regex(body: String, pattern: String, group: usize) -> Result<Option<String>, String>
```

---

### Request Dependencies

#### add_request_dependency
添加请求依赖。

```rust
#[tauri::command]
fn add_request_dependency(
    request_id: String,
    depends_on_request_id: String,
    state: tauri::State<'_, AppState>
) -> Result<RequestDependency, String>
```

#### get_request_dependencies
获取请求的依赖。

```rust
#[tauri::command]
fn get_request_dependencies(
    request_id: String,
    state: tauri::State<'_, AppState>
) -> Result<Vec<RequestDependency>, String>
```

---

### CLI / Headless

#### get_headless_status
获取 headless 模式状态。

```rust
#[tauri::command]
fn get_headless_status() -> String
```

**返回:** JSON 格式的状态信息

---

## 数据模型

### HttpRequest
```rust
pub struct HttpRequest {
    pub id: String,
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<Header>,
    pub query_params: Vec<Param>,
    pub body: Option<RequestBody>,
}
```

### HttpMethod
```rust
pub enum HttpMethod {
    GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS
}
```

### TestSuite
```rust
pub struct TestSuite {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub collection_id: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}
```

### TestCase
```rust
pub struct TestCase {
    pub id: String,
    pub suite_id: String,
    pub name: String,
    pub pre_script: Option<String>,
    pub test_script: Option<String>,
    pub enabled: bool,
}
```

---

## 断言方法

### Status Code Assertions
- `pm.response.to.have.status(code)` - 状态码等于指定值
- `pm.response.to.be.success` - 状态码 2xx

### Header Assertions
- `pm.response.to.have.header(name)` - Header 存在
- `pm.response.to.have.header(name).eql(value)` - Header 等于指定值

### Body Assertions
- `pm.response.to.have.body` - Body 包含指定内容
- `pm.response.to.not.be.empty` - Body 不为空

### JSON Assertions
- `pm.response.json().to.have.property(path)` - JSON 属性存在
- `pm.response.json().to.eql(expected)` - JSON 等于预期值
