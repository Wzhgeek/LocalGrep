import { invoke } from '@tauri-apps/api/core';

export type SearchRequest = {
  query: string;
  page: number;
  pageSize: number;
};

export type SearchHit = {
  file_id: number;
  path: string;
  filename: string;
  snippet: string;
};

export type SearchResponse = {
  hits: SearchHit[];
  total: number;
  took_ms: number;
};

/** 与 Rust `serde` 默认字段名一致（snake_case） */
export type Root = {
  id: number;
  path: string;
  enabled: boolean;
};

export type IndexStatus = {
  pending_tasks: number;
  indexed_files: number;
};

export async function searchFiles(input: SearchRequest): Promise<SearchResponse> {
  return await invoke<SearchResponse>('search', { input });
}

export async function listRoots(): Promise<Root[]> {
  return await invoke<Root[]>('list_roots');
}

export async function addRoot(path: string): Promise<void> {
  await invoke<void>('add_root', { path });
}

export async function startFullScan(): Promise<void> {
  await invoke<void>('start_full_scan');
}

export async function getIndexStatus(): Promise<IndexStatus> {
  return await invoke<IndexStatus>('get_index_status');
}
