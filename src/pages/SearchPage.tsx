import { useEffect, useMemo, useState } from 'react';
import { FilterPanel } from '../components/FilterPanel';
import { SearchBar } from '../components/SearchBar';
import { ResultList } from '../components/ResultList';
import { PreviewPanel } from '../components/PreviewPanel';
import { SettingsPanel } from '../components/SettingsPanel';
import { StatusBar } from '../components/StatusBar';
import { useSearch } from '../hooks/useSearch';

export function SearchPage() {
  const [query, setQuery] = useState('');
  const { hits, total, tookMs, error } = useSearch(query);
  const [selectedId, setSelectedId] = useState<number | null>(null);

  useEffect(() => {
    setSelectedId(null);
  }, [query]);

  useEffect(() => {
    if (hits.length === 0) {
      setSelectedId(null);
      return;
    }
    setSelectedId((prev) => {
      if (prev !== null && hits.some((h) => h.file_id === prev)) {
        return prev;
      }
      return hits[0]!.file_id;
    });
  }, [hits]);

  const selectedHit = useMemo(
    () => hits.find((h) => h.file_id === selectedId) ?? hits[0] ?? null,
    [hits, selectedId],
  );

  return (
    <main className="shell">
      <SearchBar query={query} onChange={setQuery} />
      <div className="shell-workspace">
        <aside className="rail">
          <div className="rail-scroll">
            <FilterPanel />
            <SettingsPanel />
          </div>
        </aside>
        <ResultList
          query={query}
          hits={hits}
          selectedId={selectedId}
          onSelect={(id) => setSelectedId(id)}
        />
        <PreviewPanel hit={selectedHit} />
      </div>
      <StatusBar total={total} tookMs={tookMs} searchError={error} />
    </main>
  );
}
