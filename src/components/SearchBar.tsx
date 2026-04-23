type SearchBarProps = {
  query: string;
  onChange: (value: string) => void;
};

export function SearchBar({ query, onChange }: SearchBarProps) {
  return (
    <header className="chrome-top">
      <div className="chrome-brand">
        <span className="chrome-brand-name">LocalGrep</span>
        <span className="chrome-brand-tag">本地检索</span>
      </div>
      <div className="chrome-search-wrap">
        <input
          className="chrome-search"
          value={query}
          onChange={(event) => onChange(event.target.value)}
          placeholder="查询关键词、路径或文件名…"
          spellCheck={false}
          aria-label="搜索"
        />
      </div>
    </header>
  );
}
