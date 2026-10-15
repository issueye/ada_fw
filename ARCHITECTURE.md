# Ada Framework 架构设计文档

> 一个基于 **Rust + Minijinja + Tailwind CSS + Alpine.js** 的服务端渲染（SSR）前端框架。

---

## 1. 设计目标

| 目标 | 说明 |
| --- | --- |
| **SSR 优先** | 首屏由服务端渲染完整 HTML，SEO 友好、首屏快 |
| **组件化** | 页面由可复用的「组件模板 + 组件上下文」组合而成 |
| **渐进增强** | Alpine.js 负责客户端交互，不依赖前端构建工具链 |
| **数据分层** | 页面渲染走 HTML，动态数据走 JSON API，职责清晰 |
| **零前端构建** | 开发期 Tailwind 走 CDN，生产期切换本地 CLI 产物 |

### 技术选型

| 层 | 技术 | 理由 |
| --- | --- | --- |
| HTTP 服务 | `axum` | 生态成熟、Tower 中间件体系、异步性能好 |
| 模板引擎 | `minijinja` | 运行时模板、语法接近 Jinja2、支持继承与宏 |
| 序列化 | `serde` / `serde_json` | 事实标准 |
| 交互增强 | `Alpine.js` | 声明式、无需构建、与 SSR 天然契合 |
| 样式 | `Tailwind CSS` | 原子化、按需生成、无运行时开销 |

---

## 2. 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                         浏览器 (Client)                       │
│                                                               │
│   HTML (SSR 首屏)  ──►  Alpine.js 接管交互  ──►  fetch JSON   │
│         ▲                                            │        │
└─────────┼────────────────────────────────────────────┼───────┘
          │ ① 请求页面 (GET /)                          │ ② 请求数据
          │                                             │   (GET /api/*)
┌─────────┴─────────────────────────────────────────────┴───────┐
│                        Ada Framework (Server)                  │
│                                                                │
│  ┌──────────┐   ┌──────────────┐   ┌───────────────────────┐  │
│  │  Router  │──►│  PageHandler │──►│  TemplateEngine       │  │
│  │ (axum)   │   │  (SSR 渲染)  │   │  (minijinja + 组件)   │  │
│  └────┬─────┘   └──────┬───────┘   └───────────────────────┘  │
│       │                │                                       │
│       │                ▼                                       │
│       │         ┌──────────────┐                               │
│       │         │  AppState    │  ← 共享状态 / 配置 / 引擎     │
│       │         └──────────────┘                               │
│       │                                                        │
│       ▼                                                        │
│  ┌──────────────┐   ┌──────────────┐                          │
│  │  ApiHandler  │──►│  Json<T>     │  (JSON API 层)           │
│  └──────────────┘   └──────────────┘                          │
└──────────────────────────────────────────────���─────────────────┘
```

### 请求生命周期

1. **页面请求** `GET /` → `Router` 匹配 → `PageHandler` 组装上下文 → `TemplateEngine` 渲染 → 返回 `text/html`
2. **数据请求** `GET /api/todos` → `Router` 匹配 → `ApiHandler` 查询数据 → 返回 `application/json`
3. **客户端交互** Alpine.js 通过 `fetch` 调用 JSON API，局部更新 DOM

---

## 3. 目录结构

```
ada_fw/
├── Cargo.toml
├── ARCHITECTURE.md              # 本文档
├── README.md                    # 快速上手
├── build.rs                     # 构建期：触发 Tailwind 编译（可选）
├── tailwind.config.js           # Tailwind 配置
├── package.json                 # 仅用于 Tailwind CLI（可选）
├── src/
│   ├── main.rs                  # 入口：启动服务
│   ├── lib.rs                   # 库导出
│   ├── config.rs                # 配置（端口、模板目录、静态资源）
│   ├── state.rs                 # AppState 共享状态
│   ├── error.rs                 # 统一错误类型与响应
│   ├── template/
│   │   ├── mod.rs               # TemplateEngine 封装
│   │   └── component.rs         # 组件注册与渲染
│   ├── router.rs                # 路由注册
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── pages.rs             # SSR 页面处理器
│   │   └── api.rs               # JSON API 处理器
│   └── models/
│       ├── mod.rs
│       └── todo.rs              # 示例领域模型
├── templates/
│   ├── base.html                # 基础布局（含 Tailwind / Alpine 引入）
│   ├── index.html               # 首页
│   ├── components/
│   │   ├── nav.html             # 导航组件
│   │   ├── card.html            # 卡片组件
│   │   └── todo_list.html       # 待办列表组件
│   └── partials/
│       └── head.html            # <head> 片段
└── static/
    ├── css/
    │   └── app.css              # Tailwind 产物（生产）
    └── js/
        └── app.js               # Alpine 组件定义
```

---

## 4. 核心模块设计

### 4.1 配置层 `config.rs`

集中管理运行参数，支持环境变量覆盖：

```rust
pub struct AppConfig {
    pub host: String,          // 监听地址
    pub port: u16,             // 端口
    pub template_dir: PathBuf, // 模板目录
    pub static_dir: PathBuf,   // 静态资源目录
    pub tailwind_mode: TailwindMode, // Cdn | Local
}
```

`TailwindMode` 决定 `base.html` 中注��� CDN 脚本还是本地 CSS 链接，实现「先 CDN、后本地」的平滑切换。

### 4.2 状态层 `state.rs`

`AppState` 通过 `axum::extract::State` 注入到所有 handler：

```rust
pub struct AppState {
    pub config: AppConfig,
    pub engine: TemplateEngine,   // 模板引擎（内部 Arc，可克隆）
    pub store: Arc<RwLock<Store>>, // 示例数据存储
}
```

### 4.3 模板引擎 `template/mod.rs`

对 `minijinja::Environment` 的封装，职责：

- 加载 `templates/` 目录（含子目录递归）
- 注册全局函数（如 `asset()` 生成静态资源路径）
- 注册组件宏
- 提供 `render(name, ctx)` 统一入口

```rust
pub struct TemplateEngine {
    env: Environment<'static>,
}

impl TemplateEngine {
    pub fn new(dir: &Path) -> Result<Self>;
    pub fn render<S: Serialize>(&self, name: &str, ctx: S) -> Result<String>;
}
```

### 4.4 组件系统 `template/component.rs`

采用 **Jinja 宏 + include** ��合实现组件化：

- **宏（macro）**：无状态、纯展示组件，如 `card(title, body)`
- **include + 上下文**：有状态组件，通过 `{% include %}` 传入局部上下文

组件约定：每个组件一个 `.html` 文件，放在 `templates/components/`，通过 `{% import %}` 引入。

### 4.5 路由层 `router.rs`

```rust
pub fn build_router(state: AppState) -> Router {
    Router::new()
        // SSR 页面
        .route("/", get(pages::index))
        .route("/about", get(pages::about))
        // JSON API
        .route("/api/todos", get(api::list_todos).post(api::create_todo))
        .route("/api/todos/:id", delete(api::delete_todo))
        // 静态资源
        .nest_service("/static", ServeDir::new(&state.config.static_dir))
        .with_state(state)
}
```

### 4.6 处理器层 `handlers/`

- **`pages.rs`**：返回 `Html<String>`，负责组装页面上下文并调用引擎渲染
- **`api.rs`**：返回 `Json<T>`，负责数据 CRUD

### 4.7 错误处理 `error.rs`

统一 `AppError`，实现 `IntoResponse`：

```rust
pub enum AppError {
    Template(minijinja::Error),
    NotFound,
    BadRequest(String),
    Internal(String),
}
```

- 页面请求出错 → 渲染错误页
- API 请求出错 → 返回 JSON 错误体

---

## 5. 前端集成方案

### 5.1 Tailwind CSS

**阶段一（开发期）**：CDN 引入，零配置

```html
<script src="https://cdn.tailwindcss.com"></script>
```

**阶段二（生产期）**：本地 CLI 构建

```bash
npx tailwindcss -i ./static/css/input.css -o ./static/css/app.css --minify
```

通过 `AppConfig.tailwind_mode` 切换，`base.html` 中条件渲染：

```jinja
{% if tailwind_mode == "cdn" %}
  <script src="https://cdn.tailwindcss.com"></script>
{% else %}
  <link rel="stylesheet" href="/static/css/app.css">
{% endif %}
```

### 5.2 Alpine.js

- 通过 CDN 引入 `alpinejs`（`defer`）
- 组件逻辑集中在 `static/js/app.js`，用 `Alpine.data()` 注册
- 数据交互统一走 `fetch` 调用 `/api/*`

```html
<div x-data="todoApp()" x-init="load()">
  <template x-for="todo in todos" :key="todo.id">
    <li x-text="todo.title"></li>
  </template>
</div>
```

### 5.3 SSR 与 Alpine 的衔接

- 服务端渲染初始数据到 HTML（首屏可见）
- Alpine 初始化时可选地「接管」这些数据（通过 `x-init` 或内联 JSON）
- 后续交互通过 API 增量更新

---

## 6. 数据流示例（待办列表）

```
① 用户访问 GET /
   └─► pages::index 查询 store → 渲染 index.html（含初始 todos）
       └─► 浏览器显示完整列表（SSR，无需 JS）

② 用户点击「添加」
   └─► Alpine 调用 POST /api/todos { title }
       └─► api::create_todo 写入 store → 返回新 todo JSON
           └─► Alpine 将新项插入列表（局部更新，无整页刷新）

③ 用户点击「删除」
   └─► Alpine 调用 DELETE /api/todos/:id
       └─► api::delete_todo 删除 → 返回 204
           └─► Alpine 从列表移除该项
```

---

## 7. 扩展方向

| 方向 | 方案 |
| --- | --- |
| **热重载** | 监听 `templates/` 变化，重建 `Environment`（开发模式） |
| **布局继承** | 利用 minijinja 的 `{% extends %}` 实现多级布局 |
| **表单校验** | 服务端校验 + 客户端 Alpine 预校验，错误回填 |
| **会话/鉴权** | 引入 `tower-sessions`，中间件注入用户上下文 |
| **数据库** | 将 `Store` 替换为 `sqlx` 连接池 |
| **流式渲染** | 大页面用 `axum::response::stream` 分块输出 |
| **组件库** | 沉淀 `components/` 为可复用 UI 库 |

---

## 8. 设计权衡

| 决策 | 取舍 |
| --- | --- |
| 运行时模板 vs 编译期宏 | 选运行时：上手快、模板可热改；代价是轻微运行时开销与类型安全弱化 |
| Alpine vs 前端框架 | 选 Alpine：无构建、与 SSR 契合；代价是复杂状态管理能力有限 |
| JSON API vs HTMX | 选 JSON API：前后端职责清晰、便于对接移动端；代价是需手写 fetch 逻辑 |
| CDN vs 本地 Tailwind | 分阶段：开发期 CDN 提速，生产期本地构建保证性能与离线可用 |
