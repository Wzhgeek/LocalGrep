import type { SearchHit } from '../services/tauriApi';

type ResultListProps = {
  query: string;
  hits: SearchHit[];
};

export function ResultList({ query, hits }: ResultListProps) {
  return (
    <section className="result-list">
      <h2>搜索结果</h2>
      <p>当前查询: {query || '（空）'}</p>
      <ul>
        {hits.map((hit) => (
          <li key={hit.file_id}>
            <strong>{hit.filename}</strong>
            <div>{hit.path}</div>
          </li>
        ))}
      </ul>
    </section>
  );
}
