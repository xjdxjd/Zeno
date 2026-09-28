<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
  import { CATEGORY_LABELS, type Category, type Record } from './types';
  import { copyToClipboard } from './clipboard';

  const appWindow = getCurrentWebviewWindow();
  const MAX_ITEMS = 9;

  let records = $state<Record[]>([]);
  let query = $state('');
  let selectedId = $state<string | null>(null);
  let highlight = $state(0);
  let inputEl: HTMLInputElement | undefined = $state();

  async function loadRecords() {
    try {
      records = await invoke<Record[]>('get_records');
    } catch {
      // vault 未解锁时保持空列表
    }
  }

  function matches(r: Record): boolean {
    const q = query.trim().toLowerCase();
    if (!q) return true;
    if (r.title.toLowerCase().includes(q)) return true;
    return Object.entries(r.fields).some(([k, v]) =>
      k !== '密码' && k.toLowerCase() !== 'password' && v.toLowerCase().includes(q)
    );
  }

  let filtered = $derived(records.filter(matches).slice(0, MAX_ITEMS));
  let selected = $derived(selectedId ? records.find(r => r.id === selectedId) ?? null : null);
  let fields = $derived(
    selected ? Object.entries(selected.fields).filter(([, v]) => v !== '') : []
  );

  // 当前高亮行（防越界）
  let hl = $derived.by(() => {
    const max = (selected ? fields.length : filtered.length) - 1;
    return max < 0 ? 0 : Math.min(highlight, max);
  });

  function reset() {
    query = '';
    selectedId = null;
    highlight = 0;
  }

  async function hide() {
    reset();
    try {
      await appWindow.hide();
    } catch {
      // 窗口 API 未获授权时的兜底：走自定义命令隐藏
      try {
        await invoke('hide_quick');
      } catch {
        // 忽略
      }
    }
  }

  function pickRecord(id: string) {
    selectedId = id;
    highlight = 0;
  }

  async function pickField(index: number) {
    const [key, value] = fields[index];
    if (value === undefined) return;
    await copyToClipboard(value, key === '密码');
    await hide();
  }

  function handleKeydown(e: KeyboardEvent) {
    // 输入法组词状态下的按键（含选字数字）不拦截
    if (e.isComposing) return;

    if (e.key === 'Escape') {
      e.preventDefault();
      if (selectedId) {
        // 返回列表视图
        selectedId = null;
        highlight = 0;
      } else {
        hide();
      }
      return;
    }

    // Ctrl+数字：直选记录/字段。裸数字键永远作为搜索输入（如 IP 地址）
    if (e.ctrlKey) {
      const n = Number(e.key);
      if (n >= 1 && n <= 9) {
        e.preventDefault();
        if (selected) {
          pickField(n - 1);
        } else if (filtered[n - 1]) {
          pickRecord(filtered[n - 1].id);
        }
      }
      return; // 其他 Ctrl 组合不拦截
    }

    if (e.key === 'ArrowDown') {
      e.preventDefault();
      const max = (selected ? fields.length : filtered.length) - 1;
      if (max >= 0) highlight = Math.min(highlight + 1, max);
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      highlight = Math.max(highlight - 1, 0);
    } else if (e.key === 'Enter') {
      e.preventDefault();
      if (selected) {
        pickField(hl);
      } else if (filtered[hl]) {
        pickRecord(filtered[hl].id);
      }
    }
    // 其余按键（包括数字）正常输入到搜索框
  }

  onMount(() => {
    loadRecords();

    // 每次弹窗唤起（重新获得焦点）时重置状态并刷新数据
    window.addEventListener('focus', () => {
      reset();
      loadRecords();
      setTimeout(() => inputEl?.focus(), 30);
    });
  });
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="h-screen flex flex-col bg-white">
  <div class="px-3 pt-3 pb-2 border-b border-gray-100">
    <input
      bind:this={inputEl}
      bind:value={query}
      oninput={() => {
        selectedId = null;
        highlight = 0;
      }}
      placeholder="搜索记录（支持 IP 等数字）..."
      class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent outline-none text-sm"
    />
    <p class="text-[10px] text-gray-400 mt-1.5 px-1">
      {#if selected}
        Ctrl + 数字 复制字段 · ↑↓ 换行 · Enter 复制 · Esc 返回
      {:else}
        Ctrl + 数字 选择记录 · ↑↓ 换行 · Enter 确认 · Esc 隐藏
      {/if}
    </p>
  </div>

  <div class="flex-1 overflow-y-auto p-2">
    {#if !selected}
      {#each filtered as record, i (record.id)}
        <button
          onclick={() => { highlight = i; pickRecord(record.id); }}
          class="w-full flex items-center gap-2.5 px-2 py-2 rounded-lg text-left transition {i === hl ? 'bg-gray-100' : 'hover:bg-gray-50'}"
        >
          <kbd class="px-1.5 h-5 flex items-center justify-center rounded border border-gray-200 bg-white text-[10px] text-gray-500 shrink-0 whitespace-nowrap">
            Ctrl + {i + 1}
          </kbd>
          <span class="text-sm font-medium truncate">{record.title}</span>
          <span class="text-xs text-gray-400 ml-auto shrink-0">
            {CATEGORY_LABELS[record.category as Category] ?? record.category}
          </span>
        </button>
      {:else}
        <div class="text-center text-gray-400 text-sm py-8">
          {records.length === 0 ? '暂无记录' : '无匹配记录'}
        </div>
      {/each}
    {:else}
      <div class="flex items-center gap-2.5 px-2 py-1.5">
        <span class="text-sm font-medium truncate">{selected.title}</span>
        <span class="text-xs text-gray-400 ml-auto shrink-0">
          {CATEGORY_LABELS[selected.category as Category] ?? selected.category}
        </span>
      </div>
      {#each fields as [key, value], i}
        <button
          onclick={() => { highlight = i; pickField(i); }}
          class="w-full flex items-center gap-2.5 px-2 py-2 rounded-lg text-left transition {i === hl ? 'bg-gray-100' : 'hover:bg-gray-50'}"
        >
          <kbd class="px-1.5 h-5 flex items-center justify-center rounded border border-gray-200 bg-white text-[10px] text-gray-500 shrink-0 whitespace-nowrap">
            Ctrl + {i + 1}
          </kbd>
          <span class="text-xs text-gray-400 w-14 shrink-0">{key}</span>
          <span class="text-sm font-mono truncate">
            {key === '密码' ? '••••••••' : value}
          </span>
        </button>
      {:else}
        <div class="text-center text-gray-400 text-sm py-8">该记录没有可复制的字段</div>
      {/each}
    {/if}
  </div>
</div>
