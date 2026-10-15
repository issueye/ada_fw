/** @type {import('tailwindcss').Config} */
module.exports = {
  // 扫描 Rust 源码与模板，按需生成样式
  content: [
    './templates/**/*.html',
    './src/**/*.rs',
  ],
  // 暗色模式由 data-theme 属性驱动，与 tokens.css 保持一致
  darkMode: ['selector', '[data-theme="dark"]'],
  theme: {
    extend: {
      colors: {
        // ---- 语义色：映射到 CSS 变量 ----
        primary: {
          subtle: 'var(--color-primary-subtle)',
          muted: 'var(--color-primary-muted)',
          DEFAULT: 'var(--color-primary)',
          hover: 'var(--color-primary-hover)',
          active: 'var(--color-primary-active)',
          fg: 'var(--color-primary-fg)',
        },
        success: {
          subtle: 'var(--color-success-subtle)',
          muted: 'var(--color-success-muted)',
          DEFAULT: 'var(--color-success)',
          fg: 'var(--color-success-fg)',
        },
        warning: {
          subtle: 'var(--color-warning-subtle)',
          muted: 'var(--color-warning-muted)',
          DEFAULT: 'var(--color-warning)',
          fg: 'var(--color-warning-fg)',
        },
        danger: {
          subtle: 'var(--color-danger-subtle)',
          muted: 'var(--color-danger-muted)',
          DEFAULT: 'var(--color-danger)',
          hover: 'var(--color-danger-hover)',
          fg: 'var(--color-danger-fg)',
        },
        info: {
          subtle: 'var(--color-info-subtle)',
          muted: 'var(--color-info-muted)',
          DEFAULT: 'var(--color-info)',
          fg: 'var(--color-info-fg)',
        },
        // ---- 外壳：顶栏（中性白） ----
        topbar: {
          DEFAULT: 'var(--color-topbar-bg)',
          fg: 'var(--color-topbar-fg)',
          muted: 'var(--color-topbar-muted)',
          hover: 'var(--color-topbar-hover)',
          active: 'var(--color-topbar-active)',
        },
        // ---- 数据可视化色（图表 / 图标底 / 日历事件） ----
        stat: {
          'blue-bg': 'var(--color-stat-blue-bg)',
          'blue-fg': 'var(--color-stat-blue-fg)',
          'orange-bg': 'var(--color-stat-orange-bg)',
          'orange-fg': 'var(--color-stat-orange-fg)',
          'green-bg': 'var(--color-stat-green-bg)',
          'green-fg': 'var(--color-stat-green-fg)',
          'purple-bg': 'var(--color-stat-purple-bg)',
          'purple-fg': 'var(--color-stat-purple-fg)',
          'red-bg': 'var(--color-stat-red-bg)',
          'red-fg': 'var(--color-stat-red-fg)',
        },
        // ---- 中性色 ----
        bg: 'var(--color-bg)',
        surface: {
          DEFAULT: 'var(--color-surface)',
          hover: 'var(--color-surface-hover)',
          active: 'var(--color-surface-active)',
          inset: 'var(--color-surface-inset)',
        },
        border: {
          DEFAULT: 'var(--color-border)',
          strong: 'var(--color-border-strong)',
        },
        content: {
          DEFAULT: 'var(--color-text)',
          secondary: 'var(--color-text-secondary)',
          muted: 'var(--color-text-muted)',
          inverse: 'var(--color-text-inverse)',
        },
      },
      borderRadius: {
        sm: 'var(--radius-sm)',
        md: 'var(--radius-md)',
        lg: 'var(--radius-lg)',
        xl: 'var(--radius-xl)',
        full: 'var(--radius-full)',
      },
      boxShadow: {
        xs: 'var(--shadow-xs)',
        sm: 'var(--shadow-sm)',
        md: 'var(--shadow-md)',
        lg: 'var(--shadow-lg)',
        xl: 'var(--shadow-xl)',
      },
      transitionDuration: {
        fast: 'var(--duration-fast)',
        base: 'var(--duration-base)',
        slow: 'var(--duration-slow)',
      },
      transitionTimingFunction: {
        out: 'var(--ease-out)',
        in: 'var(--ease-in)',
        'in-out': 'var(--ease-in-out)',
        spring: 'var(--ease-spring)',
      },
      spacing: {
        sidebar: 'var(--sidebar-width)',
        'sidebar-collapsed': 'var(--sidebar-width-collapsed)',
        topbar: 'var(--topbar-height)',
        tabbar: 'var(--tabbar-height)',
        content: 'var(--content-max-width)',
      },
      fontFamily: {
        sans: ['var(--font-sans)'],
        mono: ['var(--font-mono)'],
      },
      fontSize: {
        // 后台专用小号字：11/16 用于大写标签，12/16 用于辅助信息
        '2xs': ['11px', { lineHeight: '16px' }],
      },
      keyframes: {
        'fade-in': {
          from: { opacity: '0' },
          to: { opacity: '1' },
        },
        'scale-in': {
          from: { opacity: '0', transform: 'scale(0.96) translateY(4px)' },
          to: { opacity: '1', transform: 'scale(1) translateY(0)' },
        },
        'slide-in-right': {
          from: { transform: 'translateX(100%)' },
          to: { transform: 'translateX(0)' },
        },
        'slide-in-left': {
          from: { transform: 'translateX(-100%)' },
          to: { transform: 'translateX(0)' },
        },
        'slide-in-up': {
          from: { transform: 'translateY(100%)' },
          to: { transform: 'translateY(0)' },
        },
        'slide-in-down': {
          from: { transform: 'translateY(-100%)' },
          to: { transform: 'translateY(0)' },
        },
        'toast-in': {
          from: { opacity: '0', transform: 'translateY(-10px) scale(0.98)' },
          to: { opacity: '1', transform: 'translateY(0) scale(1)' },
        },
        shimmer: {
          '100%': { transform: 'translateX(100%)' },
        },
        'draw-in': {
          from: { 'stroke-dashoffset': '1000' },
          to: { 'stroke-dashoffset': '0' },
        },
      },
      animation: {
        'fade-in': 'fade-in var(--duration-base) var(--ease-out)',
        'scale-in': 'scale-in var(--duration-base) var(--ease-out)',
        'slide-in-right': 'slide-in-right var(--duration-slow) var(--ease-out)',
        'slide-in-left': 'slide-in-left var(--duration-slow) var(--ease-out)',
        'slide-in-up': 'slide-in-up var(--duration-slow) var(--ease-out)',
        'slide-in-down': 'slide-in-down var(--duration-slow) var(--ease-out)',
        'toast-in': 'toast-in var(--duration-base) var(--ease-out)',
        shimmer: 'shimmer 1.4s infinite',
        'draw-in': 'draw-in 1s var(--ease-out) both',
      },
    },
  },
  plugins: [],
};
