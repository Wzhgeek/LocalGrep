export function FilterPanel() {
  return (
    <div>
      <h3 className="rail-section-title">过滤</h3>
      <label className="rail-option">
        <input type="checkbox" defaultChecked />
        <span>仅匹配文件名</span>
      </label>
      <label className="rail-option">
        <input type="checkbox" />
        <span>排除隐藏项</span>
      </label>
      <label className="rail-option">
        <input type="checkbox" defaultChecked />
        <span>仅已抽取内容</span>
      </label>
    </div>
  );
}
