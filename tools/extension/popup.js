// Kavik Browser Control - Popup Script

async function updateStatus() {
  const wsDot = document.getElementById('ws-dot');
  const wsStatus = document.getElementById('ws-status');
  const pageUrl = document.getElementById('page-url');
  const viewport = document.getElementById('viewport');

  try {
    // Get tabs - prefer localhost:4321, fallback to active tab
    let tabs = await chrome.tabs.query({ url: 'http://localhost:4321/*' });
    if (tabs.length === 0) {
      tabs = await chrome.tabs.query({ active: true, currentWindow: true });
    }

    if (tabs.length === 0) {
      wsDot.className = 'status-dot disconnected';
      wsStatus.textContent = 'No tab found';
      pageUrl.textContent = '-';
      viewport.textContent = '-';
      return;
    }

    const tab = tabs[0];
    pageUrl.textContent = tab.url || '-';

    // Get viewport size
    try {
      const results = await chrome.scripting.executeScript({
        target: { tabId: tab.id },
        func: () => ({
          width: window.innerWidth,
          height: window.innerHeight
        })
      });

      if (results && results[0] && results[0].result) {
        const { width, height } = results[0].result;
        viewport.textContent = `${width} x ${height}`;
      }
    } catch (e) {
      viewport.textContent = 'Cannot access page';
    }

    wsDot.className = 'status-dot connected';
    wsStatus.textContent = 'Extension Active';

  } catch (e) {
    console.error('Status check error:', e);
    wsDot.className = 'status-dot disconnected';
    wsStatus.textContent = 'Error: ' + e.message;
  }
}

document.addEventListener('DOMContentLoaded', updateStatus);
document.getElementById('refresh-btn').addEventListener('click', updateStatus);
