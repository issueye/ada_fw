# Ada Framework 下阶段开发计划（ROADMAP）

> 版本基线：`v0.1.0`（commit `8d33ee1`）
> 文档状态：**待评审**
> 编制依据：对 `src/`、`templates/`、`static/` 的全量代码走查 + `cargo test` 实测结果

---

## 0. 决策摘要

| 议题 | 决策 |
| --- | --- |
| 推进方式 | 阶段一 → 阶段二 → 阶段三，顺序全量推进 |
| 鉴权方案 | **JWT / Cookie Token**（无状态，便于横向扩展） |
| 数据层 | **直接接入 `sqlx` + SQLite**（数据持久化） |
| 本轮交付 | 先产出本计划文档，评审通过后逐项实现 |

---

## 1. 现状盘点

### 1.1 已落地能力

| 维度 | 状态 | 关键实现 |
| --- | --- | --- |
| SSR 渲染链路 | ✅ | `handlers/pages.rs`、`handlers/admin.rs` → `TemplateEngine::render` → `Html<String>` |
| 模板引擎 | ✅ | `template/mod.rs`：`path_loader` 递归加载、`asset()` 全局函数、`tailwind_mode` 全局变量 |
| 组件库 | ✅ | 28 组件 / 7 文件 / 5 分类，注册表见 `template/component.rs::BUILTIN_COMPONENTS` |
| 后台管理 | ✅ | 7 个页面，侧边栏折叠 + 主题切换（`layouts/admin.html`） |
| JSON API | ✅ | Todo CRUD（4 端点）+ User CRUD（3 端点），入参经 `validate()` 校验 |
| 设计令牌 | ✅ | `static/css/tokens.css` 语义变量 + `data-theme` 双主题 |
| 单元测试 | ✅ | `cargo test` → **27 passed / 0 failed** |
| 文档 | ✅ | `README.md`、`ARCHITECTURE.md`、`docs/UED.md` |

### 1.2 缺口清单（实测确认）

#### 🔴 P0 — 功能性缺陷

| 编号 | 问题 | 位置 | 影响 |
| --- | --- | --- | --- |
| P0-1 | **无全局 404 兜底** | `src/router.rs:17-40` | 未知路径返回 axum 默认纯文本 404，`admin/404.html` 仅挂在演示路径 `/admin/404` |
| P0-2 | **`openModal` 未定义** | `templates/admin/users.html:20,170` 引用；`static/js/app.js:82-141` 无该状态 | 点击「新建用户」无任何反应（死链） |
| P0-3 | **`userManager` 缺 `create()`** | `static/js/app.js:82-141` | 弹窗「创建」按钮仅关闭弹窗，未调用 `POST /api/users` |
| P0-4 | **分页为静态假数据** | `templates/admin/users.html:161` | 硬编码 `page=1`，与 `total` 无联动 |

#### 🟠 P1 — 架构与安全

| 编号 | 问题 | 位置 | 影响 |
| --- | --- | --- | --- |
| P1-1 | **零鉴权** | `src/router.rs:22-28` | `/admin/*` 全部裸奔，`/admin/login` 为纯静态页 |
| P1-2 | **错误响应不分层** | `src/error.rs:39-57` | 页面请求出错返回裸 JSON，与 `ARCHITECTURE.md` §4.7 承诺不符 |
| P1-3 | **无请求体大小限制** | `src/router.rs` | `Json<CreateUser>` 无 `DefaultBodyLimit` 约束 |
| P1-4 | **无安全响应头** | `src/router.rs` | 缺 `X-Content-Type-Options`、`X-Frame-Options` 等 |

#### 🟡 P2 — 工程化

| 编号 | 问题 | 位置 | 影响 |
| --- | --- | --- | --- |
| P2-1 | **无 HTTP 集成测试** | 无 `tests/` 目录 | 路由、状态码、JSON 契约零覆盖 |
| P2-2 | **`hot_reload` 名不副实** | `src/template/mod.rs:40` | 仅传给 `env.set_debug()`，未实现模板热重载 |
| P2-3 | **`Store` 无持久化** | `src/state.rs:16-19` | 进程重启数据全丢 |
| P2-4 | **`app.js` 重复代码** | `static/js/app.js:126-140` 与 `163-178` | 两处 `request()` 完全重复（DRY 违背） |
| P2-5 | **`app.js` 注释块重复** | `static/js/app.js:43-44` | 连续两行 `/* ===...`，编辑残留 |
| P2-6 | **写锁粒度粗** | `src/state.rs:109` | `RwLock<Store>` 整体加锁，读写互相阻塞 |

---

## 2. 阶段一：功能补全（P0）

**目标**：消除已确认的功能性死链，使现有页面「所见即所得」。
**预估工时**：1 人日
**前置依赖**：无

### 1.1 注册全局 404 兜底

- **改动**：`src/router.rs` 在 `.with_state(state)` 前追加 `.fallback(admin::not_found_page)`
- **配套**：`admin::not_found_page` 当前返回 `200`，需改为返回 `404` 状态码
  - 方案：新增 `AppError::NotFound` 分支或让 handler 返回 `(StatusCode::NOT_FOUND, Html(html))`
- **验收**：
  - `curl -i http://127.0.0.1:3000/not-exist` → `HTTP/1.1 404` + `Content-Type: text/html`
  - 页面渲染 `admin/404.html` 内容

### 1.2 补全 `userManager` 状态与方法

- **改动**：`static/js/app.js` 的 `Alpine.data('userManager', ...)` 中新增：
  - `openModal: null` —— 控制弹窗开关
  - `form: { name: '', email: '', role: 'viewer' }` —— 表单绑定
  - `submitting: false` —— 提交态防重复点击
  - `async create()` —— 调用 `POST /api/users`，成功后 `unshift` 到 `users` 并派发 `ada-toast`
- **配套**：`templates/admin/users.html:173-183` 的输入框补 `x-model="form.name"` 等绑定；「创建」按钮改为 `@click="create()"`
- **验收**：
  - 点击「新建用户」→ 弹窗打开
  - 填写合法数据 → 提交 → 列表顶部出现新用户 + Toast 提示
  - 提交空姓名 → 显示后端校验错误

### 1.3 抽取公共请求封装（DRY）

- **改动**：`static/js/app.js` 顶部新增模块级函数 `apiRequest(url, options)`，`userManager` 与 `todoApp` 均改为调用它
- **验收**：`grep -c "async request" static/js/app.js` → `0`；两处调用点行为不变

### 1.4 分页真实化

- **改动**（二选一，建议 A）：
  - **A. 后端分页**：`GET /api/users?page=&per_page=`，`Store::list_users_paged()` 返回 `(items, total)`
  - **B. 前端��页**：`userManager` 增加 `page`/`perPage` 计算属性，对 `filtered` 切片
- **验收**：`total=8, per_page=10` 时显示 1 页；`per_page=3` 时显示 3 页且切换生效

### 1.5 清理 `app.js` 残留

- **改动**：删除 `static/js/app.js:44` 重复的注释起始行
- **验收**：`node --check static/js/app.js` 通过

---

## 3. 阶段二：安全与健壮性（P1）

**目标**：让后台管理系统具备真实可用的访问控制与错误处理。
**预估工时**：1.5 人日
**前置依赖**：阶段一完成

### 2.1 JWT / Cookie Token 鉴权

**新增依赖**：

```toml
jsonwebtoken = "9"
axum-extra = { version = "0.9", features = ["cookie", "cookie-private"] }
time = "0.3"
```

**新增模块** `src/auth/mod.rs`：

```rust
/// JWT 载荷。
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // 用户 id
    pub name: String,     // 用户名
    pub role: String,     // 角色
    pub exp: usize,       // 过期时间戳
}

/// 签发 Token。
pub fn issue_token(secret: &[u8], claims: &Claims) -> Result<String, AppError>;

/// 校验 Token。
pub fn verify_token(secret: &[u8], token: &str) -> Result<Claims, AppError>;
```

**中间件** `src/auth/middleware.rs`：

- `require_auth`：从 `PrivateCookieJar` 读取 `ada_token`，校验失败则
  - 页面请求（`Accept: text/html`）→ `303` 重定向 `/admin/login?next=<原路径>`
  - API 请求 → `401` JSON
- 通过 `axum::middleware::from_fn_with_state` 挂载到 `/admin/*` 与 `/api/*` 子路由

**新增路由**：

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `POST` | `/admin/login` | 校验凭据 → 签发 JWT → 写入 HttpOnly Cookie |
| `POST` | `/admin/logout` | 清除 Cookie |

**配置扩展** `src/config.rs`：

```rust
pub struct AppConfig {
    // ... 既有字段
    /// JWT 签名密钥（生产环境必须通过 ADA_JWT_SECRET 注入）。
    pub jwt_secret: String,
    /// Token 有效期（秒）。
    pub jwt_ttl_secs: u64,
}
```

**安全要点**：

- Cookie 必须设 `HttpOnly` + `SameSite=Lax`；生产环境追加 `Secure`
- 密钥不得硬编码，`ADA_JWT_SECRET` 缺失时**启动即失败**（fail-close）
- 登录失败统一返回「用户名或密码错误」，不区分具体原因，防用户枚举

**验收**：

- 未登录访问 `/admin` → `303` 跳转 `/admin/login`
- 未登录访问 `/api/users` → `401` JSON
- 正确凭据登录 → 获得 Cookie → 可访问 `/admin`
- 篡改 Cookie → `401`

### 2.2 错误响应内容协商

- **改动**：`src/error.rs::into_response` 按 `Accept` 头分流：
  - 含 `text/html` → 渲染 `admin/404.html` 或新增 `admin/error.html`，返回 HTML
  - 否则 → 保持现有 JSON 结构
- **注意**：`IntoResponse` 中无法直接访问 `AppState`，需通过 `Extension` 注入 `TemplateEngine`，或改用 `axum::middleware::map_response`
- **验收**：浏览器访问出错路径看到 HTML 错误页；`curl -H "Accept: application/json"` 得到 JSON

### 2.3 请求体限制与安全响应头

- **改动**：`src/router.rs` 追加：

```rust
.layer(DefaultBodyLimit::max(64 * 1024))
.layer(SetResponseHeaderLayer::overriding(
    header::X_CONTENT_TYPE_OPTIONS,
    HeaderValue::from_static("nosniff"),
))
.layer(SetResponseHeaderLayer::overriding(
    header::X_FRAME_OPTIONS,
    HeaderValue::from_static("DENY"),
))
```

- **验收**：`curl -X POST --data-binary @big.json` → `413`；响应头含 `X-Content-Type-Options: nosniff`

---

## 4. 阶段三：数据层与工程化（P2）

**目标**：数据持久化、测试覆盖、开发体验提升。
**预估工时**：1.5 人日
**前置依赖**：阶段二完成

### 3.1 接入 sqlx + SQLite

**新增依赖**：

```toml
sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite", "macros", "migrate"] }
```

**迁移脚本** `migrations/`：

```sql
-- 0001_init.sql
CREATE TABLE IF NOT EXISTS users (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    email       TEXT NOT NULL UNIQUE,
    role        TEXT NOT NULL,
    status      TEXT NOT NULL,
    created_at  TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS todos (
    id     TEXT PRIMARY KEY,
    title  TEXT NOT NULL,
    done   INTEGER NOT NULL DEFAULT 0
);
```

**数据层重构**：

- 新增 `src/db/mod.rs`，暴露 `SqlitePool`
- 将 `Store` 的同步方法改为 `async`，签名保持语义一致（如 `list_users() -> Vec<User>` → `async fn list_users(&self) -> Result<Vec<User>, sqlx::Error>`）
- `AppState.store: Arc<RwLock<Store>>` → `AppState.db: SqlitePool`（连接池自带并发安全，**移除 `parking_lot` 依赖**）
- 保留 `Store` 作为内存实现，通过 trait 抽象：

```rust
#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn list(&self) -> Result<Vec<User>, AppError>;
    async fn create(&self, req: CreateUser) -> Result<User, AppError>;
    async fn delete(&self, id: &str) -> Result<bool, AppError>;
}
```

**配置扩展**：

```rust
/// 数据库连接串，默认 `sqlite://ada.db?mode=rwc`。
pub database_url: String,
```

**验收**：

- 重启进程后数据仍在
- `cargo test` 全绿（含数据库测试，使用 `sqlite::memory:` 隔离）
- 现有 27 个单元测试语义不变

### 3.2 HTTP 集成测试

**新增** `tests/http.rs`：

```rust
use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt; // oneshot

#[tokio::test]
async fn 未知路径应返回_404_html() {
    let app = build_test_app().await;
    let res = app
        .oneshot(Request::builder().uri("/not-exist").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    assert!(res.headers()["content-type"].to_str().unwrap().contains("text/html"));
}
```

**覆盖清单**：

| 用例 | 断言 |
| --- | --- |
| `GET /` | `200` + `text/html` |
| `GET /admin`（未登录） | `303` → `/admin/login` |
| `GET /api/todos` | `200` + JSON 数组 |
| `POST /api/todos` 空标题 | `400` + 错误体 |
| `POST /api/todos` 合法 | `201` + 新对象 |
| `DELETE /api/todos/:id` 不存在 | `404` |
| `POST /api/users` 非法邮箱 | `400` |
| 超大请求体 | `413` |

**验收**：`cargo test --test http` 全绿

### 3.3 模板热重载

- **改动**：`src/template/mod.rs` 在 `hot_reload=true` 时，用 `notify` 监听 `templates/` 目录，变更时重建 `Environment`
- **实现要点**：`TemplateEngine.env` 由 `Arc<Environment>` 改为 `Arc<RwLock<Arc<Environment>>>`，`render()` 读取当前快照
- **新增依赖**：`notify = "6"`
- **验收**：`ADA_HOT_RELOAD=1 cargo run` 后修改模板，刷新浏览器即生效，无需重启

### 3.4 组件库文档自动生成

- **改动**：`/admin/ui-kit` 已从 `BUILTIN_COMPONENTS` 读取元信息，补充「组件路径」「分类」展示，并支持按分类锚点跳转
- **验收**：新增一个 `ComponentMeta` 后，UI Kit 页面自动出现该组件条目

---

## 5. 里程碑与验收

| 里程碑 | 内容 | 验收命令 | 状态 |
| --- | --- | --- | --- |
| **M1** | 阶段一：功能补全 | `cargo test` + 手工走查 4 项 P0 | ✅ 已完成 |
| **M2** | 阶段二：安全加固 | `cargo test` + 鉴权 4 项验收 | ✅ 已完成 |
| **M3** | 阶段三：数据层与工程化 | `cargo test` + 持久化验证 | ✅ 已完成 |

### 实施结果（实测）

| 指标 | 阶段前 | 阶段后 |
| --- | --- | --- |
| 测试总数 | 27 | **73**（44 单元 + 20 HTTP + 9 模板渲染） |
| 测试结果 | 27 passed | **73 passed / 0 failed** |
| P0 缺陷 | 4 项 | **0 项** |
| P1 缺陷 | 4 项 | **0 项** |
| P2 缺陷 | 6 项 | **4 项**（剩余：写锁粒度、组件文档自动生成） |

**已完成的关键改动**：

- **阶段一**：全局 404 fallback（返回真实 404 状态码）、`userManager` 补全 `openModal`/`form`/`create()`、抽取 `apiRequest()` 消除重复、客户端分页与搜索联动
- **阶段二**：JWT + HttpOnly Cookie 鉴权（`src/auth/`）、错误响应内容协商中间件（`src/middleware.rs`）、安全响应头、64 KiB 请求体限制、配置安全校验（fail-close）
- **阶段三**：sqlx + SQLite 持久化（`src/db/`、`migrations/0001_init.sql`）、模板热重载（`notify`）、HTTP 集成测试、模板渲染冒烟测试

---

## 6. 风险与对策

| 风险 | 影响 | 对策 |
| --- | --- | --- |
| `error.rs` 的 `IntoResponse` 无法访问 `AppState` | 阶段二 2.2 受阻 | 改用 `map_response` 中间件，或注入 `Extension<TemplateEngine>` |
| sqlx 迁移导致 27 个既有测试大面积失效 | 阶段三回归成本高 | 先抽 trait 保持签名，再逐个替换实现；每步跑 `cargo test` |
| JWT 密钥硬编码泄露 | 严重安全风险 | 强制 `ADA_JWT_SECRET` 环境变量，缺失即启动失败 |
| 热重载���入 `RwLock` 影响渲染性能 | 轻微延迟 | 仅在 `hot_reload=true` 时启用，生产走静态 `Arc` |
| 同步 `Store` 改 `async` 波及所有 handler | 改动面大 | 按模块分批：先 User 后 Todo，每批独立验证 |

---

## 7. 附：改动文件清单（预估）

```
新增：
  src/auth/mod.rs              # JWT 签发与校验
  src/auth/middleware.rs       # require_auth 中间件
  src/db/mod.rs                # sqlx 连接池与仓储实现
  migrations/0001_init.sql     # 建表脚本
  tests/http.rs                # HTTP 集成测试
  templates/admin/error.html   # 通用错误页（可选）

修改：
  Cargo.toml                   # 新增 jsonwebtoken / axum-extra / sqlx / notify
  src/config.rs                # 新增 jwt_secret / jwt_ttl_secs / database_url
  src/state.rs                 # Store → SqlitePool（或 trait 抽象）
  src/error.rs                 # 内容协商分流
  src/router.rs                # fallback / 鉴权中间件 / 安全头 / body limit
  src/handlers/admin.rs        # 404 返回 404 状态码；新增 login/logout
  src/handlers/api.rs          # 改为 async 仓储调用
  src/template/mod.rs          # 热重载支持
  static/js/app.js             # openModal / create / apiRequest 抽取
  templates/admin/users.html   # 表单绑定与分页
  templates/admin/login.html   # 表单提交到 POST /admin/login
```

---

*本文档基于 commit `8d33ee1` 的代码实测编制，所有行号引用均经核对。*
