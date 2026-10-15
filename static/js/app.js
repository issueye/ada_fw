/**
 * Ada Framework 客户端交互层。
 *
 * 约定：所有 Alpine 组件通过 Alpine.data() 注册，
 * 数据交互统一走 /api/* JSON 接口。
 *
 * 注意：本脚本必须在 Alpine 核心脚本之前加载（同为 defer 时按顺序执行），
 * 否则 alpine:init 事件会在注册前派发，导致组件未定义。
 */

document.addEventListener('alpine:init', () => {
  if (typeof Alpine === 'undefined') {
    console.error('[ada] Alpine 未加载，组件注册失败');
    return;
  }

  /* ======================================================================
     公共请求封装
     统一处理 JSON 头、错误体解析与 204 空响应，供各组件复用。
     ====================================================================== */
  async function apiRequest(url, options = {}) {
    const res = await fetch(url, {
      headers: { 'Content-Type': 'application/json' },
      ...options,
    });
    if (!res.ok) {
      let message = `请求失败 (${res.status})`;
      try {
        const body = await res.json();
        message = body?.error?.message || message;
      } catch (_) { /* 忽略解析错误 */ }
      throw new Error(message);
    }
    return res.status === 204 ? null : res.json();
  }

  /* ======================================================================
     后台布局：侧边栏折叠 / 移动端抽屉 / 主题切换 / 快捷导航（⌘K）
     ====================================================================== */
  Alpine.data('adminLayout', () => ({
    sidebarCollapsed: false,
    /** 移动端（<768px）侧边栏抽屉是否展开。 */
    mobileNav: false,
    theme: 'light',

    /** 快捷导航状态。 */
    paletteOpen: false,
    paletteQuery: '',
    paletteIndex: 0,
    paletteItems: [],

    init() {
      // 侧边栏状态持久化
      this.sidebarCollapsed = localStorage.getItem('ada-sidebar') === 'collapsed';
      // 主题从 <html data-theme> 读取（已在 head 中初始化）
      this.theme = document.documentElement.getAttribute('data-theme') || 'light';
      // 快捷导航数据源由布局模板以 JSON 脚本下发
      this.paletteItems = this.readNavIndex();

      this.onKeydown = (e) => {
        // ⌘K / Ctrl+K 打开快捷导航
        if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
          e.preventDefault();
          this.paletteOpen ? this.closePalette() : this.openPalette();
          return;
        }
        // "/" 快捷呼出（输入框内不触发）
        if (e.key === '/' && !this.paletteOpen) {
          const tag = (e.target?.tagName || '').toLowerCase();
          if (tag === 'input' || tag === 'textarea' || e.target?.isContentEditable) return;
          e.preventDefault();
          this.openPalette();
        }
      };
      window.addEventListener('keydown', this.onKeydown);
    },

    destroy() {
      window.removeEventListener('keydown', this.onKeydown);
    },

    /** 读取布局下发的导航索引。 */
    readNavIndex() {
      const el = document.getElementById('ada-nav-index');
      if (!el) return [];
      try {
        return JSON.parse(el.textContent) || [];
      } catch (e) {
        console.warn('[ada] 解析快捷导航数据失败', e);
        return [];
      }
    },

    toggleSidebar() {
      this.sidebarCollapsed = !this.sidebarCollapsed;
      localStorage.setItem('ada-sidebar', this.sidebarCollapsed ? 'collapsed' : 'expanded');
    },

    toggleTheme() {
      this.theme = this.theme === 'dark' ? 'light' : 'dark';
      document.documentElement.setAttribute('data-theme', this.theme);
      localStorage.setItem('ada-theme', this.theme);
    },

    /** 全屏切换。 */
    toggleFullscreen() {
      if (document.fullscreenElement) {
        document.exitFullscreen();
      } else {
        document.documentElement.requestFullscreen?.();
      }
    },

    // ---- 快捷导航 ----

    openPalette() {
      this.paletteQuery = '';
      this.paletteIndex = 0;
      this.paletteOpen = true;
      this.$nextTick(() => this.$refs.paletteInput?.focus());
    },

    closePalette() {
      this.paletteOpen = false;
    },

    /** 过滤后的跳转目标（最多 8 条）。 */
    get paletteResults() {
      const q = this.paletteQuery.trim().toLowerCase();
      if (!q) return this.paletteItems.slice(0, 8);
      return this.paletteItems
        .filter((i) =>
          (i.label || '').toLowerCase().includes(q) ||
          (i.href || '').toLowerCase().includes(q) ||
          (i.group || '').toLowerCase().includes(q)
        )
        .slice(0, 8);
    },

    movePalette(delta) {
      const total = this.paletteResults.length;
      if (total === 0) return;
      this.paletteIndex = (this.paletteIndex + delta + total) % total;
    },

    runPalette() {
      const item = this.paletteResults[this.paletteIndex];
      if (item) this.goPalette(item);
    },

    goPalette(item) {
      this.closePalette();
      // 已在当前页则不触发无意义的跳转
      if (item.href && item.href !== window.location.pathname) {
        window.location.href = item.href;
      }
    },
  }));

  /* ======================================================================
     全局反馈：Toast + Confirm
     通过 window 事件触发，与调用方解耦：
       $dispatch('ada-toast',   { title, message, type })
       $dispatch('ada-confirm', { title, message })
     确认后派发 ada-confirm-ok 事件。
     ====================================================================== */
  Alpine.data('feedback', () => ({
    toasts: [],
    confirmState: { open: false, title: '', message: '', payload: null, onConfirm: null },

    /** 弹出轻提示。type: info | success | warning | danger */
    toast(title, message = '', type = 'info', duration = 3000) {
      const id = Date.now() + Math.random();
      this.toasts.push({ id, title, message, type });
      if (duration > 0) {
        setTimeout(() => this.dismissToast(id), duration);
      }
    },

    dismissToast(id) {
      this.toasts = this.toasts.filter((t) => t.id !== id);
    },

    /**
     * 弹出二次确认。确认后派发 ada-confirm-ok 事件，
     * 并把调用方随附的 payload 原样回传（用于定位待处理对象）。
     */
    confirm(title, message, payload = null) {
      this.confirmState = {
        open: true,
        title,
        message,
        payload,
        onConfirm: () => this.$dispatch('ada-confirm-ok', payload),
      };
    },
  }));

  /* ======================================================================
     登录表单
     ====================================================================== */
  Alpine.data('loginForm', () => ({
    email: '',
    password: '',
    error: '',
    submitting: false,

    async submit() {
      if (this.submitting) return;
      this.submitting = true;
      this.error = '';
      try {
        await apiRequest('/admin/login', {
          method: 'POST',
          body: JSON.stringify({ email: this.email, password: this.password }),
        });
        // 登录成功：跳转到 next 参数指定页，默认 /admin
        const next = new URLSearchParams(window.location.search).get('next');
        window.location.href = next && next.startsWith('/') ? next : '/admin';
      } catch (e) {
        this.error = e.message;
      } finally {
        this.submitting = false;
      }
    },
  }));

  /* ======================================================================
     用户管理：列表 + 搜索 + 状态筛选 + 分页 + 新建 + 删除
     ====================================================================== */
  Alpine.data('userManager', () => ({
    users: [],
    keyword: '',
    /** 状态筛选（分段控件）：all | active | pending | inactive */
    statusFilter: 'all',
    loading: false,
    error: '',
    /** 当前打开的弹窗 id，null 表示全部关闭。 */
    openModal: null,
    /** 新建用户表单。 */
    form: { name: '', email: '', role: 'viewer' },
    /** 提交中标记，防止重复提交。 */
    submitting: false,
    /** 当前页码（从 1 开始）。 */
    page: 1,
    /** 每页条数。 */
    perPage: 10,

    async init() {
      await this.load();
    },

    get filtered() {
      const kw = this.keyword.trim().toLowerCase();
      return this.users.filter((u) => {
        if (this.statusFilter !== 'all' && u.status !== this.statusFilter) return false;
        if (!kw) return true;
        return (
          u.name.toLowerCase().includes(kw) ||
          u.email.toLowerCase().includes(kw)
        );
      });
    },

    /** 总页数（至少 1 页）。 */
    get totalPages() {
      return Math.max(1, Math.ceil(this.filtered.length / this.perPage));
    },

    /** 当前页数据切片。 */
    get paged() {
      const start = (this.page - 1) * this.perPage;
      return this.filtered.slice(start, start + this.perPage);
    },

    /** 页码列表（最多 5 个，围绕当前页）。 */
    get pageNumbers() {
      const total = this.totalPages;
      let start = Math.max(1, this.page - 2);
      const end = Math.min(total, start + 4);
      start = Math.max(1, end - 4);
      const pages = [];
      for (let p = start; p <= end; p++) pages.push(p);
      return pages;
    },

    /** 搜索关键词变化时回到第一页。 */
    onSearch() {
      this.page = 1;
    },

    /** 切换状态筛选并回到第一页。 */
    setStatusFilter(value) {
      this.statusFilter = value;
      this.page = 1;
    },

    goTo(p) {
      if (p >= 1 && p <= this.totalPages) this.page = p;
    },

    async load() {
      this.loading = true;
      this.error = '';
      try {
        this.users = await apiRequest('/api/users');
      } catch (e) {
        this.error = e.message;
      } finally {
        this.loading = false;
      }
    },

    /** 打开新建用户弹窗并重置表单。 */
    openCreate() {
      this.form = { name: '', email: '', role: 'viewer' };
      this.error = '';
      this.openModal = 'createUser';
    },

    /** 提交新建用户。 */
    async create() {
      if (this.submitting) return;
      this.submitting = true;
      this.error = '';
      try {
        const user = await apiRequest('/api/users', {
          method: 'POST',
          body: JSON.stringify(this.form),
        });
        this.users.unshift(user);
        this.openModal = null;
        this.$dispatch('ada-toast', {
          title: '创建成功',
          message: `已创建用户 ${user.name}`,
          type: 'success',
        });
      } catch (e) {
        this.error = e.message;
        this.$dispatch('ada-toast', { title: '创建失败', message: e.message, type: 'danger' });
      } finally {
        this.submitting = false;
      }
    },

    /** 删除用户（先二次确认，确认结果经 ada-confirm-ok 回传）。 */
    remove(user) {
      this.$dispatch('ada-confirm', {
        title: '删除用户',
        message: `确认删除「${user.name}」？该操作不可撤销。`,
        payload: user,
      });
    },

    /** 二次确认回调：detail 即 dispatch 时随附的 payload。 */
    onConfirmOk(payload) {
      if (payload && payload.id) this.doRemove(payload);
    },

    async doRemove(user) {
      try {
        await apiRequest(`/api/users/${user.id}`, { method: 'DELETE' });
        this.users = this.users.filter((u) => u.id !== user.id);
        this.$dispatch('ada-toast', { title: '删除成功', message: `已删除用户 ${user.name}`, type: 'success' });
      } catch (e) {
        this.error = e.message;
        this.$dispatch('ada-toast', { title: '删除失败', message: e.message, type: 'danger' });
      }
    },
  }));

  /* ======================================================================
     待办清单（示例页）
     ====================================================================== */
  Alpine.data('todoApp', () => ({
    todos: [],
    draft: '',
    error: '',

    init() {
      const el = document.getElementById('initial-todos');
      if (el) {
        try {
          this.todos = JSON.parse(el.textContent) || [];
        } catch (e) {
          console.warn('解析初始待办数据失败', e);
          this.todos = [];
        }
      }
    },

    async request(url, options = {}) {
      this.error = '';
      return apiRequest(url, options);
    },

    async add() {
      const title = this.draft.trim();
      if (!title) return;
      try {
        const todo = await this.request('/api/todos', {
          method: 'POST',
          body: JSON.stringify({ title }),
        });
        this.todos.push(todo);
        this.draft = '';
      } catch (e) {
        this.error = e.message;
      }
    },

    async remove(todo) {
      try {
        await this.request(`/api/todos/${todo.id}`, { method: 'DELETE' });
        this.todos = this.todos.filter((t) => t.id !== todo.id);
      } catch (e) {
        this.error = e.message;
      }
    },

    async toggle(todo) {
      try {
        const updated = await this.request(`/api/todos/${todo.id}/toggle`, {
          method: 'PATCH',
        });
        const idx = this.todos.findIndex((t) => t.id === todo.id);
        if (idx !== -1) this.todos[idx] = updated;
      } catch (e) {
        this.error = e.message;
      }
    },
  }));
});
