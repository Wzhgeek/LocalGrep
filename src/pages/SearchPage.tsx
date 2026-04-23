import { useState } from 'react';
import { FilterPanel } from '../components/FilterPanel';
import { SearchBar } from '../components/SearchBar';
import { ResultList } from '../components/ResultList';
import { PreviewPanel } from '../components/PreviewPanel';
import { SettingsPanel } from '../components/SettingsPanel';
import { StatusBar } from '../components/StatusBar';
import { useSearch } from '../hooks/useSearch';

export function SearchPage() {
  const [query, setQuery] = useState('');
  const { hits, total, tookMs } = useSearch(query);

  return (
    <main className="layout">
      <SearchBar query={query} onChange={setQuery} />
      <section className="content">
        <FilterPanel />
        <ResultList query={query} hits={hits} />
        <PreviewPanel />
        <SettingsPanel />
      </section>
      <StatusBar total={total} tookMs={tookMs} />
    </main>
  );
}
