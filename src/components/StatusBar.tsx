type StatusBarProps = {
  total: number;
  tookMs: number;
  searchError: string | null;
};

export function StatusBar({ total, tookMs, searchError }: StatusBarProps) {
  return (
    <footer className="chrome-bottom">
      <span>
        命中数 <strong>{total}</strong>
      </span>
      <span>
        查询耗时 <strong>{tookMs} ms</strong>
      </span>
      {searchError ? <span className="chrome-bottom-error">错误: {searchError}</span> : null}
    </footer>
  );
}
