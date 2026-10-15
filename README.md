# Ada Framework

> 基于 **Rust + Minijinja + Tailwind CSS + Alpine.js** 的服务端渲染（SSR）前端框架，
> 内置 **Ada Design System** 组件库与后台管理系统模板。

首屏由 Rust 服务端渲染完整 HTML，交互由 Alpine.js 渐进增强，动态数据通过 JSON API 增量更新。

---

## 特性

- **SSR 优先**：首屏完整渲染，SEO 友好、加载快
- **组件化**：10 个组件文件、6 个分类，基于 Jinja 宏实现，全部符合 UED 规范
- **设计令牌**：语义化 CSS 变量，支持浅色/暗色双模式
- **渐进增强**：Alpine.js 声明式交互，无需前端构建工具链
- **数据分层**：页面走 HTML，动态数据走 `/api/*` JSON 接口
- **Tailwind 双模式**：开发期 CDN、生产期本地构建，一键切换
- **后台模板**：现代 SaaS 外壳 + 7 个完整页面（仪表盘/用户/表单/设置/组件库/登录/404）
- **图表零依赖**：面积图 / 迷你趋势线 / 条形排行均为纯 SVG 实现
- **JWT 鉴权**：HttpOnly Cookie + 无状态 Token，页面重定向 / API 401
- **数据持久化**：可选 SQLite（sqlx），未配置时纯内存运行
- **模板热重载**：开发模式监听 `templates/` 变更，改完即生效
- **安全加固**：安全响应头、请求体限制、错误响应内容协商

---

## 快速开始

```bash
cargo run
```

| 入口 | 地址 |
| --- | --- |
| 后台管理 | <http://127.0.0.1:3000/admin> |
| 组件库预览 | <http://127.0.0.1:3000/admin/ui-kit> |
| 登录页 | <http://127.0.0.1:3000/admin/login> |
| 示例页 | <http://127.0.0.1:3000/> |

测试：

```bash
cargo test
```

共 **73 个测试**：44 单元测试 + 20 HTTP 集成测试 + 9 模板渲染冒烟测试。

---

## 文档

| 文档 | 内容 |
| --- | --- |
| [`docs/UED.md`](./docs/UED.md) | **UED 设计规范**：设计原则、令牌、色彩、字体、间距、动效、无障碍 |
| [`ARCHITECTURE.md`](./ARCHITECTURE.md) | 框架架构设计：模块划分、数据流、扩展方向 |

---

## 目录结构

```
ada_fw/
├── docs/UED.md                  # UED 设计规范
├── docs/ROADMAP.md              # 下阶段开发计划
├── ARCHITECTURE.md              # 架构设计文档
├── migrations/                  # SQLite 迁移脚本
├── src/
│   ├── main.rs                  # 入口
│   ├── lib.rs                   # 库导出与状态初始化
│   ├── config.rs                # 配置层
│   ├── state.rs                 # 共享状态与数据存储
│   ├── error.rs                 # 统一错误类型
│   ├── middleware.rs            # 错误响应内容协商
│   ├── router.rs                # 路由注册
│   ├── auth/                    # JWT 鉴权与中间件
│   ├── db/                      # sqlx + SQLite 持久化
│   ├── template/                # 模板引擎与组件注册表
│   ├── handlers/
│   │   ├── pages.rs             # 示例页面
│   │   ├── admin.rs             # 后台页面与登录
│   │   └── api.rs               # JSON API
│   └── models/                  # 领域模型（Todo / User）
├── tests/
│   ├── http.rs                  # HTTP 集成测试
│   └── template_render.rs       # 模板渲染冒烟测试
├── templates/
│   ├── layouts/admin.html       # 后台布局（模板继承）
│   ├── admin/                   # 后台页面
│   ├── components/              # 组件库（10 个文件 / 6 个分类）
│   ├── partials/                # 令牌与 Tailwind 配置片段（CDN 模式内联）
│   ├── base.html                # 示例页布局
│   └── index.html / about.html  # 示例页
└── static/
    ├── css/tokens.css           # 设计令牌（浅色 + 暗色）
    ├── css/input.css            # Tailwind 源文件
    └── js/app.js                # Alpine 组件定义
```

---

## 组件库

共 **10 个组件文件 / 6 个分类**，完整预览见 `/admin/ui-kit`。

设计基调：**现代 SaaS 专业中后台**——中性外壳 + 靛蓝单强调色 + 分层柔和阴影，
详细规范见 [`docs/UED.md`](./docs/UED.md)。

| 分类 | 文件 | 组件 |
| --- | --- | --- |
| **基础** | `button.html` | Button（5 变体 × 3 尺寸）、IconButton、Link |
| | `input.html` | Input、Textarea、Select |
| | `form-controls.html` | Checkbox、Radio、Switch、Segmented |
| **展示** | `display.html` | Card、Badge、Tag、Avatar、Alert、Progress、Skeleton、Empty、StatRow |
| **导航** | `navigation.html` | Breadcrumb、Tabs、Pagination、Dropdown、Menu、CommandPalette |
| | `page.html` | PageHeader、SectionTitle |
| **反馈** | `feedback.html` | Modal、Drawer、Toast、Tooltip、Confirm |
| **数据** | `data.html` | Card、Table、Form、StatCard、StatGroup |
| | `chart.html` | AreaChart、Sparkline、BarChart（纯 SVG，无第三方依赖） |
| **业务** | `calendar.html` | Calendar、CalendarToolbar、NoticeBar |

### 使用方式

```jinja
{% import "components/button.html" as button %}
{% import "components/display.html" as display %}

{{ button.render(label="保存", variant="primary", size="md") }}
{{ display.badge(text="启用", variant="success") }}

{# 带插槽的组件用 call 块 #}
{% import "components/data.html" as data %}
{% call data.render(title="标题", subtitle="副标题") %}
  <p>内容</p>
{% endcall %}
```

> **重要**：minijinja 的宏调用使用**点号**语法 `module.macro(...)`，
> 不支持 Python Jinja2 的 `::` 命名空间语法。

---

## 设计令牌

所有样式通过**语义令牌**驱动，业务代码不直接引用基础色阶。

```css
/* 主色（靛蓝） */
--color-primary(#5b5bd6) / -hover / -active / -subtle / -muted / -fg / -ring

/* 功能色 */
--color-success / --color-warning / --color-danger / --color-info
   （每个都配 -subtle 浅底 与 -muted 边框）

/* 数据可视化色（指标卡图标底 / 图表系列 / 日历事件） */
--color-stat-{blue|orange|green|purple|red}-{bg|fg}

/* 中性色 */
--color-bg / --color-surface(-hover,-active,-inset)
--color-border(-strong) / --color-text(-secondary,-muted)

/* 圆角 / 阴影 / 动效 */
--radius-sm|md|lg|xl|full
--shadow-xs|sm|md|lg|xl
--duration-fast|base|slow

/* 布局 */
--sidebar-width(232px) / --sidebar-width-collapsed(68px)
--topbar-height(56px) / --content-max-width(1440px)
```

> ⚠️ 令牌值是 `var(--color-*)`，**不能使用 Tailwind 的透明度修饰符**
> （如 `bg-primary/50`）。Tailwind v3 会生成 `rgb(var(…) / .5)` 这类无效值，
> 样式静默失效。需要降低强度时请使用 `opacity-*` 或 `color-mix()`。

### 暗色模式

通过 `<html data-theme="dark">` 切换，用户偏好持久化到 `localStorage`，
首次访问跟随系统 `prefers-color-scheme`。

```html
<button @click="toggleTheme()">切换主题</button>
```

---

## 后台管理系统

### 布局

```
┌────────────┬──────────────────────────────────────┐
│ 品牌区      │  顶栏：面包屑 · ⌘K · 通知 · 主题 · 用户 │  ← 56px
├────────────┼──────────────────────────────────────┤
│ 侧边栏      │  内容区                               │
│ 概览        │   ┌─ 页面头（标题 + 说明 + 操作）      │
│ 管理        │   └─ 主体（卡片网格 / 表格）          │
│ 系统        │                                      │
└────────────┴──────────────────────────────────────┘
   232px              自适应（内容最大 1440px）
```

- 侧边栏全高常驻、按「概览 / 管理 / 系统」分组，可折叠（状态持久化到 `localStorage`）。
- 顶栏只覆盖内容区，含面包屑、`⌘K` 快捷导航、设计规范入口、通知、主题切换与用户菜单。
- 不使用浏览器式多页签栏：跨页跳转由 `⌘K` 快捷导航承担。
- `< md` 时侧边栏变为覆盖式抽屉，顶栏出现汉堡按钮。

### 页面

| 路径 | 说明 |
| --- | --- |
| `/admin` | 仪表盘：指标卡（含迷你趋势线）、营收面积图、最近用户、活动时间轴 |
| `/admin/users` | 用户管理：账号概览、搜索 + 状态分段筛选、表格、新建弹窗、删除确认 |
| `/admin/forms` | 表单示例：各类控件与校验状态 |
| `/admin/settings` | 设置：纵向分类导航、主题选择、通知偏好 |
| `/admin/ui-kit` | 组件库预览（含语义色板、图表、日历） |
| `/admin/login` | 登录页（左右分栏独立布局） |
| `/admin/404` | 404 页（独立布局） |

### 新增后台页面

1. 在 `templates/admin/` 创建页面，继承布局：

```jinja
{% extends "layouts/admin.html" %}
{% import "components/data.html" as data %}
{% import "components/page.html" as page %}
{% import "components/button.html" as button %}

{% block content %}
<div class="space-y-5">
  {# 页面头统一用 page.header，保证各页标题区结构一致 #}
  {{ page.header(
    title="页面标题",
    description="一句话说明这个页面能做什么。",
    actions=button.render(label="新建", variant="primary")
  ) }}

  {% call data.render(title="卡片") %}
    <p>内容</p>
  {% endcall %}
</div>
{% endblock %}
```

2. 在 `src/handlers/admin.rs` 添加上下文与处理器
3. 在 `src/router.rs` 注册路由
4. 在 `build_menu()` 中添加菜单项（记得填 `group`，否则不会分组显示）

> 上下文只需下发 `title`、`breadcrumb`、`menu_items` 三项即可套用布局；
> 菜单项会同时用于侧边栏与 `⌘K` 快捷导航。

---

## 路由一览

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `GET` | `/` `/about` | 示例页面 |
| `GET` | `/admin` | 仪表盘 |
| `GET` | `/admin/users` | 用户管理 |
| `GET` | `/admin/forms` | 表单示例 |
| `GET` | `/admin/settings` | 设置 |
| `GET` | `/admin/ui-kit` | 组件库 |
| `GET` | `/admin/login` | 登录页 |
| `POST` | `/admin/login` | 登录（签发 JWT Cookie） |
| `POST` | `/admin/logout` | 登出（清除 Cookie） |
| `GET` | `/admin/404` | 404 页 |
| `GET` `POST` | `/api/todos` | 待办列表 / 新建 |
| `DELETE` | `/api/todos/:id` | 删除待办 |
| `PATCH` | `/api/todos/:id/toggle` | 切换完成状态 |
| `GET` `POST` | `/api/users` | 用户列表 / 新建 |
| `DELETE` | `/api/users/:id` | 删除用户 |
| `GET` | `/static/*` | 静态资源 |
| `*` | 其他路径 | 404 页（HTML） |

---

## 配置

| 变量 | 默认值 | 说明 |
| --- | --- | --- |
| `ADA_HOST` | `127.0.0.1` | 监听地址 |
| `ADA_PORT` | `3000` | 监听端口 |
| `ADA_TEMPLATE_DIR` | `templates` | 模板目录 |
| `ADA_STATIC_DIR` | `static` | 静态资源目录 |
| `ADA_TAILWIND_MODE` | `cdn` | `cdn` 或 `local` |
| `ADA_HOT_RELOAD` | `false` | 模板热重载（监听 `templates/` 变更） |
| `ADA_JWT_SECRET` | 开发默认值 | **生产必须注入**，长度 ≥ 16 字符 |
| `ADA_JWT_TTL_SECS` | `86400` | Token 有效期（秒） |
| `ADA_AUTH_ENABLED` | `true` | 是否启用鉴权中间件 |
| `ADA_DATABASE_URL` | 未设置 | 设置后启用 SQLite 持久化，如 `sqlite://ada.db?mode=rwc` |

---

## 鉴权

后台与 API 默认受 **JWT + HttpOnly Cookie** 保护：

| 路径 | 鉴权 |
| --- | --- |
| `/`、`/about`、`/admin/login`、`/admin/404`、`/static/*` | 免鉴权 |
| `/admin`、`/admin/users`、`/admin/forms`、`/admin/settings`、`/admin/ui-kit` | 需登录 |
| `/api/*` | 需登录 |

**行为约定**：

- 未登录访问页面 → `303` 重定向到 `/admin/login?next=<原路径>`
- 未登录访问 API → `401` JSON
- 登录失败统一返回「邮箱或密码错误」，不区分具体原因（防用户枚举）
- 演示环境：任意合法邮箱 + 密码（≥ 6 位）即可登录

**生产部署**：

```bash
export ADA_JWT_SECRET="$(openssl rand -hex 32)"
export ADA_DATABASE_URL="sqlite://ada.db?mode=rwc"
ADA_TAILWIND_MODE=local cargo run --release
```

---

## 数据持久化

设置 `ADA_DATABASE_URL` 后启用 SQLite 持久化：

- 启动时执行 `migrations/0001_init.sql` 建表
- 首次启动（表为空）自动写入种子数据
- 内存 `Store` 为唯一事实来源，变更后异步落库
- 未设置该变量时使用纯内存存储（进程重启数据丢失）

---

## ���误响应内容协商

服务端错误按请求期望自动分流：

| 请求 `Accept` | 响应 |
| --- | --- |
| 含 `text/html` | 渲染 HTML 错误页（复用 `admin/404.html`） |
| 其他（如 `application/json`） | 返回 JSON 错误体 |

---

## 安全响应头

所有响应自动附加：

- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY`
- `Referrer-Policy: strict-origin-when-cross-origin`

请求体上限 **64 KiB**，超限返回 `413`。

---

## Tailwind CSS 集成

**开发期（CDN，默认）**：零配置，直接 `cargo run`。

**生产期（本地构建）**：

```bash
npm install
npm run css:build
ADA_TAILWIND_MODE=local cargo run
```

`tailwind.config.js` 已配置扫描 `templates/**/*.html` 与 `src/**/*.rs`，
并将语义令牌映射为 Tailwind 工具类（`bg-primary`、`text-content`、`border-border` 等）。

---

## 编写 Alpine 组件

在 `static/js/app.js` 中通过 `Alpine.data()` 注册：

```js
document.addEventListener('alpine:init', () => {
  Alpine.data('myWidget', () => ({
    count: 0,
    increment() { this.count++; },
  }));
});
```

内置组件：`adminLayout`（侧边栏 + 主题 + ⌘K 快捷导航）、`feedback`（Toast + Confirm）、
`loginForm`（登录）、`userManager`（用户 CRUD）、`todoApp`（待办 CRUD）。

> ⚠️ `Alpine.data()` 注册的组件其 `init()` 会被 Alpine 自动调用，
> 因此**不要再写 `x-init="init()"`**，否则监听器与数据加载会执行两遍。

### SSR 数据交接

服务端渲染的初始数据通过内联 JSON 注入，Alpine 初始化时接管：

```jinja
<script type="application/json" id="initial-todos">{{ initial_todos_json | safe }}</script>
```

---

## 安全说明

- 模板默认开启 **HTML 自动转义**，用户输入无需手动处理
- URL 等可信输出使用 `| safe` 显式标记，避免 `&#x2f;` 等冗余转义
- API 入参在 `CreateTodo::validate()` / `CreateUser::validate()` 中统一校验
- 服务端错误仅记录日志，不向客户端泄露内部细节

---

## 已知约束

| 约束 | 说明 |
| --- | --- |
| 宏调用语法 | minijinja 用 `module.macro()`，不支持 `::` |
| 同名宏覆盖 | 同一文件内不可定义同名宏，后者会覆盖前者 |
| 过滤器 | `truncate` 等需 `builtins` feature，默认不可用 |
| 布局机制 | 用 `{% extends %}` + `{% block %}`，不要用宏包裹整页 |

---

## License

MIT
