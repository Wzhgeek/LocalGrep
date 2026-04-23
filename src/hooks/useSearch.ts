import { useEffect, useState } from 'react';
import { searchFiles, type SearchHit } from '../services/tauriApi';

export function useSearch(query: string) {
  const [hits, setHits] = useState<SearchHit[]>([]);
  const [total, setTotal] = useState(0);
  const [tookMs, setTookMs] = useState(0);

  useEffect(() => {
    let cancelled = false;
    const handler = setTimeout(async () => {
      const result = await searchFiles({ query, page: 1, pageSize: 50 });
      if (cancelled) {
        return;
      }
      setHits(result.hits);
      setTotal(result.total);
      setTookMs(result.took_ms);
    }, 200);

    return () => {
      cancelled = true;
      clearTimeout(handler);
    };
  }, [query]);

  return { hits, total, tookMs };
}
