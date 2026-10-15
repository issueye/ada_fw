# Ada Design System · UED 设计规范

> 版本 `2.0.0` · 适用于 Ada Framework 后台管理系统
>
> **v2.0 变更**：设计基调由「Indigo 中性风」调整为「企业级管理后台风」，
> 主色改为亮蓝 `#1677ff`，新增蓝色顶栏、多页签栏、多色统计卡与排班日历组件，
> 圆角收紧至 2-6px，卡片默认去除阴影。

---

## 目录

1. [设计原则](#1-设计原则)
2. [设计令牌](#2-设计令牌)
3. [色彩系统](#3-色彩系统)
4. [字体系统](#4-字体系统)
5. [间距系统](#5-间距系统)
6. [圆角与阴影](#6-圆角与阴影)
7. [动效规范](#7-动效规范)
8. [布局栅格](#8-布局栅格)
9. [组件规范](#9-组件规范)
10. [无障碍规范](#10-无障碍规范)
11. [命名约定](#11-命名约定)

---

## 1. 设计原则

| 原则 | 说明 | 落地要求 |
| --- | --- | --- |
| **清晰优先** | 信息层级一目了然，减少视觉噪音 | 同一屏内主色使用不超过 3 处 |
| **一致性** | 相同语义使用相同视觉表达 | 所有交互元素必须来自组件库 |
| **高效** | 为数据密集型场景优化信息密度 | 表格行高 44px，紧凑模式 36px |
| **可预期** | 交互反馈及时且符合直觉 | 所有异步操作必须有 loading 态 |
| **可访问** | 键盘可达、对比度达标 | 正文对比度 ≥ 4.5:1 |

### 设计基调

**企业级管理后台风**：以中性灰为骨架，**高饱和亮蓝**为主色，顶栏整条铺满主色。
圆角克制（2-6px），阴影极弱，以细边框区分层级，信息密度高。
参考调性：Element Plus / Ant Design。

### 视觉特征

| 特征 | 说明 |
| --- | --- |
| **蓝色顶栏** | 顶栏整条铺满主色 `#1677ff`，Logo 与系统名居左，操作区居右 |
| **浅色侧边栏** | 白底侧边栏，选中项浅蓝底 + 左侧蓝色竖条 |
| **多页签栏** | 顶栏下方可关闭的页签，当前页白底蓝字 |
| **细边框卡片** | 1px 边框 + 2-6px 圆角，几乎无阴影 |
| **多色统计卡** | 5 色并排（蓝/橙/绿/紫/红），大号数字 + 右侧水印图标 |
| **紧凑表格** | 表头浅灰底、行分隔线、行高 40px |

---

## 2. 设计令牌

���计令牌（Design Token）是设计系统的最小原子，以 **CSS 变量** 形式实现，
通过 `data-theme` 属性切换浅色/暗色两套值。

### 令牌分层

```
基础令牌 (Primitive)  →  语义令牌 (Semantic)  →  组件令牌 (Component)
   --indigo-600            --color-primary          --btn-primary-bg
```

- **基础令牌**：原始色阶，不直接使用
- **语义令牌**：表达用途（`--color-primary`、`--color-danger`），**业务代码只用这一层**
- **组件令牌**：特定组件的私有变量，由语义令牌派生

### 令牌定义位置

| 文件 | 职责 |
| --- | --- |
| `static/css/tokens.css` | 定义全部 CSS 变量（浅色 + 暗色） |
| `tailwind.config.js` | 将语义令牌映射为 Tailwind 工具类 |

---

## 3. 色彩系统

### 3.1 主色（Primary · Bright Blue）

| 令牌 | 色值 | 用途 |
| --- | --- | --- |
| `--color-primary-subtle` | `#e8f3ff` | 选中背景、浅色标签底 |
| `--color-primary-muted` | `#a3ccff` | 边框、分隔 |
| `--color-primary` | `#1677ff` | 主按钮、链接、激活态、顶栏 |
| `--color-primary-hover` | `#4096ff` | 悬停 |
| `--color-primary-active` | `#0958d9` | 按下 |
| `--color-primary-fg` | `#ffffff` | 主色上的文字 |

### 3.1.1 顶栏色（Topbar）

| 令牌 | 色值 | 用途 |
| --- | --- | --- |
| `--color-topbar-bg` | `#1677ff` | 顶栏背景（浅色模式） |
| `--color-topbar-fg` | `#ffffff` | 顶栏文字与图标 |
| `--color-topbar-hover` | `rgb(255 255 255 / 0.16)` | 顶栏按钮悬停 |
| `--color-topbar-active` | `rgb(255 255 255 / 0.24)` | 顶栏按钮按下 |

### 3.2 中性色（Neutral · Slate）

| 令牌 | 浅色 | 暗色 | 用途 |
| --- | --- | --- | --- |
| `--color-bg` | `#f0f2f5` | `#141414` | 页面背景 |
| `--color-surface` | `#ffffff` | `#1f1f1f` | 卡片、面板 |
| `--color-surface-hover` | `#f5f7fa` | `#2a2a2a` | 悬停背景 |
| `--color-border` | `#e4e7ed` | `#303030` | 边框、分隔线 |
| `--color-border-strong` | `#d4d7de` | `#424242` | 输入框边框 |
| `--color-text` | `#1f2329` | `#e8e8e8` | 主文本 |
| `--color-text-secondary` | `#5c6370` | `#a6a6a6` | 次要文本 |
| `--color-text-muted` | `#8f959e` | `#6b6b6b` | 占位、禁用 |
| `--color-text-inverse` | `#ffffff` | `#141414` | 反色文本 |

### 3.2.1 统计卡专用色（Stat）

5 色并排，用于数据概览卡片。每色提供 `-bg`（浅底）与 `-fg`（前景）两个令牌。

| 变体 | 浅底 | 前景 |
| --- | --- | --- |
| `blue` | `#e8f3ff` | `#1677ff` |
| `orange` | `#fff5e6` | `#fa8c16` |
| `green` | `#e8f8ee` | `#00a854` |
| `purple` | `#f4eeff` | `#722ed1` |
| `red` | `#fff1f0` | `#f5222d` |

### 3.3 功能色（Semantic）

| 语义 | 令牌前缀 | 色值 | 用途 |
| --- | --- | --- | --- |
| 成功 | `--color-success` | `#00a854` | 操作成功、已完成 |
| 警告 | `--color-warning` | `#fa8c16` | 需注意、待处理 |
| 危险 | `--color-danger` | `#f5222d` | 错误、删除、破坏性操作 |
| 信息 | `--color-info` | `#1890ff` | 中性提示 |

每个功能色均提供 `-subtle`（浅底）、`-muted`（边框）、`-fg`（前景）三个衍生令牌。

### 3.4 色彩使用规则

- **主色**：仅用于主操作按钮、激活态、链接、焦���环
- **功能色**：仅用于状态表达，**禁止**用于装饰
- **危险色**：破坏性操作必须二次确认
- **禁止**：直接使用基础色阶（如 `bg-indigo-600`），必须走语义令牌

---

## 4. 字体系统

### 4.1 ���体族

```css
--font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", "PingFang SC",
             "Hiragino Sans GB", "Microsoft YaHei", "Helvetica Neue", sans-serif;
--font-mono: "JetBrains Mono", "SFMono-Regular", Consolas, monospace;
```

### 4.2 字号阶梯

| 令牌 | 字号 / 行高 | 字重 | 用途 |
| --- | --- | --- | --- |
| `text-xs` | 12px / 16px | 400 | 辅助说明、标签 |
| `text-sm` | 14px / 20px | 400 | **正文默认**、表格、表单 |
| `text-base` | 16px / 24px | 400 | 强调正文 |
| `text-lg` | 18px / 28px | 500 | 卡片标题 |
| `text-xl` | 20px / 28px | 600 | 区块标题 |
| `text-2xl` | 24px / 32px | 600 | 页面标题 |
| `text-3xl` | 30px / 36px | 700 | 数据大屏数字 |

### 4.3 字重

| 令牌 | 值 | 用途 |
| --- | --- | --- |
| `font-normal` | 400 | 正文 |
| `font-medium` | 500 | 按钮、标签、表头 |
| `font-semibold` | 600 | 标题 |
| `font-bold` | 700 | 强调数字 |

### 4.4 规则

- 后台系统**正文默认 `text-sm`**（14px），而非 16px，以提升信息密度
- 数字类数据使用 `font-mono` + `tabular-nums` 保证对齐
- 单行文本超长使用 `truncate`，多行使用 `line-clamp-2`

---

## 5. 间距系统

基于 **4px 基准网格**：

| 令牌 | 值 | 典型用途 |
| --- | --- | --- |
| `space-1` | 4px | 图标与文字间距 |
| `space-2` | 8px | 紧凑元素间距 |
| `space-3` | 12px | 表单控件内边距 |
| `space-4` | 16px | **默认间距**、卡片内边距 |
| `space-5` | 20px | 区块内间距 |
| `space-6` | 24px | 卡片间距、区块间距 |
| `space-8` | 32px | 页面区块间距 |
| `space-12` | 48px | 大区块分隔 |

### 规则

- 所有间距必须是 **4 的倍数**
- 卡片内边距统一 `p-4`（16px），紧凑卡片 `p-3`
- 页面内容区左右内边距 `px-4`，上下 `py-4`
- 表单控件垂直间距 `space-y-4`

---

## 6. 圆角与阴影

### 6.1 圆角（克制）

| 令牌 | 值 | 用途 |
| --- | --- | --- |
| `rounded-sm` | 2px | 标签、徽章、菜单项 |
| `rounded-md` | 4px | **按钮、输入框、卡片（默认）** |
| `rounded-lg` | 6px | 面板、下拉菜单 |
| `rounded-xl` | 8px | 弹窗、抽屉 |
| `rounded-full` | 9999px | 头像、圆形按钮 |

> **设计约束**：圆角最大不超过 8px。管理后台追求信息密度与秩序感，
> 大圆角会削弱专业感。

### 6.2 阴影（极弱）

| 令牌 | 用途 |
| --- | --- |
| `shadow-xs` | 输入框、按钮（几乎不可见） |
| `shadow-sm` | 卡片悬停 |
| `shadow-md` | 下拉菜单、浮层 |
| `shadow-lg` | 弹窗、抽屉 |
| `shadow-xl` | 全局浮层 |

> **设计约束**：卡片默认**不使用阴影**，仅用 1px 边框区分层级。
> 阴影仅用于真正浮起的元素（弹窗、下拉、抽屉）。

### 规则

- 暗色模式下阴影减弱，改用边框区分层级
- 弹窗必须同时有阴影 + 遮罩层（`bg-black/50`）

---

## 7. 动效规范

### 7.1 时长

| 令牌 | 值 | 用途 |
| --- | --- | --- |
| `duration-fast` | 100ms | 悬停、颜色变化 |
| `duration-base` | 150ms | **默认**、按钮、输入框 |
| `duration-slow` | 250ms | 弹窗、抽屉、折叠 |

### 7.2 缓动

| 令牌 | 值 | 用途 |
| --- | --- | --- |
| `ease-out` | `cubic-bezier(0, 0, 0.2, 1)` | 元素进入 |
| `ease-in` | `cubic-bezier(0.4, 0, 1, 1)` | 元素退出 |
| `ease-in-out` | `cubic-bezier(0.4, 0, 0.2, 1)` | 状态切换 |

### 7.3 规则

- 动效必须服务于**状态变化**，禁止纯装饰动画
- 尊重 `prefers-reduced-motion`，用户关闭动效时降级为瞬时
- 弹窗进入：`opacity 0→1` + `scale 0.95→1`，150ms `ease-out`
- 抽屉进入：`translateX` 滑入，250ms `ease-out`

---

## 8. 布局栅格

### 8.1 后台布局结构

```
┌──────────────────────────────────────────────┐
│  Topbar 48px（蓝色铺满：Logo + 系统名 + 右侧操作）      │
├──────────┬───────────────────────────────────────────┤
│          │  Tabbar 40px（多页签，可关闭）             │
│ Sidebar  ├───────────────────────────────────────────┤
│ 200px    │  Breadcrumb（面包屑）                      │
│ (折叠56) ├───────────────────────────────────────────┤
│          │  Content  p-4                             │
└──────────┴───────────────────────────────────────────┘
```

| 区域 | 尺寸 | 说明 |
| --- | --- | --- |
| 顶栏 | 48px | 整条铺满主色，含 Logo、系统名、全屏/通知/主题/用户菜单 |
| 侧边栏 | 200px（展开）/ 56px（折叠） | 白底，选中项浅蓝底 + 左侧蓝竖条 |
| 页签栏 | 40px | 多页签，首页不可关闭 |
| 面包屑 | 自适应 | 白底，`px-4 py-2` |
| 内容区 | 自适应 | `p-4`，不限制最大宽度 |

### 8.2 断点

| 断点 | 宽度 | 行为 |
| --- | --- | --- |
| `sm` | ≥ 640px | 表格显示更多列 |
| `md` | ≥ 768px | 侧边栏可折叠 |
| `lg` | ≥ 1024px | 侧边栏默认展开 |
| `xl` | ≥ 1280px | 统计卡 5 列并排 |

### 8.3 响应式规则

- `< md`：侧边栏转为抽屉式浮层
- 表格在窄屏下横向滚动，不折行
- 统计卡片：`grid-cols-1` → `sm:grid-cols-2` → `lg:grid-cols-5`

---

## 9. 组件规范

### 9.1 尺寸体系

所有交互组件统一三档尺寸：

| 尺寸 | 高度 | 内边距 | 字号 | 用途 |
| --- | --- | --- | --- | --- |
| `sm` | 32px | `px-3` | 12px | 表格内操作、紧凑区 |
| `md` | 36px | `px-4` | 14px | **默认** |
| `lg` | 44px | `px-6` | 16px | 登录页、强调操作 |

### 9.2 状态规范

所有交互组件必须覆盖以下状态：

| 状态 | 视觉表现 |
| --- | --- |
| 默认 | 基础样式 |
| 悬停 | 背景/边框加深一档 |
| 聚焦 | 2px 焦点环 `ring-2 ring-primary/30` |
| 激活 | 再加深一档 |
| 禁用 | `opacity-50` + `cursor-not-allowed` |
| 加载 | 显示 spinner，禁用交互 |

### 9.3 组件清单

#### 基础组件��7）

| 组件 | 变体 |
| --- | --- |
| Button | primary / secondary / outline / ghost / danger，三档尺寸，loading，icon |
| Input | 默认 / 带前缀图标 / 带后缀 / 错误态 / 禁用 |
| Textarea | 默认 / 错误 / 禁用 / 自适应高度 |
| Select | 原生 / 自定义下拉 |
| Checkbox | 默认 / 半选 / 禁用 |
| Radio | 默认 / 禁用 |
| Switch | 默认 / 禁用 / 带标签 |

#### 展示组件（8）

| 组件 | 说明 |
| --- | --- |
| Card | 标题 / 副标题 / 操作区 / 内容 / 底部 |
| Badge | 圆点 / 数字 / 状态色 |
| Tag | 可关闭 / 颜色变体 |
| Avatar | 图片 / 文字 / 尺寸 / 状态点 |
| Alert | info / success / warning / danger，可关闭 |
| Progress | 线性 / 环形，带标签 |
| Skeleton | 文本 / 卡片 / 表格 |
| Empty | 空状态，带图标与操作 |

#### 导航组件（5）

| 组件 | 说明 |
| --- | --- |
| Breadcrumb | 面包屑，支持图标 |
| Tabs | 下划线 / 胶囊，支持徽章 |
| Pagination | 页码 / 每页条数 / 跳转 |
| Dropdown | 下拉菜单，支持分组与危险项 |
| Menu | 侧边栏菜单，支持折叠与子菜单 |

#### 反馈组件（5）

| 组件 | 说明 |
| --- | --- |
| Modal | 尺寸 / 标题 / 底部操作 / 遮罩关闭 |
| Drawer | 四方向 / 尺寸 |
| Toast | 四类型 / 自动消失 / 堆叠 |
| Tooltip | 四方向 / 延迟显示 |
| Confirm | 二次确认，危险操作专用 |

#### 数据组件（4）

| 组件 | 说明 |
| --- | --- |
| Table | 排序 / 选择 / 分页 / 空态 / 加载 |
| Form | 布局 / 校验 / 错误提示 / 分组 |
| StatCard | 指标卡，浅色底 + 大号数字 + 水印图标 |
| StatGroup | 指标卡组，5 色并排 |

#### 业务组件（3）

| 组件 | 说明 |
| --- | --- |
| Calendar | 排班日历，7 列网格 + 彩色事件条 |
| CalendarToolbar | 日历工具栏，时间选择 + 视图切换 |
| NoticeBar | 提示条，四语义变体，可展开 |

**合计 31 个组件。**

---

## 9.4 关键组件视觉规范

### 顶栏（Topbar）

- 高度 48px，背景铺满主色 `#1677ff`
- 左侧：Logo（白色半透明底 28×28）+ 系统名（16px semibold）
- 右侧：全屏 / 通知 / 主题切换 / 用户菜单，均为 32×32 图标按钮
- 图标按钮悬停：`rgb(255 255 255 / 0.16)` 背景

### 侧边栏菜单项（Menu Item）

| 状态 | 视觉 |
| --- | --- |
| 默认 | 透明底，`text-content-secondary` |
| 悬停 | `bg-surface-hover`，文字转 `text-content` |
| 选中 | `bg-primary-subtle` + `text-primary` + **左侧 2px 蓝色竖条** |

### 多页签（Tabbar）

- 高度 40px，背景 `bg-surface-hover`
- 未选中：透明底，`text-content-secondary`，右侧 1px 分隔线
- 选中：白底 + `text-primary` + `font-medium`
- 首页不可关闭，其余页签带关闭按钮

### 统计卡（StatCard）

- 浅色底（5 色可选）+ 1px 边框 + 4px 圆角
- 标签：12px `text-content-secondary`
- 数值：24px `font-semibold` + `tabular-nums`
- 右侧水印图标：64×64，`opacity-10`，溢出裁切

### 排班日历（Calendar）

- 7 列网格，星期表头浅灰底
- 日期数字：14px `font-medium`，节假日为 `text-danger`
- 事件条：12px，圆角 2px，按类型着色（蓝=班次、绿=休息、红=异常）
- 单元格最小高度 96px，非本月日期 `bg-surface-hover`

---

## 10. 无障碍规范

### 10.1 对比度

| 内容类型 | 最低对比度 |
| --- | --- |
| 正文文本 | 4.5:1 |
| 大号文本（≥18px 或 ≥14px 粗体） | 3:1 |
| 图标、边框 | 3:1 |

### 10.2 键盘可达

- 所有交��元素必须可通过 `Tab` 聚焦
- 焦点样式不可移除，使用 `focus-visible:ring-2`
- 弹窗打开时焦点锁定在弹窗内，`Esc` 关闭
- 下拉菜单支持 `↑` `↓` `Enter` `Esc`

### 10.3 语义化

- 按钮用 `<button>`，链接用 `<a>`
- 表单控件必须关联 `<label>`（`for` / `id`）
- 图标按钮必须有 `aria-label`
- 状态变化使用 `aria-live` 通知

### 10.4 暗色模式

- 通过 `<html data-theme="dark">` 切换
- 用户偏好持久化到 `localStorage`
- 首次访问跟随系统 `prefers-color-scheme`

---

## 11. 命名约定

### 11.1 CSS 变量

```
--color-{语义}           语义色，如 --color-primary
--color-{语义}-{变体}    变体，如 --color-primary-hover
--{类别}-{属性}          其他令牌，如 --radius-md、--shadow-sm
```

### 11.2 组件模板

```
templates/components/{组件名}.html
```

组件内宏统一命名为 `render`，参数使用具名传递：

```jinja
{% import "components/button.html" as button %}
{{ button.render(label="保存", variant="primary", size="md") }}
```

### 11.3 组件参数

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `variant` | string | 视觉变体 |
| `size` | string | 尺寸档位 |
| `disabled` | bool | 禁用态 |
| `loading` | bool | 加载态 |

---

## 附录：令牌速查

```css
/* 语义色 */
--color-primary / --color-primary-hover / --color-primary-active
--color-success / --color-warning / --color-danger / --color-info

/* 中性色 */
--color-bg / --color-surface / --color-border / --color-text
--color-text-secondary / --color-text-muted

/* 圆角 */
--radius-sm: 4px / --radius-md: 6px / --radius-lg: 8px / --radius-xl: 12px

/* 动效 */
--duration-fast: 100ms / --duration-base: 150ms / --duration-slow: 250ms
```
