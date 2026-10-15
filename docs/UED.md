# Ada Design System · UED 设计规范

> 版本 `3.0.0` · 适用于 Ada Framework 后台管理系统
>
> **v3.0 变更**：设计基调由「Element Plus 式企业后台」调整为「现代 SaaS 专业中后台」。主要变化：
> 1. 主色由亮蓝 `#1677ff` 改为克制的靛蓝 `#5b5bd6`，并把主色使用面积压到 10% 以内；
> 2. 外壳（顶栏 / 侧边栏）由「高饱和蓝色铺满」改为中性白 + hairline 分隔；
> 3. 移除浏览器式多页签栏，改为「面包屑 + 页面头」+ 全局 ⌘K 快捷导航；
> 4. 指标卡由「五色便签并排」改为白底卡片 + 图标色块，色彩只用于强调；
> 5. 圆角放宽到 6–14px，阴影改为分层柔和投影，取代原先「靠重边框区分层级」；
> 6. 新增 `Chart`（纯 SVG 面积图）与 `Page`（页面头）组件。

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
| **色不过三** | 一屏内的强调色种类越少，信息层级越清晰 | 主色使用面积 ≤ 10%，同一屏主色出现 ≤ 3 处 |
| **中性为骨** | 外壳、卡片、表格保持中性，让数据本身成为视觉主体 | 顶栏与侧边栏不使用高饱和大色块 |
| **留白优先** | 用间距而非分隔线划分区块 | 优先 `gap` / `padding`，其次才是边框 |
| **一致可预期** | 相同语义必须有相同视觉表达 | 所有交互元素必须来自 `components/` |
| **可访问** | 键盘可达、对比度达标、暗色模式对等 | 正文对比度 ≥ 4.5:1，所有交互有可见焦点环 |

### 设计基调

**现代 SaaS 专业中后台**：冷调中性灰作骨架，靛蓝作唯一强调色。
外壳（顶栏 / 侧边栏）保持中性白，用 1px hairline 分隔；
卡片以「hairline 边框 + 极轻投影」浮起，圆角 6–14px；
信息密度适中，靠排版层级（字号 / 字重 / 颜色深浅）而非色块堆叠区分内容。

参考调性：Linear / Stripe Dashboard / Vercel Dashboard。

### 视觉特征

| 特征 | 说明 |
| --- | --- |
| **中性外壳** | 侧边栏全高常驻、白底 hairline 右边框；顶栏只覆盖内容区，形成 L 形分隔 |
| **分组侧边栏** | 菜单分「概览 / 管理 / 系统」三组，组名以大写小字标签呈现 |
| **药丸激活态** | 菜单选中项为主色浅底圆角药丸，不再使用左侧竖条 |
| **页面头** | 面包屑在顶栏，内容区顶部统一为「标题 + 说明 + 右侧操作」 |
| **白底指标卡** | 卡片本体白底，色彩只落在右上角图标块与涨跌胶囊上 |
| **面积图** | 纯 SVG 实现，网格线用 `--color-border`，主系列靛蓝、对比系列中性灰 |
| **紧凑表格** | 表头小字 + 弱底色，行以 hairline 分隔，整行悬停高亮 |
| **分段控件** | 同页视图切换用分段控件（`.segmented`），不是浏览器式页签 |

### 明确不采用的做法

以下形态在 v3.0 中被有意移除，新增页面不应再引入：

| 形态 | 原因 |
| --- | --- |
| 高饱和主色铺满顶栏 | 大面积色块压过数据本身，视觉噪音重 |
| 浏览器式多页签栏 | 页签会无限堆积，且与浏览器标签页职责重复 |
| 多色浅底「便签卡片」并排 | 五色并排使任何一项都不突出，是典型的模板感来源 |
| 满屏大圆角 + 紫色渐变 | 与业务工具的产品调性不符 |
| 声明实现细节的页脚（如 "Powered by …"） | 属于实现细节，对使用者无信息量 |

---

## 2. 设计令牌

设计令牌（Design Token）是设计系统的最小原子，以 **CSS 变量** 形式实现，
通过 `data-theme` 属性切换浅色 / 暗色两套值。

### 令牌分层

```
基础令牌 (Primitive)  →  语义令牌 (Semantic)  →  组件令牌 (Component)
   （色阶原值）            --color-primary          --btn-primary-bg
```

- **基础令牌**：原始色阶，不直接使用
- **语义令牌**：表达用途（`--color-primary`、`--color-danger`），**业务代码只用这一层**
- **组件令牌**：特定组件的私有变量，由语义令牌派生

### 令牌定义位置

| 文件 | 职责 |
| --- | --- |
| `static/css/tokens.css` | 定义全部 CSS 变量（浅色 + 暗色）与基础样式 |
| `static/css/components.css` | 组件类（纯 CSS，不含 Tailwind 指令） |
| `tailwind.config.js` | 本地构建：将语义令牌映射为 Tailwind 工具类 |
| `templates/partials/tailwind-config.html` | CDN 模式：同一份映射的 CDN 版本 |

> ⚠️ **一致性约束**：新增令牌时必须同时更新 `tailwind.config.js` 与
> `templates/partials/tailwind-config.html`。两者脱节会导致
> 「开发期正常、生产期样式错乱」这类难排查的问题。
> `tests/cdn_styles.rs` 会校验 CDN 内联产物包含关键令牌与组件类。

> ⚠️ **Tailwind v3 限制**：语义令牌的值是 `var(--color-*)`，
> 因此**不能对其使用透明度修饰符**（如 `bg-primary/50`、`text-content-muted/70`）。
> v3 会生成 `rgb(var(…) / .5)` 这类无效值，样式静默失效。
> 需要降低强度时改用 `opacity-*` 工具类或 `color-mix()`。

---

## 3. 色彩系统

### 3.1 主色（Primary · Iris 靛蓝）

| 令牌 | 浅色 | 暗色 | 用途 |
| --- | --- | --- | --- |
| `--color-primary-subtle` | `#f0f0fd` | `#1e1e3d` | 选中背景、浅色标签底 |
| `--color-primary-muted` | `#c9c9f2` | `#33336b` | 边框、分隔、选中文字底色 |
| `--color-primary` | `#5b5bd6` | `#7c7ce8` | 主按钮、链接、激活态、图表主系列 |
| `--color-primary-hover` | `#6e6ade` | `#8d8dee` | 悬停 |
| `--color-primary-active` | `#4b4bc0` | `#6a6ad9` | 按下 |
| `--color-primary-fg` | `#ffffff` | `#ffffff` | 主色上的文字 |
| `--color-primary-ring` | `rgb(91 91 214 / .22)` | `rgb(124 124 232 / .28)` | 输入框投影、焦点环 |

**暗色模式主色提亮**：深底上 `#5b5bd6` 对比度不足，因此暗色下改用 `#7c7ce8`。

### 3.2 外壳色（Topbar）

外壳**不使用主色**，只使用中性色。顶栏与侧边栏均取 `--color-surface`，以 `--color-border` 的 1px 线分隔。

| 令牌 | 浅色 | 暗色 | 用途 |
| --- | --- | --- | --- |
| `--color-topbar-bg` | `#ffffff` | `#16181f` | 顶栏背景 |
| `--color-topbar-fg` | `#101322` | `#e9eaf0` | 顶栏文字与图标 |
| `--color-topbar-muted` | `#6b7280` | `#8b91a1` | 顶栏次要文字 |
| `--color-topbar-hover` | `rgb(16 19 34 / .05)` | `rgb(255 255 255 / .06)` | 顶栏按钮悬停 |
| `--color-topbar-active` | `rgb(16 19 34 / .09)` | `rgb(255 255 255 / .11)` | 顶栏按钮按下 |

### 3.3 中性色（Neutral · Cool Gray）

| 令牌 | 浅色 | 暗色 | 用途 |
| --- | --- | --- | --- |
| `--color-bg` | `#f6f7f9` | `#0e0f14` | 应用画布 |
| `--color-surface` | `#ffffff` | `#16181f` | 卡片、侧边栏、顶栏 |
| `--color-surface-hover` | `#f3f4f7` | `#1d2029` | 悬停底 |
| `--color-surface-active` | `#e9ebf0` | `#252833` | 按下底、分段控件槽 |
| `--color-surface-inset` | `#f8f9fb` | `#12141a` | 表头、内嵌区块 |
| `--color-border` | `#e8eaef` | `#252832` | hairline 边框、分隔线 |
| `--color-border-strong` | `#d8dbe3` | `#343846` | 输入框、描边按钮 |
| `--color-text` | `#101322` | `#e9eaf0` | 正文与标题 |
| `--color-text-secondary` | `#5b6273` | `#a2a7b6` | 次级文字 |
| `--color-text-muted` | `#8b91a1` | `#767d8d` | 辅助说明、占位符 |

**暗色模式不用纯黑**：纯黑会让边框消失、阴影失效，因此采用 `#0e0f14` 一类的深冷灰。

### 3.4 功能色（Semantic）

| 令牌 | 浅色 | 暗色 | 用途 |
| --- | --- | --- | --- |
| `--color-success` | `#12864f` | `#3fbf7f` | 成功、上升趋势、启用状态 |
| `--color-warning` | `#b3690b` | `#e0a355` | 警告、待处理 |
| `--color-danger` | `#d92d20` | `#f2685c` | 危险操作、错误、下降趋势 |
| `--color-info` | `#1f6feb` | `#6ea8f5` | 中性提示 |

每个功能色都配有 `-subtle`（浅底）与 `-muted`（边框）两个衍生令牌，
用于徽章、提示条与事件条等「浅底 + 同源文字」的组合。

### 3.5 数据可视化色（Stat）

命名为 `--color-stat-{blue|orange|green|purple|red}-{bg|fg}`，
语义是「第 N 号数据色」，与业务无关。

用途：指标卡图标块、图表系列、日历事件条、头像底色。

| 编号 | 浅色前景 | 典型用途 |
| --- | --- | --- |
| blue | `#5b5bd6` | 主数据系列、图表主曲线 |
| green | `#12864f` | 正向指标（活跃、完成） |
| orange | `#b3690b` | 待处理指标 |
| purple | `#7b3fd4` | 金额类指标、管理员角色 |
| red | `#d92d20` | 异常指标 |

> **重要**：这五个色**只能作为点缀**（图标底、小色块、曲线），
> 不得再用于整张卡片的背景铺色。

### 3.6 色彩使用规则

1. 主色使用面积 ≤ 10%：主按钮、链接、激活态、图表主系列。
2. 同一屏内主色出现 ≤ 3 处，超出说明层级设计失效，应改用中性色区分。
3. 表格内不使用填充色块，状态用「圆点 + 文字」或小号徽章表达。
4. 图表的第二条对比系列使用 `--color-text-muted`（中性灰），
   与主系列形成「当前 vs 参照」的语义区分，而不是两个相近色。
5. 任何颜色都必须通过语义令牌引用，禁止在模板里硬编码色值。

---

## 4. 字体系统

### 4.1 字体族

```css
--font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI Variable Text",
  "Segoe UI", "PingFang SC", "Hiragino Sans GB", "Microsoft YaHei UI",
  "Microsoft YaHei", "Helvetica Neue", Helvetica, Arial, sans-serif;
--font-mono: "JetBrains Mono", "SFMono-Regular", ui-monospace, Consolas,
  "Liberation Mono", monospace;
```

**不引入 Web Font**：系统字体栈在 Windows / macOS 上已是高质量现代无衬线体，
引入外部字体的渲染阻塞与网络不可达风险大于收益。
现代感由字号阶梯、字重与字距纪律提供，而非字体本身。

### 4.2 字号阶梯

| 场景 | 类名 | 字号 / 行高 | 字重 |
| --- | --- | --- | --- |
| 页面标题 | `text-xl` | 20 / 28 | 600 |
| 弹窗标题、登录标题 | `text-xl` ~ `text-2xl` | 20–24 / 28–32 | 600 |
| 数据大数 | `.stat-card-value` | 26 / 32 | 600 |
| 卡片标题 | `text-[15px]` | 15 / 22 | 600 |
| 正文 | 基础样式 | 14 / 21 | 400 |
| 次要说明 | `text-xs` | 12 / 18 | 400 / 500 |
| 大写标签 | `.label-caps` / `text-2xs` | 11 / 16 | 600 + `0.06em` |

### 4.3 字距

- 中文标题**不做负字距**（中文字面宽，收紧会挤），仅拉丁文受 `--tracking-heading: -0.011em` 影响。
- 大号数字（≥ 20px）额外收紧至 `-0.02em`，小字号不做收紧。
- 全大写小标签统一 `0.06em` 正字距，避免小字挤成一团。

### 4.4 数字与规则

- **所有数字必须带 `.tabular`**：金额、计数、日期、百分比。
  表格中数字列纵向对齐，是数据密集型界面最基本的要求。
- 正文行高 21px（1.5），长段落文字行高 24–26px（`leading-6`）。
- 单行文字超过约 90 个字符时须限宽（`max-w-2xl` / `max-w-prose`）。

---

## 5. 间距系统

基于 4px 网格，Tailwind 默认间距刻度可直接使用。

| 令牌 | 值 | 用途 |
| --- | --- | --- |
| `1` | 4px | 图标与文字、紧凑内联间距 |
| `1.5` | 6px | 标签内边距 |
| `2` | 8px | 菜单项间距、按钮内图标间距 |
| `2.5` | 10px | 菜单项内图标与文字间距 |
| `3` | 12px | 表单字段内边距、卡片之间（紧凑） |
| `4` | 16px | 卡片之间、卡片内边距（紧凑） |
| `5` | 20px | 卡片内边距（默认）、页面区块之间 |
| `6` | 24px | 页面级留白 |

### 规则

- 卡片之间一律 `gap-4`；页面区块之间 `space-y-5`。
- 卡片内边距：有头部的卡片头部 `px-5 py-4`、主体 `p-5`；
  表格类内容用 `flush` 形态，由表格自身控制内边距。
- 页面内容区留白：移动端 `px-4 py-5`，`sm` 以上 `px-6 py-6`。
- 相邻同类元素用 `gap` 而非 `margin`，避免 margin 折叠带来的不可预期间距。

---

## 6. 圆角与阴影

### 6.1 圆角

| 令牌 | 值 | 用途 |
| --- | --- | --- |
| `--radius-sm` | 6px | 小标签、分段项、事件条 |
| `--radius-md` | 8px | **按钮、输入框、菜单项（默认基准）** |
| `--radius-lg` | 10px | 卡片、提示条、日历容器 |
| `--radius-xl` | 14px | 弹窗、抽屉、快捷导航面板 |
| `--radius-full` | 9999px | 徽章、头像、圆点、进度条 |

嵌套圆角遵循「内小外大」：弹窗 14px 时内部按钮用 8px，视觉上才同心。

### 6.2 阴影

阴影为**分层柔和投影**，用于表达浮起层级，不用于装饰。

| 令牌 | 用途 |
| --- | --- |
| `--shadow-xs` | 卡片、默认按钮 |
| `--shadow-sm` | 卡片悬停 |
| `--shadow-md` | 下拉菜单、卡片强悬停 |
| `--shadow-lg` | 轻提示、下拉面板 |
| `--shadow-xl` | 弹窗、抽屉、快捷导航 |

### 规则

- 卡片**默认带 `--shadow-xs`**，但层级主要仍由 hairline 边框表达。
- 悬停提升遵循「边框变深 + 阴影升一级」，两项同时变化，不要只改阴影。
- 暗色模式下阴影加深（`rgb(0 0 0 / .4)` ~ `.7`），因为深色底上浅阴影不可见。
- 禁止彩色阴影（如 `shadow-primary/20`），视觉廉价且破坏中性基调。

---

## 7. 动效规范

### 7.1 时长

| 令牌 | 值 | 用途 |
| --- | --- | --- |
| `--duration-fast` | 120ms | 颜色、背景、透明度变化 |
| `--duration-base` | 180ms | 常规过渡、按钮、输入框 |
| `--duration-slow` | 280ms | 抽屉滑入、面板展开 |

### 7.2 缓动

| 令牌 | 曲线 | 用途 |
| --- | --- | --- |
| `--ease-out` | `cubic-bezier(.16, 1, .3, 1)` | **默认**，进入 / 出现 |
| `--ease-in` | `cubic-bezier(.4, 0, 1, 1)` | 离开 / 消失 |
| `--ease-in-out` | `cubic-bezier(.4, 0, .2, 1)` | 位移往返 |
| `--ease-spring` | `cubic-bezier(.34, 1.4, .64, 1)` | 轻量回弹（开关、选中） |

### 7.3 规则

- 只对 `color` / `background-color` / `border-color` / `box-shadow` / `transform` / `opacity` 做过渡；
  禁止过渡 `width` / `height` / `top` / `left`（合成器不友好）。
  例外：侧边栏宽度折叠是刻意的布局动效，允许过渡 `width`。
- 动效时长上限 280ms，超过会显得迟钝。
- 必须尊重 `prefers-reduced-motion`（已在 `tokens.css` 全局处理）。
- 加载态一律使用骨架屏或按钮内联 spinner，不使用全屏遮罩。

---

## 8. 布局栅格

### 8.1 后台布局结构

```
┌────────────┬──────────────────────────────────────┐
│ 品牌区      │  顶栏：面包屑 · ⌘K · 通知 · 主题 · 用户 │  ← 56px
├────────────┼──────────────────────────────────────┤
│ 侧边栏      │  内容区                               │
│ 概览        │   ┌─ 页面头（标题 + 说明 + 操作）      │
│ 管理        │   └─ 主体（卡片网格 / 表格）          │
│ 系统        │                                      │
└────────────┴──────────────────────────────────────┘
   232px                 自适应（内容最大 1440px）
```

关键点：

- **侧边栏全高常驻**，品牌区高度与顶栏一致，与顶栏形成 L 形分隔。
- **顶栏只覆盖内容区**，不横跨整个页面宽度。
- 侧边栏折叠态 68px（仅图标 + `title` 提示），展开态 232px，状态持久化到 `localStorage`。
- 内容区限宽 `--content-max-width: 1440px`，超宽屏下居中，避免视线跨度失控。

### 8.2 断点

采用 Tailwind 默认断点：

| 断点 | 宽度 | 行为 |
| --- | --- | --- |
| `sm` | ≥ 640px | 页面内容区左右留白从 16px 增至 24px |
| `md` | ≥ 768px | 侧边栏从抽屉态转为常驻态；卡片网格开始多列 |
| `lg` | ≥ 1024px | 顶栏显示完整搜索框；三栏布局生效 |
| `xl` | ≥ 1280px | 指标卡一屏四列；辅助列显示 |

### 8.3 响应式规则

- 卡片网格一律「移动单列 → `sm` 两列 → `lg` 目标列数」。
- `< md` 时侧边栏为覆盖式抽屉：`fixed` + 向左平移隐藏，配合半透明遮罩，
  顶栏出现汉堡按钮；`md` 以上恢复 `sticky` 常驻。
- 表格用 `overflow-x-auto` 横向滚动，不做列隐藏（隐藏列会让数据无法对应）。
- 触摸目标最小 32×32px；`h-8` 及以下的按钮仅用于非主要操作。

---

## 9. 组件规范

### 9.1 尺寸体系

| 尺寸 | 高度 | 字号 | 用途 |
| --- | --- | --- | --- |
| `sm` | 32px | 12px | 表格内操作、紧凑工具条 |
| `md` | 36px | 14px | **默认**：表单、按钮、工具条 |
| `lg` | 40px | 14px | 主行动点、登录表单 |
| 图标按钮 | 32 / 36 / 40px | — | 正方形，图标 16–18px |

### 9.2 状态规范

| 状态 | 表现 |
| --- | --- |
| 悬停 | 背景升一级（`surface` → `surface-hover`）或边框变深 |
| 激活 / 按下 | 背景再升一级，或主色转为 `--color-primary-active` |
| 选中 | 主色浅底 + 主色文字（`menu-item-active` / `tab-item-active`） |
| 焦点 | 仅键盘可见：2px 主色轮廓 + 2px offset，或 `--color-primary-ring` 投影 |
| 禁用 | `opacity-45` + `pointer-events-none` + `cursor-not-allowed` |
| 加载 | 按钮内联 spinner + `aria-busy`；内容区用骨架屏 |
| 错误 | `border-danger` + 输入框下方红色说明 + `aria-invalid` |

### 9.3 组件清单（10 个文件 / 6 个分类）

| 分类 | 文件 | 组件 |
| --- | --- | --- |
| 基础 | `button.html` | `render`、`icon_only`、`link` |
| 基础 | `input.html` | `render`、`textarea`、`select` |
| 基础 | `form-controls.html` | `checkbox`、`radio`、`switch`、`segmented` |
| 展示 | `display.html` | `render`、`with_actions`、`badge`、`tag`、`avatar`、`alert`、`progress`、`skeleton`、`empty`、`stat_row` |
| 导航 | `navigation.html` | `breadcrumb`、`tabs`、`pagination`、`dropdown`、`menu`、`command_palette` |
| 导航 | `page.html` | `header`、`section` |
| 反馈 | `feedback.html` | `render`、`with_footer`、`drawer`、`toast_container`、`tooltip`、`confirm_dialog` |
| 数据 | `data.html` | `render`、`table`、`form`、`group`、`actions`、`stat_card`、`stat_group` |
| 数据 | `chart.html` | `area`、`sparkline`、`bars` |
| 业务 | `calendar.html` | `render`、`toolbar`、`notice` |

> 组件注册表见 `src/template/component.rs::BUILTIN_COMPONENTS`，
> 新增组件须同步登记，`/admin/ui-kit` 会自动列出。

### 9.4 关键组件视觉规范

#### 顶栏（Topbar）

- 高度 56px，白底，下方 1px `--color-border`。
- 左：面包屑（当前页为 600 字重主文字，其余为 `text-muted` 可点击链接）。
- 右：搜索框（`w-64`，`surface-inset` 底，含 ⌘K 提示）→ 设计规范 → 通知 → 主题 → 分隔线 → 用户。
- 图标按钮统一 36×36px、圆角 8px、图标 18px、`stroke-width: 1.8`。
- 未读提示用主色环包边的红点（`ring-2 ring-topbar`）与顶栏底色贴合。

#### 侧边栏菜单项（Menu Item）

- 高度 34px，圆角 8px，图标 18px，字号 13.5px。
- 默认 `text-secondary`；悬停 `surface-hover`；选中 `primary-subtle` + 主色文字 + 500 字重。
- 分组标题：11px / 600 字重 / 0.06em 正字距 / `text-muted` / 组间距 20px。
- 折叠态仅显示图标并居中，通过 `title` 提供文字提示。

#### 视图切换（Segmented）

- 容器 `surface-active` 底 + 3px 内边距 + 圆角 8px，形成「凹槽」。
- 激活项为白底药丸 + `shadow-xs` + 500 字重；非激活项仅文字变化。
- 用于同页视图切换（全部 / 启用 / 待激活…），**不用于跨页导航**。
- 计数以 `text-2xs text-muted` 跟在标签后，不使用彩色徽章。

#### 页面头（Page Header）

- 标题 20px / 600 字重 / 收紧字距；说明 14px `text-muted`，限宽 `max-w-2xl`。
- 右侧操作区与标题垂直对齐，主行动点在最右，最多 2 个按钮。

#### 指标卡（StatCard）

- 白底卡片 + hairline 边框 + `--shadow-xs`，内边距 16px。
- 标签 12px `text-secondary`；数值 26px / 600 / 收紧字距 / 等宽数字。
- 右上角 32×32px 图标块，圆角 8px，底色取「数据可视化色」的 `-bg` / `-fg` 配对。
- 涨跌胶囊：`success-subtle` 或 `danger-subtle` 底 + 同源文字 + 趋势箭头。
- 可选迷你趋势线（`spark`），颜色与图标块一致，整体 `opacity-70`。

#### 面积图（Chart）

- 纯 SVG，`viewBox="0 0 720 240"` + `preserveAspectRatio="none"` 横向自适应。
- 线条使用 `vector-effect="non-scaling-stroke"`，防止横向拉伸压扁线宽。
- **文字必须放在 SVG 之外**：横向拉伸会同时压扁字形。
  Y 轴标签用绝对定位 HTML，X 轴标签用 `flex justify-between` 均分。
- 网格线用 `--color-border`，1px；面积填充为同色 10% 透明度。
- 几何数据（path 字符串）由 Rust 侧预计算，模板只负责渲染，
  避免在模板里做浮点运算。

#### 数据表格（Table）

- 表头 12px / 600 字重 / `text-muted` / `surface-inset` 底 / `sticky top-0`。
- 行高约 44px（`py-3` + `px-4`），以 hairline 分隔，最后一行无下边框。
- 悬停整行高亮 `surface-hover`；数字列右对齐并带 `.tabular`。
- 空态居中显示图标 + 文案 + 可选主操作。

#### 快捷导航（Command Palette）

- 由 `⌘K` / `Ctrl+K` 触发，非输入态下 `/` 也可呼出。
- 宽 576px（`max-w-xl`），顶部下落 10vh，圆角 14px。
- 输入框自动聚焦；`↑` `↓` 移动，`↵` 打开，`Esc` 关闭；鼠标悬停同步高亮项。
- 数据源为布局下发的 `#ada-nav-index` JSON，包含标签、路径与所属分组。

#### 排班日历（Calendar）

- 7 列网格，星期表头 `surface-inset` 底 + 12px 字重 600。
- 日期数字 12px / 600；节假日用 `text-danger`。
- 事件条圆角 6px、11px 字号、浅底同源文字；时间右对齐并以 `opacity-75` 弱化。

---

## 10. 无障碍规范

### 10.1 对比度

- 正文与浅底：≥ 4.5:1（`--color-text` 于 `--color-surface` 约 15:1）。
- 大字号（≥ 18.66px 或 14px 加粗）：≥ 3:1。
- 次要文字（`text-secondary`）仍须满足 4.5:1；`text-muted` 只用于辅助信息，不承载关键内容。
- 功能色在浅色模式下均经过加深（如 success `#12864f`），保证浅底上的可读性。

### 10.2 键盘可达

- 所有交互元素可 Tab 到达，顺序与视觉顺序一致。
- 焦点环统一为 2px 主色 + 2px offset，**不得移除 outline**。
- 弹窗与抽屉：`Esc` 关闭，打开后焦点移入内部。
- 快捷导航：`⌘K` 打开、方向键选择、`↵` 打开、`Esc` 关闭。
- 折叠侧边栏按钮带 `aria-expanded`。

### 10.3 语义化

- 图标按钮必须有 `aria-label`；纯装饰图标加 `aria-hidden="true"`。
- 表格用原生 `<table>` / `<thead>` / `<tbody>` / `<th>`。
- 分组导航用 `<nav aria-label="…">`，当前项加 `aria-current="page"`。
- 异步反馈容器 `aria-live="polite"`；错误提示 `role="alert"`。
- 表单控件用 `<label for>` 关联，错误时补 `aria-invalid` 与 `aria-describedby`。

### 10.4 暗色模式

- 由 `<html data-theme="dark">` 驱动，令牌值在 `tokens.css` 中成对定义。
- 主题在 `<head>` 内联脚本中于首次绘制前应用，避免闪烁；
  用户选择持久化到 `localStorage`（键 `ada-theme`），未选择时跟随系统。
- 暗色下必须重新审视：主色需提亮、阴影需加深、功能色需提高明度。
- 两套主题的**信息层级必须对等**，不允许某个元素在暗色下消失。

---

## 11. 命名约定

### 11.1 CSS 变量

```
--color-{语义}[-{变体}]       --color-primary-subtle, --color-text-secondary
--radius-{尺寸}               --radius-md
--shadow-{尺寸}               --shadow-lg
--duration-{速度}             --duration-base
--ease-{类型}                 --ease-out
--font-{用途}                 --font-sans
--{区块}-{属性}               --sidebar-width, --topbar-height
```

### 11.2 组件模板

- 文件名用 `kebab-case`，放于 `templates/components/`。
- 页面布局放 `templates/layouts/`，后台页面放 `templates/admin/`。
- 每个组件文件头部必须有注释块，说明用途与参数。

### 11.3 组件参数

- 参数名用 `snake_case`，与模板内变量一致。
- 变体参数统一叫 `variant`，尺寸统一叫 `size`，透传属性统一叫 `attrs`。
- 回调内容通过 `{% call %}` + `caller()` 注入，不拼接 HTML 字符串
  （唯一例外是需要预先构造的按钮 HTML，用 `~` 拼接后以 `| safe` 传入）。

### 11.4 模板中的三条硬性约束

1. **`x-data` 属性内部只能用单引号。**
   `x-data` 由双引号包裹，若内部出现未转义的双引号（例如 `tojson` 的输出），
   属性会提前闭合，Alpine 静默初始化失败——页面返回 200 但交互全部失效。
   `tests/template_render.rs::assert_x_data_well_formed` 会拦截这类问题。

2. **不要写 `x-init="init()"`。**
   `Alpine.data()` 注册的组件其 `init()` 由 Alpine 自动调用，
   再写一次会导致初始化逻辑（事件监听、数据加载）执行两遍。

3. **组件文件与上下文变量不要重名。**
   `{% import "components/chart.html" as chart %}` 会遮蔽同名上下文变量
   `chart`，导致 `chart.area(chart=chart)` 把模块自身当作数据传入。
   导入别名统一用易区分的名字（如 `charts`）。

---

## 附录：令牌速查

```
主色    --color-primary(#5b5bd6) / -hover / -active / -subtle / -muted / -fg / -ring
外壳    --color-topbar-bg / -fg / -muted / -hover / -active
中性    --color-bg / surface(-hover,-active,-inset) / border(-strong)
        --color-text / -secondary / -muted / -inverse
功能    --color-{success|warning|danger|info}[-subtle|-muted|-hover|-fg]
数据    --color-stat-{blue|orange|green|purple|red}-{bg|fg}
圆角    --radius-{sm(6)|md(8)|lg(10)|xl(14)|full}
阴影    --shadow-{xs|sm|md|lg|xl}
动效    --duration-{fast(120)|base(180)|slow(280)}  --ease-{out|in|in-out|spring}
字体    --font-sans / --font-mono / --tracking-heading
布局    --sidebar-width(232) / -collapsed(68) / --topbar-height(56)
        --tabbar-height(36，分段控件高度) / --content-max-width(1440)
```
