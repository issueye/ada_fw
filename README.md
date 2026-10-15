# Ada Framework

> 基于 **Rust + Minijinja + Tailwind CSS + Alpine.js** 的服务端渲染（SSR）前端框架，
> 内置 **Ada Design System** 组件库与后台管理系统模板。

首屏由 Rust 服务端渲染完整 HTML，交互由 Alpine.js 渐进增强，动态数据通过 JSON API 增量更新。

---

## 特性

- **SSR 优先**：首屏完整渲染，SEO 友好、加载快
- **组件化**：28 个符合 UED 规范的组件，基于 Jinja 宏实现
- **设计令牌**：语义化 CSS 变量，支持浅色/暗色双模式
- **渐进增强**：Alpine.js 声明式交互，无需前端构建工具链
- **数据分层**：页面走 HTML，动态数据走 `/api/*` JSON 接口
- **Tailwind 双模式**：开发期 CDN、生产期本地构建，一键切换
- **后台模板**：侧边栏布局 + 6 个完整页面（仪表盘/用户/表单/设置/登录/404）

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
├── ARCHITECTURE.md              # 架构设计文档
├── src/
│   ├── main.rs                  # 入口
│   ├── lib.rs                   # 库导出与状态初始化
│   ├── config.rs                # 配置层
│   ├── state.rs                 # 共享状态与数据存储
│   ├── error.rs                 # 统一错误类型
│   ├── router.rs                # 路由注册
│   ├── template/                # 模板引擎与组件注册表
│   ├── handlers/
│   │   ├── pages.rs             # 示例页面
│   │   ├── admin.rs             # 后台页面
│   │   └── api.rs               # JSON API
│   └── models/                  # 领域模型（Todo / User）
├── templates/
│   ├── layouts/admin.html       # 后台布局（模板继承）
│   ├── admin/                   # 后台页面
│   ├── components/              # 组件库（7 个文件 / 28 个组件）
│   ├── base.html                # 示例页布局
│   └── index.html / about.html  # 示例页
└── static/
    ├── css/tokens.css           # 设计令牌（浅色 + 暗色）
    ├── css/input.css            # Tailwind 源文件
    └── js/app.js                # Alpine 组件定义
```

---

## 组件库

共 **28 个组件**，分 5 类。完整预览见 `/admin/ui-kit`。

| 分类 | 文件 | 组件 |
| --- | --- | --- |
| **基础** | `button.html` | Button（5 变体 × 3 尺寸）、IconButton |
| | `input.html` | Input、Textarea、Select |
| | `form-controls.html` | Checkbox、Radio、Switch |
| **展示** | `display.html` | Card、Badge、Tag、Avatar、Alert、Progress、Skeleton、Empty |
| **导航** | `navigation.html` | Breadcrumb、Tabs、Pagination、Dropdown、Menu |
| **反馈** | `feedback.html` | Modal、Drawer、Toast、Tooltip、Confirm |
| **数据** | `data.html` | Card、Table、Form、StatCard |

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
/* 语义色 */
--color-primary / --color-primary-hover / --color-primary-active
--color-success / --color-warning / --color-danger / --color-info

/* 中性色 */
--color-bg / --color-surface / --color-border / --color-text
--color-text-secondary / --color-text-muted

/* 圆角 / 动效 */
--radius-sm|md|lg|xl
--duration-fast|base|slow
```

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
┌──────────────────────────────────────────────┐
│  Sidebar (240px / 折叠 64px)  │   Topbar 56px │
│                               ├───────────────┤
│                               │   Content     │
│                               │   max-w-7xl   │
└──────────────────────────────────────────────┘
```

侧边栏可折叠（状态持久化），顶栏含面包屑、主题切换、通知、用户菜单。

### 页面

| 路径 | 说明 |
| --- | --- |
| `/admin` | 仪表盘：指标卡、最近用户、活动流 |
| `/admin/users` | 用户管理：搜索、表格、新建弹窗、删除 |
| `/admin/forms` | 表单示例：各类控件与校验状态 |
| `/admin/settings` | 设置：标签页、主题选择、通知偏好 |
| `/admin/ui-kit` | 组件库预览 |
| `/admin/login` | 登录页（独立布局） |
| `/admin/404` | 404 页（独立布局） |

### 新增后台页面

1. 在 `templates/admin/` 创建页面，继承布局：

```jinja
{% extends "layouts/admin.html" %}
{% import "components/data.html" as data %}

{% block content %}
<div class="space-y-6">
  <h1 class="text-2xl font-semibold text-content">页面标题</h1>
  {% call data.render(title="卡片") %}
    <p>内容</p>
  {% endcall %}
</div>
{% endblock %}
```

2. 在 `src/handlers/admin.rs` 添加上下文与处理器
3. 在 `src/router.rs` 注册路由
4. 在 `build_menu()` 中添加菜单项

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
| `GET` | `/admin/404` | 404 页 |
| `GET` `POST` | `/api/todos` | 待办列表 / 新建 |
| `DELETE` | `/api/todos/:id` | 删除待办 |
| `PATCH` | `/api/todos/:id/toggle` | 切换完成状态 |
| `GET` `POST` | `/api/users` | 用户列表 / 新建 |
| `DELETE` | `/api/users/:id` | 删除用户 |
| `GET` | `/static/*` | 静态资源 |

---

## 配置

| 变量 | 默认值 | 说明 |
| --- | --- | --- |
| `ADA_HOST` | `127.0.0.1` | 监听地址 |
| `ADA_PORT` | `3000` | 监听端口 |
| `ADA_TEMPLATE_DIR` | `templates` | 模板目录 |
| `ADA_STATIC_DIR` | `static` | 静态资源目录 |
| `ADA_TAILWIND_MODE` | `cdn` | `cdn` 或 `local` |
| `ADA_HOT_RELOAD` | `false` | 模板调试信息 |

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

内置组件：`adminLayout`（侧边栏 + 主题）、`feedback`（Toast + Confirm）、
`userManager`（用户 CRUD）、`todoApp`（待办 CRUD）。

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
