<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { CATEGORY_LABELS, CATEGORY_FIELDS, CATEGORY_ORDER, type Category } from './types';

  let { onNavigate }: { onNavigate: (page: string) => void } = $props();

  let category = $state<Category | ''>('');
  let title = $state('');
  let fields = $state<Record<string, string>>({});
  let error = $state('');
  let loading = $state(false);

  function isDatabaseCategory(): boolean {
    return category === 'database';
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

  function handleCategoryChange() {
    if (category) {
      const newFields: Record<string, string> = {};
      CATEGORY_FIELDS[category as Category].forEach(f => {
        newFields[f] = '';
      });
      fields = newFields;
    } else {
      fields = {};
    }
  }

  async function handleSubmit() {
    error = '';
    
    if (!category) {
      error = '请选择类别';
      return;
    }
    if (!title.trim()) {
      error = '请输入标题';
      return;
    }

    loading = true;
    try {
      await invoke('add_record', {
        category,
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
</script>

<div class="max-w-lg mx-auto p-4">
  <div class="bg-white rounded-lg border border-gray-200">
    <div class="px-4 py-3 border-b border-gray-200 flex items-center justify-between">
      <h2 class="font-medium text-sm">新建记录</h2>
      <button
        onclick={() => onNavigate('main')}
        aria-label="关闭"
        class="text-gray-400 hover:text-gray-600 transition"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
        </svg>
      </button>
    </div>

    <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="p-4 space-y-4">
      <div>
        <label for="cp-category" class="block text-xs font-medium text-gray-600 mb-1">类别</label>
        <select
          id="cp-category"
          bind:value={category}
          onchange={handleCategoryChange}
          class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
        >
          <option value="">请选择</option>
          {#each CATEGORY_ORDER as cat}
            <option value={cat}>{CATEGORY_LABELS[cat]}</option>
          {/each}
        </select>
      </div>

      <div>
        <label for="cp-title" class="block text-xs font-medium text-gray-600 mb-1">标题 <span class="text-red-500">*</span></label>
        <input
          id="cp-title"
          type="text"
          bind:value={title}
          placeholder="例如：GitHub 账号"
          class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
        />
      </div>

      {#if category}
        {#if isDatabaseCategory()}
          <!-- URL -->
          <div>
            <label for="cp-url" class="block text-xs font-medium text-gray-600 mb-1">URL</label>
            <input
              id="cp-url"
              type="text"
              bind:value={fields['URL']}
              placeholder="请输入URL"
              class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
            />
          </div>

          <!-- Host + Port -->
          <div class="flex gap-4">
            <div class="flex-1">
              <label for="cp-host" class="block text-xs font-medium text-gray-600 mb-1">Host</label>
              <input
                id="cp-host"
                type="text"
                bind:value={fields['Host']}
                placeholder="请输入Host"
                class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
              />
            </div>
            <div class="w-32">
              <label for="cp-port" class="block text-xs font-medium text-gray-600 mb-1">Port</label>
              <input
                id="cp-port"
                type="text"
                bind:value={fields['Port']}
                placeholder="端口"
                class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
              />
            </div>
          </div>

          <!-- 数据库 + Schema -->
          <div class="flex gap-4">
            <div class="flex-1">
              <label for="cp-db" class="block text-xs font-medium text-gray-600 mb-1">数据库</label>
              <input
                id="cp-db"
                type="text"
                bind:value={fields['数据库']}
                placeholder="请输入数据库名"
                class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
              />
            </div>
            <div class="flex-1">
              <label for="cp-schema" class="block text-xs font-medium text-gray-600 mb-1">Schema</label>
              <input
                id="cp-schema"
                type="text"
                bind:value={fields['Schema']}
                placeholder="请输入Schema"
                class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
              />
            </div>
          </div>

          <!-- 用户名 -->
          <div>
            <label for="cp-username" class="block text-xs font-medium text-gray-600 mb-1">用户名</label>
            <input
              id="cp-username"
              type="text"
              bind:value={fields['用户名']}
              placeholder="请输入用户名"
              class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
            />
          </div>

          <!-- 密码 -->
          <div>
            <label for="cp-password" class="block text-xs font-medium text-gray-600 mb-1">密码</label>
            <div class="flex gap-2">
              <input
                id="cp-password"
                type="password"
                bind:value={fields['密码']}
                placeholder="请输入密码"
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
          {#each Object.entries(fields) as [key, _]}
            <div>
              <label for="cp-f-{key}" class="block text-xs font-medium text-gray-600 mb-1">{key}</label>
              {#if key.includes('密码') || key.toLowerCase().includes('password')}
                <div class="flex gap-2">
                  <input
                    id="cp-f-{key}"
                    type="password"
                    bind:value={fields[key]}
                    placeholder="请输入{key}"
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
                  id="cp-f-{key}"
                  type="text"
                  bind:value={fields[key]}
                  placeholder="请输入{key}"
                  class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
                />
              {/if}
            </div>
          {/each}
        {/if}
      {/if}

      {#if error}
        <p class="text-red-500 text-xs text-center">{error}</p>
      {/if}
    </form>

    <div class="px-4 py-3 border-t border-gray-200 flex justify-end gap-2">
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
