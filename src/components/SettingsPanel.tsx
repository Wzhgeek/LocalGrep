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
      setMessage('请先填写要索引的目录绝对路径。');
      return;
    }
    setBusy(true);
    setMessage(null);
    try {
      await addRoot(trimmed);
      await startFullScan();
      setMessage('已加入目录并开始扫描，请稍等几秒后再在左侧搜索。');
      setPath('');
      await refresh();
    } catch (e) {
      setMessage(e instanceof Error ? e.message : String(e));
    } finally {
      setBusy(false);
    }
  };

  return (
    <section className="settings-panel">
      <h3>索引目录</h3>
      <p className="settings-hint">
        搜索前必须先添加至少一个目录并完成扫描。可填本仓库源码目录做测试，例如：
        <code>/Volumes/CodeandDataset/FileSearchTool/src</code>
      </p>
      <div className="settings-row">
        <input
          type="text"
          value={path}
          onChange={(e) => setPath(e.target.value)}
          placeholder="/绝对路径/到/要索引的文件夹"
          disabled={busy}
        />
        <button type="button" onClick={() => void onAddAndScan()} disabled={busy}>
          {busy ? '处理中…' : '添加并扫描'}
        </button>
      </div>
      {message ? <p className="settings-message">{message}</p> : null}
      <h4>已添加根目录</h4>
      <ul className="settings-roots">
        {roots.length === 0 ? <li>（暂无）</li> : null}
        {roots.map((r) => (
          <li key={r.id}>
            <code>{r.path}</code>
          </li>
        ))}
      </ul>
      <h4>索引状态</h4>
      <p className="settings-status">
        {status
          ? `已入库文件: ${status.indexed_files} · 待处理任务: ${status.pending_tasks}`
          : '加载中…'}
      </p>
    </section>
  );
}
