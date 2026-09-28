<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import UnlockPage from './lib/UnlockPage.svelte';
  import MainPage from './lib/MainPage.svelte';
  import CreatePage from './lib/CreatePage.svelte';
  import GeneratorPage from './lib/GeneratorPage.svelte';
  import EditPage from './lib/EditPage.svelte';
  import type { Record } from './lib/types';

  let status = $state<'loading' | 'setup' | 'ready'>('loading');
  let currentPage = $state<'main' | 'create' | 'edit' | 'generator'>('main');
  let editingRecord = $state<Record | null>(null);

  async function init() {
    try {
      const exists = await invoke<boolean>('check_vault_exists');
      if (exists) {
        await invoke('auto_unlock');
        status = 'ready';
      } else {
        status = 'setup';
      }
    } catch {
      status = 'setup';
    }
  }

  init();

  function handleSetup() {
    status = 'ready';
    currentPage = 'main';
  }

  function navigateTo(page: 'main' | 'create' | 'edit' | 'generator', record?: Record) {
    currentPage = page;
    if (record) {
      editingRecord = record;
    }
  }
</script>

<main class="min-h-screen bg-[#f9fafb]">
  {#if status === 'loading'}
    <div class="flex items-center justify-center min-h-screen">
      <div class="text-gray-400 text-sm">加载中...</div>
    </div>
  {:else if status === 'setup'}
    <UnlockPage onSetup={handleSetup} />
  {:else}
    {#if currentPage === 'main'}
      <MainPage onNavigate={navigateTo} />
    {:else if currentPage === 'create'}
      <CreatePage onNavigate={navigateTo} />
    {:else if currentPage === 'edit' && editingRecord}
      <EditPage record={editingRecord} onNavigate={navigateTo} />
    {:else if currentPage === 'generator'}
      <GeneratorPage onNavigate={navigateTo} />
    {/if}
  {/if}
</main>
