<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  let { onSetup }: { onSetup: () => void } = $props();

  let password = $state('');
  let confirmPassword = $state('');
  let error = $state('');
  let loading = $state(false);

  async function handleSubmit() {
    error = '';
    
    if (!password) {
      error = '请输入主密码';
      return;
    }
    if (password !== confirmPassword) {
      error = '两次密码不一致';
      return;
    }
    if (password.length < 6) {
      error = '密码至少6位';
      return;
    }

    loading = true;
    try {
      await invoke('setup_master_password', { password });
      onSetup();
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }
</script>

<div class="min-h-screen flex items-center justify-center p-4">
  <div class="w-full max-w-sm">
    <div class="text-center mb-8">
      <div class="w-16 h-16 bg-gray-900 rounded-2xl mx-auto mb-4 flex items-center justify-center">
        <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
        </svg>
      </div>
      <h2 class="text-xl font-semibold">Zeno</h2>
      <p class="text-gray-500 text-sm mt-1">本地凭据管理</p>
    </div>

    <p class="text-center text-gray-500 text-sm mb-4">首次使用，请设置主密码</p>
    <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="space-y-3">
      <input
        type="password"
        bind:value={password}
        placeholder="设置主密码"
        class="w-full px-4 py-2.5 bg-white border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
      />
      <input
        type="password"
        bind:value={confirmPassword}
        placeholder="确认密码"
        class="w-full px-4 py-2.5 bg-white border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
      />
      {#if error}
        <p class="text-red-500 text-xs text-center">{error}</p>
      {/if}
      <button
        type="submit"
        disabled={loading}
        class="w-full py-2.5 bg-gray-900 hover:bg-gray-800 text-white rounded-lg font-medium text-sm transition disabled:opacity-50"
      >
        {loading ? '处理中...' : '创建主密码'}
      </button>
    </form>
    <p class="text-xs text-gray-400 text-center mt-3">主密码丢失将无法找回</p>
  </div>
</div>
