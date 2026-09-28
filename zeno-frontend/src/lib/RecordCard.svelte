<script lang="ts">
  import type { Record } from './types';
  import { CATEGORY_FIELDS, type Category } from './types';

  let { record, onEdit, onDelete, onCopy }: {
    record: Record;
    onEdit: () => void;
    onDelete: () => void;
    onCopy: (text: string, isPassword?: boolean) => void;
  } = $props();

  let passwordVisible = $state(false);

  function isPasswordField(key: string): boolean {
    return key.toLowerCase().includes('password') || key === '密码';
  }

  function isDatabaseCategory(): boolean {
    return record.category === 'database';
  }

  function getSortedEntries(): [string, string][] {
    const category = record.category as Category;
    const fieldOrder = CATEGORY_FIELDS[category] || [];
    const entries = Object.entries(record.fields);
    return entries.sort((a, b) => {
      const ai = fieldOrder.indexOf(a[0]);
      const bi = fieldOrder.indexOf(b[0]);
      const aIdx = ai === -1 ? 999 : ai;
      const bIdx = bi === -1 ? 999 : bi;
      return aIdx - bIdx;
    });
  }

  function getValue(key: string): string {
    return record.fields[key] || '';
  }
</script>

<div class="bg-white rounded-lg border border-gray-200 p-3 hover:shadow-sm transition">
  <div class="flex items-center justify-between mb-2">
    <button onclick={onEdit} class="font-bold text-base text-left hover:opacity-80 transition rainbow-text">
      {record.title}
    </button>
    <button
      onclick={onDelete}
      class="p-1 text-gray-400 hover:text-red-500 hover:bg-red-50 rounded transition"
      title="删除"
    >
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
      </svg>
    </button>
  </div>

  {#if isDatabaseCategory()}
    <!-- URL -->
    <div class="flex items-center justify-between py-1.5">
      <div class="flex-1 min-w-0">
        <span class="text-xs text-gray-400">URL</span>
        <div class="text-sm font-mono truncate text-blue-600">{getValue('URL')}</div>
      </div>
      <div class="flex items-center gap-0.5 ml-2 shrink-0">
        <button
          onclick={() => onCopy(getValue('URL'), false)}
          class="p-1.5 text-gray-400 hover:text-gray-900 hover:bg-gray-100 rounded transition"
          title="复制"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"/>
          </svg>
        </button>
      </div>
    </div>

    <!-- Host + Port -->
    <div class="py-1.5 border-t border-gray-100">
      <div class="flex items-end gap-4">
        <div class="flex-1 min-w-0">
          <span class="text-xs text-gray-400">Host</span>
          <div class="text-sm font-mono truncate">{getValue('Host')}</div>
        </div>
        <button
          onclick={() => onCopy(getValue('Host'), false)}
          class="p-1.5 text-gray-400 hover:text-gray-900 hover:bg-gray-100 rounded transition shrink-0 mb-0.5"
          title="复制 Host"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"/>
          </svg>
        </button>
        <div class="w-24 shrink-0">
          <span class="text-xs text-gray-400">Port</span>
          <div class="text-sm font-mono truncate">{getValue('Port')}</div>
        </div>
        <button
          onclick={() => onCopy(getValue('Port'), false)}
          class="p-1.5 text-gray-400 hover:text-gray-900 hover:bg-gray-100 rounded transition shrink-0 mb-0.5"
          title="复制 Port"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"/>
          </svg>
        </button>
      </div>
    </div>

    <!-- 数据库 + Schema -->
    <div class="py-1.5 border-t border-gray-100">
      <div class="flex items-end gap-4">
        <div class="flex-1 min-w-0">
          <span class="text-xs text-gray-400">数据库</span>
          <div class="text-sm font-mono truncate">{getValue('数据库')}</div>
        </div>
        <button
          onclick={() => onCopy(getValue('数据库'), false)}
          class="p-1.5 text-gray-400 hover:text-gray-900 hover:bg-gray-100 rounded transition shrink-0 mb-0.5"
          title="复制 数据库"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"/>
          </svg>
        </button>
        <div class="flex-1 min-w-0">
          <span class="text-xs text-gray-400">Schema</span>
          <div class="text-sm font-mono truncate">{getValue('Schema')}</div>
        </div>
        <button
          onclick={() => onCopy(getValue('Schema'), false)}
          class="p-1.5 text-gray-400 hover:text-gray-900 hover:bg-gray-100 rounded transition shrink-0 mb-0.5"
          title="复制 Schema"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"/>
          </svg>
        </button>
      </div>
    </div>

    <!-- 用户名 -->
    <div class="flex items-center justify-between py-1.5 border-t border-gray-100">
      <div class="flex-1 min-w-0">
        <span class="text-xs text-gray-400">用户名</span>
        <div class="text-sm font-mono truncate">{getValue('用户名')}</div>
      </div>
      <div class="flex items-center gap-0.5 ml-2 shrink-0">
        <button
          onclick={() => onCopy(getValue('用户名'), false)}
          class="p-1.5 text-gray-400 hover:text-gray-900 hover:bg-gray-100 rounded transition"
          title="复制"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"/>
          </svg>
        </button>
      </div>
    </div>

    <!-- 密码 -->
    <div class="flex items-center justify-between py-1.5 border-t border-gray-100">
      <div class="flex-1 min-w-0">
        <span class="text-xs text-gray-400">密码</span>
        <div class="text-sm font-mono truncate">
          {#if passwordVisible}
            {getValue('密码')}
          {:else}
            ••••••••••••
          {/if}
        </div>
      </div>
      <div class="flex items-center gap-0.5 ml-2 shrink-0">
        <button
          onclick={() => passwordVisible = !passwordVisible}
          class="p-1.5 text-gray-400 hover:text-gray-900 hover:bg-gray-100 rounded transition"
          title={passwordVisible ? '隐藏' : '显示'}
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"/>
          </svg>
        </button>
        <button
          onclick={() => onCopy(getValue('密码'), true)}
          class="p-1.5 text-gray-400 hover:text-gray-900 hover:bg-gray-100 rounded transition"
          title="复制"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"/>
          </svg>
        </button>
      </div>
    </div>
  {:else}
    {#each getSortedEntries() as [key, value]}
      <div class="flex items-center justify-between py-1.5 {key === Object.keys(record.fields)[0] ? '' : 'border-t border-gray-100'}">
        <div class="flex-1 min-w-0">
          <span class="text-xs text-gray-400">{key}</span>
          {#if isPasswordField(key)}
            <div class="text-sm font-mono truncate">
              {#if passwordVisible}
                {value}
              {:else}
                ••••••••••••
              {/if}
            </div>
          {:else}
            <div class="text-sm font-mono truncate {key === 'URL' || key === 'url' ? 'text-blue-600' : ''}">
              {value}
            </div>
          {/if}
        </div>
        <div class="flex items-center gap-0.5 ml-2 shrink-0">
          {#if isPasswordField(key)}
            <button
              onclick={() => passwordVisible = !passwordVisible}
              class="p-1.5 text-gray-400 hover:text-gray-900 hover:bg-gray-100 rounded transition"
              title={passwordVisible ? '隐藏' : '显示'}
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"/>
              </svg>
            </button>
          {/if}
          <button
            onclick={() => onCopy(value, isPasswordField(key))}
            class="p-1.5 text-gray-400 hover:text-gray-900 hover:bg-gray-100 rounded transition"
            title="复制"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"/>
            </svg>
          </button>
        </div>
      </div>
    {/each}
  {/if}
</div>

<style>
  .rainbow-text {
    background: linear-gradient(
      90deg,
      #ff6b6b,
      #ffa94d,
      #ffd43b,
      #69db7c,
      #4dabf7,
      #9775fa
    );
    -webkit-background-clip: text;
    background-clip: text;
    -webkit-text-fill-color: transparent;
  }
</style>
