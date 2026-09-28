export interface Record {
  id: string;
  category: string;
  title: string;
  fields: Record<string, string>;
  created_at: string;
  updated_at: string;
}

export type Category = 'credential' | 'database' | 'server' | 'website';

export const CATEGORY_LABELS: Record<Category, string> = {
  credential: '用户名/密码',
  database: '数据库',
  server: '服务器',
  website: '网站',
};

export const CATEGORY_FIELDS: Record<Category, string[]> = {
  credential: ['用户名', '密码'],
  database: ['URL', 'Host', 'Port', '数据库', 'Schema', '用户名', '密码'],
  server: ['IP', '用户名', '密码'],
  website: ['URL', '用户名', '密码'],
};

export const CATEGORY_ORDER: Category[] = ['credential', 'database', 'server', 'website'];
