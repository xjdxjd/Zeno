// 复制文本；密码字段复制后 30 秒自动清空（仅当剪贴板未被其他内容覆盖时）
let clipboardTimer: ReturnType<typeof setTimeout> | null = null;
let copiedPassword = '';

function fallbackCopy(text: string) {
  const textarea = document.createElement('textarea');
  textarea.value = text;
  document.body.appendChild(textarea);
  textarea.select();
  document.execCommand('copy');
  document.body.removeChild(textarea);
}

export async function copyToClipboard(text: string, isPassword: boolean = false): Promise<void> {
  try {
    await navigator.clipboard.writeText(text);
  } catch {
    fallbackCopy(text);
  }

  if (isPassword) {
    if (clipboardTimer) {
      clearTimeout(clipboardTimer);
    }
    copiedPassword = text;
    clipboardTimer = setTimeout(async () => {
      try {
        const current = await navigator.clipboard.readText();
        if (current === copiedPassword) {
          await navigator.clipboard.writeText('');
        }
      } catch {
        // 剪贴板读取被拒绝时跳过清空
      }
      clipboardTimer = null;
      copiedPassword = '';
    }, 30000);
  }
}
