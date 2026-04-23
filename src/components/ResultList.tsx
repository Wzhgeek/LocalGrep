import type { SearchHit } from '../services/tauriApi';

type ResultListProps = {
  query: string;
  hits: SearchHit[];
  selectedId: number | null;
  onSelect: (fileId: number) => void;
};

export function ResultList({ query, hits, selectedId, onSelect }: ResultListProps) {
  return (
    <section className="pane-results">
      <div className="pane-header">
        <h2>命中列表</h2>
        <span className="pane-header-meta">
          {hits.length} 条
          {query.trim() ? ` · 「${query.trim()}」` : ''}
        </span>
      </div>
      <div className="hit-scroll">
        {hits.length === 0 ? (
          <p className="hit-empty">
            无命中。若侧栏「数据源」尚无根目录，请先添加路径并执行扫描；完成后等待索引写入再试。
          </p>
        ) : (
          <ul className="hit-list">
            {hits.map((hit) => (
              <li key={hit.file_id}>
                <button
                  type="button"
                  className={`hit-row ${selectedId === hit.file_id ? 'is-active' : ''}`}
                  aria-selected={selectedId === hit.file_id}
                  onClick={() => onSelect(hit.file_id)}
                >
                  <span className="hit-name">{hit.filename}</span>
                  <span className="hit-path">{hit.path}</span>
                </button>
              </li>
            ))}
          </ul>
        )}
      </div>
    </section>
  );
}
