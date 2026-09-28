<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { CATEGORY_FIELDS, type Category, type Record } from './types';

  let { record, onNavigate }: { record: Record; onNavigate: (page: string) => void } = $props();

  let title = $state(record.title);
  let fields = $state<Record<string, string>>({ ...record.fields });
  let error = $state('');
  let loading = $state(false);

  function isDatabaseCategory(): boolean {
    return record.category === 'database';
  }

  function getSortedEntries(): [string, string][] {
    const category = record.category as Category;
    const fieldOrder = CATEGORY_FIELDS[category] || [];
    const entries = Object.entries(fields);
    return entries.sort((a, b) => {
      const ai = fieldOrder.indexOf(a[0]);
      const bi = fieldOrder.indexOf(b[0]);
      const aIdx = ai === -1 ? 999 : ai;
      const bIdx = bi === -1 ? 999 : bi;
      return aIdx - bIdx;
    });
  }

  // 快速生成密码：16位、全字符集、字母开头、剔除易混淆字符
  function generateQuickPassword(): string {
    const uppercaseChars = 'ABCDEFGHJKLMNPQRSTUVWXYZ'; // 剔除 I, O
    const lowercaseChars = 'abcdefghjkmnpqrstuvwxyz'; // 剔除 i, l, o
    const numberChars = '23456789'; // 剔除 0, 1
    const symbolChars = '!@#$%^&*_+-=';

    const allChars = uppercaseChars + lowercaseChars + numberChars + symbolChars;
    const length = 16;

    // 第一个字符必须是字母
    const letterChars = uppercaseChars + lowercaseChars;
    const firstChar = letterChars[Math.floor(Math.random() * letterChars.length)];

    // 剩余字符随机生成
    let password = firstChar;
    for (let i = 1; i < length; i++) {
      password += allChars[Math.floor(Math.random() * allChars.length)];
    }
    return password;
  }

  function handleQuickGenerate() {
    fields['密码'] = generateQuickPassword();
  }

  async function handleSubmit() {
    error = '';
    
    if (!title.trim()) {
      error = '请输入标题';
      return;
    }

    loading = true;
    try {
      await invoke('update_record', {
        id: record.id,
        title: title.trim(),
        fields,
      });
      onNavigate('main');
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function handleDelete() {
    if (!confirm('确定要删除这条记录吗？')) return;
    
    loading = true;
    try {
      await invoke('delete_record', { id: record.id });
      onNavigate('main');
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }
</script>

<div class="max-w-lg mx-auto p-4">
  <div class="bg-white rounded-lg border border-gray-200">
    <div class="px-4 py-3 border-b border-gray-200 flex items-center justify-between">
      <h2 class="font-medium text-sm">编辑记录</h2>
      <button
        onclick={() => onNavigate('main')}
        class="text-gray-400 hover:text-gray-600 transition"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
        </svg>
      </button>
    </div>

    <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="p-4 space-y-4">
      <div>
        <label class="block text-xs font-medium text-gray-600 mb-1">类别</label>
        <input
          type="text"
          value={record.category}
          disabled
          class="w-full px-3 py-2 bg-gray-100 border border-gray-200 rounded-lg text-sm text-gray-500"
        />
        <p class="text-xs text-gray-400 mt-1">类别不可修改</p>
      </div>

      <div>
        <label class="block text-xs font-medium text-gray-600 mb-1">标题 <span class="text-red-500">*</span></label>
        <input
          type="text"
          bind:value={title}
          class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
        />
      </div>

      {#if isDatabaseCategory()}
        <!-- URL -->
        <div>
          <label class="block text-xs font-medium text-gray-600 mb-1">URL</label>
          <input
            type="text"
            bind:value={fields['URL']}
            class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
          />
        </div>

        <!-- Host + Port -->
        <div class="flex gap-4">
          <div class="flex-1">
            <label class="block text-xs font-medium text-gray-600 mb-1">Host</label>
            <input
              type="text"
              bind:value={fields['Host']}
              class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
            />
          </div>
          <div class="w-32">
            <label class="block text-xs font-medium text-gray-600 mb-1">Port</label>
            <input
              type="text"
              bind:value={fields['Port']}
              class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
            />
          </div>
        </div>

        <!-- 数据库 + Schema -->
        <div class="flex gap-4">
          <div class="flex-1">
            <label class="block text-xs font-medium text-gray-600 mb-1">数据库</label>
            <input
              type="text"
              bind:value={fields['数据库']}
              class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
            />
          </div>
          <div class="flex-1">
            <label class="block text-xs font-medium text-gray-600 mb-1">Schema</label>
            <input
              type="text"
              bind:value={fields['Schema']}
              class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
            />
          </div>
        </div>

        <!-- 用户名 -->
        <div>
          <label class="block text-xs font-medium text-gray-600 mb-1">用户名</label>
          <input
            type="text"
            bind:value={fields['用户名']}
            class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
          />
        </div>

        <!-- 密码 -->
        <div>
          <label class="block text-xs font-medium text-gray-600 mb-1">密码</label>
          <div class="flex gap-2">
            <input
              type="password"
              bind:value={fields['密码']}
              class="flex-1 px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
            />
            <button
              type="button"
              onclick={handleQuickGenerate}
              class="px-3 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 text-sm rounded-lg transition whitespace-nowrap"
              title="快速生成16位安全密码"
            >
              生成
            </button>
          </div>
        </div>
      {:else}
        {#each getSortedEntries() as [key, _]}
          <div>
            <label class="block text-xs font-medium text-gray-600 mb-1">{key}</label>
            {#if key.includes('密码') || key.toLowerCase().includes('password')}
              <div class="flex gap-2">
                <input
                  type="password"
                  bind:value={fields[key]}
                  class="flex-1 px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
                />
                <button
                  type="button"
                  onclick={handleQuickGenerate}
                  class="px-3 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 text-sm rounded-lg transition whitespace-nowrap"
                  title="快速生成16位安全密码"
                >
                  生成
                </button>
              </div>
            {:else}
              <input
                type="text"
                bind:value={fields[key]}
                class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
              />
            {/if}
          </div>
        {/each}
      {/if}

      {#if error}
        <p class="text-red-500 text-xs text-center">{error}</p>
      {/if}
    </form>

    <div class="px-4 py-3 border-t border-gray-200 flex justify-between">
      <button
        onclick={handleDelete}
        disabled={loading}
        class="px-4 py-1.5 text-sm text-red-500 hover:bg-red-50 rounded-lg transition disabled:opacity-50"
      >
        删除
      </button>
      <div class="flex gap-2">
        <button
          onclick={() => onNavigate('main')}
          class="px-4 py-1.5 text-sm text-gray-600 hover:bg-gray-100 rounded-lg transition"
        >
          取消
        </button>
        <button
          onclick={handleSubmit}
          disabled={loading}
          class="px-4 py-1.5 text-sm bg-gray-900 hover:bg-gray-800 text-white rounded-lg transition disabled:opacity-50"
        >
          {loading ? '保存中...' : '保存'}
        </button>
      </div>
    </div>
  </div>
</div>
