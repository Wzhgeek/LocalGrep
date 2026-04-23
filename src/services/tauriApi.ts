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

export async function searchFiles(input: SearchRequest): Promise<SearchResponse> {
  try {
    return await invoke<SearchResponse>('search', { input });
  } catch {
    return { hits: [], total: 0, took_ms: 0 };
  }
}
