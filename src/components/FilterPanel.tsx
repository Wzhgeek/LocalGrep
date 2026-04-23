export function FilterPanel() {
  return (
    <aside className="filter-panel">
      <h3>过滤器</h3>
      <label>
        <input type="checkbox" defaultChecked />
        仅文件名
      </label>
      <label>
        <input type="checkbox" />
        排除隐藏文件
      </label>
      <label>
        <input type="checkbox" defaultChecked />
        抽取成功优先
      </label>
    </aside>
  );
}
