import type { SearchHit } from '../services/tauriApi';

type PreviewPanelProps = {
  hit: SearchHit | null;
};

export function PreviewPanel({ hit }: PreviewPanelProps) {
  return (
    <aside className="pane-inspector">
      <div className="inspector-header">
        <h2>片段预览</h2>
      </div>
      <div className="inspector-body">
        {!hit ? (
          <p className="inspector-empty">在命中列表中选择一行以查看路径与摘要。</p>
        ) : (
          <>
            <p className="inspector-file">{hit.filename}</p>
            <p className="inspector-path">{hit.path}</p>
            <p className="inspector-label">摘要</p>
            <p className="inspector-snippet">{hit.snippet || '（无摘要）'}</p>
          </>
        )}
      </div>
    </aside>
  );
}
