import React, { useState, useEffect, useRef } from 'react';
import { 
  Link2, 
  Sparkles, 
  Copy, 
  Check, 
  QrCode, 
  ExternalLink, 
  Trash2, 
  Activity, 
  BookOpen, 
  X, 
  Download,
  ShieldCheck,
  Zap,
  Layers
} from 'lucide-react';
import QRCode from 'qrcode';

interface ShortenedLink {
  id: number;
  slug: string;
  longurl: string;
  shortUrl: string;
  createdAt: string;
  clicks: number;
}

interface ToastMessage {
  id: string;
  text: string;
  type: 'success' | 'error';
}

const API_BASE = import.meta.env.VITE_API_URL || 'http://localhost:8080';

export default function App() {
  const [urlInput, setUrlInput] = useState('');
  const [loading, setLoading] = useState(false);
  const [currentResult, setCurrentResult] = useState<ShortenedLink | null>(null);
  const [history, setHistory] = useState<ShortenedLink[]>(() => {
    const saved = localStorage.getItem('shorty_history');
    return saved ? JSON.parse(saved) : [];
  });
  const [backendOnline, setBackendOnline] = useState<boolean | null>(null);
  const [copiedId, setCopiedId] = useState<string | null>(null);
  const [qrModalUrl, setQrModalUrl] = useState<string | null>(null);
  const [toasts, setToasts] = useState<ToastMessage[]>([]);

  const qrCanvasRef = useRef<HTMLCanvasElement>(null);

  // Check backend health
  useEffect(() => {
    const checkHealth = async () => {
      try {
        const res = await fetch(`${API_BASE}/metrics`, { method: 'GET' });
        setBackendOnline(res.ok);
      } catch {
        setBackendOnline(false);
      }
    };
    checkHealth();
    const interval = setInterval(checkHealth, 15000);
    return () => clearInterval(interval);
  }, []);

  // Save history
  useEffect(() => {
    localStorage.setItem('shorty_history', JSON.stringify(history));
  }, [history]);

  // Render QR Code in modal
  useEffect(() => {
    if (qrModalUrl && qrCanvasRef.current) {
      QRCode.toCanvas(qrCanvasRef.current, qrModalUrl, {
        width: 260,
        margin: 2,
        color: {
          dark: '#0f172a',
          light: '#ffffff'
        }
      });
    }
  }, [qrModalUrl]);

  const showToast = (text: string, type: 'success' | 'error' = 'success') => {
    const id = Math.random().toString(36).substring(2, 9);
    setToasts((prev) => [...prev, { id, text, type }]);
    setTimeout(() => {
      setToasts((prev) => prev.filter((t) => t.id !== id));
    }, 3000);
  };

  const handleShorten = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!urlInput.trim()) return;

    let targetUrl = urlInput.trim();
    if (!/^https?:\/\//i.test(targetUrl)) {
      targetUrl = `https://${targetUrl}`;
    }

    try {
      new URL(targetUrl);
    } catch {
      showToast('Please enter a valid URL', 'error');
      return;
    }

    setLoading(true);

    try {
      const res = await fetch(`${API_BASE}/api/v1/data/shorten`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ longurl: targetUrl }),
      });

      if (!res.ok) {
        const errData = await res.json().catch(() => ({ error: 'Failed to shorten URL' }));
        throw new Error(errData.error || 'Server error');
      }

      const data = await res.json();
      const output = data.output;
      const fullShortUrl = `${API_BASE}/api/v1/${output.slug}`;

      const newLink: ShortenedLink = {
        id: output.id,
        slug: output.slug,
        longurl: output.longurl,
        shortUrl: fullShortUrl,
        createdAt: new Date().toLocaleDateString('en-US', { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' }),
        clicks: 0,
      };

      setCurrentResult(newLink);
      setHistory((prev) => [newLink, ...prev.filter((item) => item.slug !== output.slug)]);
      setUrlInput('');
      showToast('Short link generated successfully!');
    } catch (err: any) {
      showToast(err.message || 'Network error connecting to backend', 'error');
    } finally {
      setLoading(false);
    }
  };

  const copyToClipboard = (text: string, idKey: string) => {
    navigator.clipboard.writeText(text);
    setCopiedId(idKey);
    showToast('Copied to clipboard!');
    setTimeout(() => setCopiedId(null), 2000);
  };

  const deleteLink = (slug: string) => {
    setHistory((prev) => prev.filter((item) => item.slug !== slug));
    if (currentResult?.slug === slug) {
      setCurrentResult(null);
    }
    showToast('Link removed from history');
  };

  const downloadQrCode = () => {
    if (!qrCanvasRef.current) return;
    const link = document.createElement('a');
    link.download = `qrcode-${Date.now()}.png`;
    link.href = qrCanvasRef.current.toDataURL();
    link.click();
  };

  return (
    <div className="app-container">
      {/* Navbar */}
      <header className="navbar">
        <div className="brand">
          <div className="brand-icon">
            <Link2 size={24} />
          </div>
          <span>Shorty<span className="gradient-text">.rs</span></span>
        </div>

        <div className="nav-controls">
          <div className="status-badge">
            <span className={`status-dot ${backendOnline === false ? 'offline' : ''}`} />
            <span>{backendOnline ? 'API Active' : backendOnline === false ? 'API Offline' : 'Checking...'}</span>
          </div>

          <a
            href={`${API_BASE}/swagger-ui/`}
            target="_blank"
            rel="noreferrer"
            className="nav-link"
            title="Swagger API Documentation"
          >
            <BookOpen size={16} />
            <span>API Docs</span>
          </a>
        </div>
      </header>

      {/* Hero Header */}
      <main>
        <section className="hero-section">
          <h1 className="hero-title">
            Shorten links, <br />
            <span className="gradient-text">amplify your reach.</span>
          </h1>
          <p className="hero-subtitle">
            Lightning-fast URL shortener built with Rust, Redis cache, and PostgreSQL. 
            Instant redirection with sub-millisecond responses.
          </p>
        </section>

        {/* Shorten Card */}
        <section className="glass-panel shorten-box">
          <form onSubmit={handleShorten} className="shorten-form">
            <div className="input-wrapper">
              <input
                type="text"
                className="url-input"
                placeholder="Paste your long URL here (e.g. https://github.com/..."
                value={urlInput}
                onChange={(e) => setUrlInput(e.target.value)}
                disabled={loading}
              />
              <Link2 className="input-icon" size={20} />
            </div>

            <button type="submit" className="btn-primary" disabled={loading || !urlInput.trim()}>
              {loading ? (
                <div className="spinner" />
              ) : (
                <>
                  <Sparkles size={18} />
                  <span>Shorten URL</span>
                </>
              )}
            </button>
          </form>

          {/* Current Shortened Result */}
          {currentResult && (
            <div className="result-card">
              <div className="result-info">
                <span className="result-label">Your Shortened Link</span>
                <a
                  href={currentResult.shortUrl}
                  target="_blank"
                  rel="noreferrer"
                  className="short-url-link"
                >
                  {currentResult.shortUrl}
                  <ExternalLink size={16} />
                </a>
                <span className="original-url-preview">{currentResult.longurl}</span>
              </div>

              <div className="result-actions">
                <button
                  onClick={() => copyToClipboard(currentResult.shortUrl, 'current')}
                  className="btn-secondary"
                >
                  {copiedId === 'current' ? <Check size={16} color="#34d399" /> : <Copy size={16} />}
                  <span>{copiedId === 'current' ? 'Copied' : 'Copy'}</span>
                </button>

                <button
                  onClick={() => setQrModalUrl(currentResult.shortUrl)}
                  className="btn-secondary"
                >
                  <QrCode size={16} />
                  <span>QR Code</span>
                </button>
              </div>
            </div>
          )}
        </section>

        {/* Feature Grid */}
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))', gap: '1.25rem', marginBottom: '3.5rem' }}>
          <div className="glass-panel" style={{ padding: '1.5rem', display: 'flex', gap: '1rem', alignItems: 'flex-start' }}>
            <div style={{ padding: '0.6rem', background: 'rgba(99, 102, 241, 0.15)', borderRadius: '10px', color: '#c084fc' }}>
              <Zap size={22} />
            </div>
            <div>
              <h3 style={{ fontSize: '1.05rem', fontWeight: 700, marginBottom: '0.25rem' }}>Sub-ms Redis Cache</h3>
              <p style={{ fontSize: '0.85rem', color: 'var(--text-muted)' }}>High-concurrency GET requests served straight from in-memory Redis cache.</p>
            </div>
          </div>

          <div className="glass-panel" style={{ padding: '1.5rem', display: 'flex', gap: '1rem', alignItems: 'flex-start' }}>
            <div style={{ padding: '0.6rem', background: 'rgba(56, 189, 248, 0.15)', borderRadius: '10px', color: '#38bdf8' }}>
              <ShieldCheck size={22} />
            </div>
            <div>
              <h3 style={{ fontSize: '1.05rem', fontWeight: 700, marginBottom: '0.25rem' }}>Postgres & Base62</h3>
              <p style={{ fontSize: '0.85rem', color: 'var(--text-muted)' }}>Collision-resistant base62 encoding powered by PostgreSQL persistence.</p>
            </div>
          </div>

          <div className="glass-panel" style={{ padding: '1.5rem', display: 'flex', gap: '1rem', alignItems: 'flex-start' }}>
            <div style={{ padding: '0.6rem', background: 'rgba(236, 72, 153, 0.15)', borderRadius: '10px', color: '#f472b6' }}>
              <Activity size={22} />
            </div>
            <div>
              <h3 style={{ fontSize: '1.05rem', fontWeight: 700, marginBottom: '0.25rem' }}>Prometheus Ready</h3>
              <p style={{ fontSize: '0.85rem', color: 'var(--text-muted)' }}>Exposes real-time HTTP metrics on <code>/metrics</code> for Grafana dashboards.</p>
            </div>
          </div>
        </div>

        {/* Dashboard / Link History */}
        <section className="dashboard-section">
          <div className="section-header">
            <h2 className="section-title">
              <Layers size={20} className="gradient-text" />
              Recent Links
            </h2>
            <span style={{ fontSize: '0.85rem', color: 'var(--text-dim)' }}>
              {history.length} {history.length === 1 ? 'link' : 'links'} generated
            </span>
          </div>

          {history.length === 0 ? (
            <div className="glass-panel empty-state">
              <Link2 size={40} style={{ opacity: 0.3 }} />
              <p style={{ fontSize: '1rem', fontWeight: 600 }}>No links generated yet</p>
              <p style={{ fontSize: '0.85rem' }}>Paste a long URL above to create your first short link!</p>
            </div>
          ) : (
            <div className="links-list">
              {history.map((link) => (
                <div key={link.slug} className="glass-panel link-item">
                  <div className="link-details">
                    <a
                      href={link.shortUrl}
                      target="_blank"
                      rel="noreferrer"
                      className="link-short-text"
                    >
                      {link.shortUrl}
                    </a>
                    <span className="link-original-text">{link.longurl}</span>
                    <div className="link-meta">
                      <span>Created {link.createdAt}</span>
                      <span className="badge">Slug: {link.slug}</span>
                    </div>
                  </div>

                  <div style={{ display: 'flex', gap: '0.5rem', alignItems: 'center' }}>
                    <button
                      onClick={() => copyToClipboard(link.shortUrl, link.slug)}
                      className="btn-secondary"
                      style={{ padding: '0.6rem 0.9rem' }}
                      title="Copy short link"
                    >
                      {copiedId === link.slug ? <Check size={15} color="#34d399" /> : <Copy size={15} />}
                    </button>

                    <button
                      onClick={() => setQrModalUrl(link.shortUrl)}
                      className="btn-secondary"
                      style={{ padding: '0.6rem 0.9rem' }}
                      title="View QR Code"
                    >
                      <QrCode size={15} />
                    </button>

                    <button
                      onClick={() => deleteLink(link.slug)}
                      className="btn-secondary"
                      style={{ padding: '0.6rem 0.9rem', color: '#f43f5e' }}
                      title="Delete link"
                    >
                      <Trash2 size={15} />
                    </button>
                  </div>
                </div>
              ))}
            </div>
          )}
        </section>
      </main>

      {/* QR Code Modal */}
      {qrModalUrl && (
        <div className="modal-overlay" onClick={() => setQrModalUrl(null)}>
          <div className="glass-panel modal-content" onClick={(e) => e.stopPropagation()}>
            <button
              onClick={() => setQrModalUrl(null)}
              style={{ position: 'absolute', top: '1.25rem', right: '1.25rem', color: 'var(--text-muted)' }}
            >
              <X size={20} />
            </button>

            <h3 style={{ fontFamily: 'var(--font-display)', fontSize: '1.3rem', fontWeight: 700 }}>
              QR Code Preview
            </h3>
            <p style={{ fontSize: '0.85rem', color: 'var(--text-muted)', marginTop: '0.3rem' }}>
              Scan to open link on mobile
            </p>

            <div className="qr-canvas-container">
              <canvas ref={qrCanvasRef} />
            </div>

            <div style={{ display: 'flex', gap: '0.75rem', justifyContent: 'center' }}>
              <button onClick={downloadQrCode} className="btn-primary" style={{ padding: '0.75rem 1.25rem', fontSize: '0.9rem' }}>
                <Download size={16} />
                Download PNG
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Toast Notification Container */}
      <div className="toast-container">
        {toasts.map((toast) => (
          <div key={toast.id} className={`toast ${toast.type}`}>
            {toast.type === 'error' ? <X size={16} /> : <Check size={16} color="#34d399" />}
            <span>{toast.text}</span>
          </div>
        ))}
      </div>
    </div>
  );
}
