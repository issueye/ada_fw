/**
 * Ada Framework 客户端交互层。
 *
 * 约定：所有 Alpine 组件通过 Alpine.data() 注册，
 * 数据交互统一走 /api/* JSON 接口。
 */

document.addEventListener('alpine:init', () => {
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
     全��反馈：Toast + Confirm
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

    /** 弹出二次确认。 */
    confirm(title, message, onConfirm) {
      this.confirmState = { open: true, title, message, onConfirm };
    },
  }));

  /* ======================================================================
     用户管理：列表 + 搜索 + 删除
     ====================================================================== */
  Alpine.data('userManager', () => ({
    users: [],
    keyword: '',
    loading: false,
    error: '',

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

    async load() {
      this.loading = true;
      this.error = '';
      try {
        this.users = await this.request('/api/users');
      } catch (e) {
        this.error = e.message;
      } finally {
        this.loading = false;
      }
    },

    async remove(user) {
      const fb = this.$root._x_dataStack?.find((d) => d.confirm);
      try {
        await this.request(`/api/users/${user.id}`, { method: 'DELETE' });
        this.users = this.users.filter((u) => u.id !== user.id);
        this.$dispatch('toast', { title: '删除成功', message: `已删除用户 ${user.name}`, type: 'success' });
      } catch (e) {
        this.error = e.message;
        this.$dispatch('toast', { title: '删除失败', message: e.message, type: 'danger' });
      }
    },

    /** 统一请求封装，自动处理错误。 */
    async request(url, options = {}) {
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
