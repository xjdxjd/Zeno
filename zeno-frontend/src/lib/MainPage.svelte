<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import RecordCard from './RecordCard.svelte';
  import type { Record } from './types';
  import { CATEGORY_LABELS, CATEGORY_ORDER, type Category } from './types';
  import { copyToClipboard } from './clipboard';

  let { onNavigate }: { onNavigate: (page: string, record?: Record) => void } = $props();

  let records = $state<Record[]>([]);
  let searchQuery = $state('');
  let loading = $state(true);

  // Tip notification
  let tipVisible = $state(false);
  let tipText = $state('');
  let tipTimer = $state<ReturnType<typeof setTimeout> | null>(null);

  function showTip(text: string) {
    if (tipTimer) {
      clearTimeout(tipTimer);
    }
    tipText = text;
    tipVisible = true;
    tipTimer = setTimeout(() => {
      tipVisible = false;
      tipTimer = null;
    }, 1500);
  }

  async function loadRecords() {
    try {
      records = await invoke<Record[]>('get_records');
    } catch (e) {
      console.error('Failed to load records:', e);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    loadRecords();
  });

  async function handleSearch() {
    if (!searchQuery.trim()) {
      await loadRecords();
      return;
    }
    try {
      records = await invoke<Record[]>('search_records', { query: searchQuery });
    } catch (e) {
      console.error('Search failed:', e);
    }
  }

  $effect(() => {
    if (searchQuery !== undefined) {
      handleSearch();
    }
  });

  function getRecordsByCategory(category: Category): Record[] {
    return records.filter(r => r.category === category);
  }

  async function handleDelete(id: string) {
    if (!confirm('确定要删除这条记录吗？')) return;
    try {
      await invoke('delete_record', { id });
      await loadRecords();
    } catch (e) {
      console.error('Delete failed:', e);
    }
  }

  async function handleCopy(text: string, isPassword: boolean = false) {
    await copyToClipboard(text, isPassword);
    showTip('已复制');
  }

  let totalCount = $derived(records.length);
</script>

<div class="max-w-3xl mx-auto p-4 relative">
  <!-- 搜索栏 -->
  <div class="sticky top-0 z-10 -mx-4 px-4 py-3 bg-gray-50/95 backdrop-blur-sm">
    <div class="relative">
      <svg class="w-4 h-4 text-gray-400 absolute left-3 top-1/2 -translate-y-1/2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
      </svg>
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="搜索..."
        class="w-full pl-9 pr-4 py-2 bg-white border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
      />
    </div>
  </div>

  <!-- 操作栏 -->
  <div class="flex gap-2 mb-6 mt-2">
    <button
      onclick={() => onNavigate('create')}
      class="px-3 py-1.5 bg-gray-900 hover:bg-gray-800 text-white rounded-lg text-sm font-medium transition"
    >
      + 新建
    </button>
    <button
      onclick={() => onNavigate('generator')}
      class="px-3 py-1.5 bg-white border border-gray-200 hover:bg-gray-50 rounded-lg text-sm font-medium transition"
    >
      生成密码
    </button>
  </div>

  {#if loading}
    <div class="text-center text-gray-400 py-8">加载中...</div>
  {:else}
    {#each CATEGORY_ORDER as category}
      {@const categoryRecords = getRecordsByCategory(category)}
      {#if categoryRecords.length > 0}
        <div class="mb-6">
          <h3 class="text-xs font-medium text-gray-400 uppercase tracking-wider mb-2 px-1">
            {CATEGORY_LABELS[category]}
          </h3>
          <div class="space-y-2">
            {#each categoryRecords as record (record.id)}
              <RecordCard
                {record}
                onEdit={() => onNavigate('edit', record)}
                onDelete={() => handleDelete(record.id)}
                onCopy={handleCopy}
              />
            {/each}
          </div>
        </div>
      {/if}
    {/each}

    {#if records.length === 0}
      <div class="text-center text-gray-400 py-8">
        <p class="mb-2">暂无记录</p>
        <p class="text-sm">点击「+ 新建」添加第一条记录</p>
      </div>
    {:else}
      <div class="text-center text-gray-400 text-xs py-4">
        共 {totalCount} 条记录
      </div>
    {/if}
  {/if}

  <!-- Tip 提示 -->
  {#if tipVisible}
    <div class="fixed bottom-4 left-1/2 -translate-x-1/2 z-50">
      <div class="bg-gray-900 text-white text-sm px-4 py-2 rounded-lg shadow-lg">
        {tipText}
      </div>
    </div>
  {/if}
</div>
