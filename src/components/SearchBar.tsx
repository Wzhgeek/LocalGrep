type SearchBarProps = {
  query: string;
  onChange: (value: string) => void;
};

export function SearchBar({ query, onChange }: SearchBarProps) {
  return (
    <header className="search-bar">
      <input
        value={query}
        onChange={(event) => onChange(event.target.value)}
        placeholder="搜索文件名、路径、内容..."
      />
    </header>
  );
}
