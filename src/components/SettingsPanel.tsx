import { useCallback, useEffect, useState } from 'react';
import {
  addRoot,
  getIndexStatus,
  listRoots,
  startFullScan,
  type IndexStatus,
  type Root,
} from '../services/tauriApi';

export function SettingsPanel() {
  const [path, setPath] = useState('');
  const [roots, setRoots] = useState<Root[]>([]);
  const [status, setStatus] = useState<IndexStatus | null>(null);
  const [message, setMessage] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const refresh = useCallback(async () => {
    try {
      const [r, s] = await Promise.all([listRoots(), getIndexStatus()]);
      setRoots(r);
      setStatus(s);
    } catch (e) {
      setMessage(e instanceof Error ? e.message : String(e));
    }
  }, []);

  useEffect(() => {
    void refresh();
    const t = window.setInterval(() => void refresh(), 2000);
    return () => window.clearInterval(t);
  }, [refresh]);

  const onAddAndScan = async () => {
    const trimmed = path.trim();
    if (!trimmed) {
      setMessage('填写绝对路径后再扫描。');
      return;
    }
    setBusy(true);
    setMessage(null);
    try {
      await addRoot(trimmed);
      await startFullScan();
      setMessage('已排队扫描；数秒后可检索。');
      setPath('');
      await refresh();
    } catch (e) {
      setMessage(e instanceof Error ? e.message : String(e));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div>
      <div className="rail-divider" />
      <h3 className="rail-section-title">数据源</h3>
      <p className="rail-hint">
        未索引则无命中。测试可填本仓库源码目录，例如：
        <code>/Volumes/CodeandDataset/FileSearchTool/src</code>
      </p>
      <div className="rail-field-row">
        <input
          className="rail-input"
          type="text"
          value={path}
          onChange={(e) => setPath(e.target.value)}
          placeholder="/绝对路径/目录"
          disabled={busy}
        />
        <button className="rail-btn" type="button" onClick={() => void onAddAndScan()} disabled={busy}>
          {busy ? '扫描中…' : '添加并扫描'}
        </button>
      </div>
      {message ? <p className="rail-msg">{message}</p> : null}
      <h4 className="rail-section-title is-spaced">根目录</h4>
      <ul className="rail-list">
        {roots.length === 0 ? <li>（无）</li> : null}
        {roots.map((r) => (
          <li key={r.id}>{r.path}</li>
        ))}
      </ul>
      <h4 className="rail-section-title is-spaced">索引队列</h4>
      <p className="rail-status">
        {status
          ? `已入库 ${status.indexed_files} · 待处理 ${status.pending_tasks}`
          : '读取中…'}
      </p>
    </div>
  );
}
