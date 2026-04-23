type StatusBarProps = {
  total: number;
  tookMs: number;
  searchError: string | null;
};

export function StatusBar({ total, tookMs, searchError }: StatusBarProps) {
  return (
    <footer className="status-bar">
      <span>搜索结果: {total}</span>
      <span>查询耗时: {tookMs}ms</span>
      {searchError ? <span className="status-bar-error">搜索错误: {searchError}</span> : null}
    </footer>
  );
}
