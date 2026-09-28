<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  let { onNavigate }: { onNavigate: (page: string) => void } = $props();

  let length = $state(16);
  let uppercase = $state(true);
  let lowercase = $state(true);
  let numbers = $state(true);
  let symbols = $state(true);
  let generatedPassword = $state('');
  let loading = $state(false);

  async function generate() {
    if (!uppercase && !lowercase && !numbers && !symbols) {
      alert('请至少选择一种字符集');
      return;
    }

    loading = true;
    try {
      generatedPassword = await invoke<string>('generate_password', {
        length,
        uppercase,
        lowercase,
        numbers,
        symbols,
      });
    } catch (e) {
      console.error('Generate failed:', e);
    } finally {
      loading = false;
    }
  }

  async function copyPassword() {
    if (!generatedPassword) return;
    try {
      await navigator.clipboard.writeText(generatedPassword);
      alert('已复制');
    } catch {
      const textarea = document.createElement('textarea');
      textarea.value = generatedPassword;
      document.body.appendChild(textarea);
      textarea.select();
      document.execCommand('copy');
      document.body.removeChild(textarea);
      alert('已复制');
    }
  }

  $effect(() => {
    generate();
  });
</script>

<div class="max-w-lg mx-auto p-4">
  <div class="bg-white rounded-lg border border-gray-200">
    <div class="px-4 py-3 border-b border-gray-200 flex items-center justify-between">
      <h2 class="font-medium text-sm">密码生成器</h2>
      <button
        onclick={() => onNavigate('main')}
        class="text-gray-400 hover:text-gray-600 transition"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
        </svg>
      </button>
    </div>

    <div class="p-4 space-y-5">
      <!-- 长度 -->
      <div>
        <div class="flex justify-between items-center mb-2">
          <label class="text-xs font-medium text-gray-600">长度</label>
          <span class="text-sm font-mono">{length}</span>
        </div>
        <input
          type="range"
          min="8"
          max="64"
          bind:value={length}
          class="w-full h-1.5 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-gray-900"
        />
        <div class="flex justify-between text-xs text-gray-400 mt-1">
          <span>8</span>
          <span>64</span>
        </div>
      </div>

      <!-- 字符集 -->
      <div>
        <label class="text-xs font-medium text-gray-600 mb-2 block">字符集</label>
        <div class="grid grid-cols-2 gap-2">
          <label class="flex items-center gap-2 p-2 bg-gray-50 rounded-lg cursor-pointer hover:bg-gray-100 transition">
            <input type="checkbox" bind:checked={uppercase} class="w-3.5 h-3.5 text-gray-900 bg-gray-200 border-gray-300 rounded" />
            <span class="text-xs">大写字母 A-Z</span>
          </label>
          <label class="flex items-center gap-2 p-2 bg-gray-50 rounded-lg cursor-pointer hover:bg-gray-100 transition">
            <input type="checkbox" bind:checked={lowercase} class="w-3.5 h-3.5 text-gray-900 bg-gray-200 border-gray-300 rounded" />
            <span class="text-xs">小写字母 a-z</span>
          </label>
          <label class="flex items-center gap-2 p-2 bg-gray-50 rounded-lg cursor-pointer hover:bg-gray-100 transition">
            <input type="checkbox" bind:checked={numbers} class="w-3.5 h-3.5 text-gray-900 bg-gray-200 border-gray-300 rounded" />
            <span class="text-xs">数字 0-9</span>
          </label>
          <label class="flex items-center gap-2 p-2 bg-gray-50 rounded-lg cursor-pointer hover:bg-gray-100 transition">
            <input type="checkbox" bind:checked={symbols} class="w-3.5 h-3.5 text-gray-900 bg-gray-200 border-gray-300 rounded" />
            <span class="text-xs">符号 !@#$</span>
          </label>
        </div>
      </div>

      <!-- 生成按钮 -->
      <button
        onclick={generate}
        disabled={loading}
        class="w-full py-2 bg-gray-900 hover:bg-gray-800 text-white rounded-lg text-sm font-medium transition disabled:opacity-50"
      >
        {loading ? '生成中...' : '生成'}
      </button>

      <!-- 结果 -->
      {#if generatedPassword}
        <div class="bg-gray-50 rounded-lg p-3 border border-gray-200">
          <div class="flex items-center gap-2">
            <code class="flex-1 text-sm font-mono break-all">{generatedPassword}</code>
            <button
              onclick={copyPassword}
              class="p-1.5 text-gray-400 hover:text-gray-900 hover:bg-gray-200 rounded transition shrink-0"
              title="复制"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"/>
              </svg>
            </button>
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>
