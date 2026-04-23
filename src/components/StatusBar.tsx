type StatusBarProps = {
  total: number;
  tookMs: number;
};

export function StatusBar({ total, tookMs }: StatusBarProps) {
  return (
    <footer className="status-bar">
      <span>搜索结果: {total}</span>
      <span>任务队列: 处理中</span>
      <span>查询耗时: {tookMs}ms</span>
    </footer>
  );
}
