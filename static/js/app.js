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
     后台布局：侧边栏折叠 + 主题切换
     ====================================================================== */
  Alpine.data('adminLayout', () => ({
    sidebarCollapsed: false,
    theme: 'light',

    init() {
      // 侧边栏状态持久化
      this.sidebarCollapsed = localStorage.getItem('ada-sidebar') === 'collapsed';
      // 主题从 <html data-theme> 读取（已在 head 中初始化）
      this.theme = document.documentElement.getAttribute('data-theme') || 'light';
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
    confirmState: { open: false, title: '', message: '', onConfirm: null },

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

    /** 弹出二次确认。确认后派发 ada-confirm-ok 事件。 */
    confirm(title, message) {
      this.confirmState = {
        open: true,
        title,
        message,
        onConfirm: () => this.$dispatch('ada-confirm-ok'),
      };
    },
  }));

  /* ======================================================================
     用户管理：列表 + 搜索 + 新建 + 删除
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

  Alpine.data('userManager', () => ({
    users: [],
    keyword: '',
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
      if (!kw) return this.users;
      return this.users.filter(
        (u) =>
          u.name.toLowerCase().includes(kw) ||
          u.email.toLowerCase().includes(kw)
      );
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

    async remove(user) {
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
