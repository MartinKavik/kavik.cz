// Kavik Browser Control - Background Service Worker
// Connects to WebSocket server and routes commands to browser via CDP

const WS_URL = 'ws://127.0.0.1:9223';
let ws = null;
let reconnectTimer = null;

// Exponential backoff for reconnection
let reconnectAttempts = 0;
const MAX_RECONNECT_DELAY = 30000;

// CDP state
let debuggerAttached = new Map(); // tabId -> boolean
let currentViewport = { width: null, height: null };

// ============ CDP INFRASTRUCTURE ============

async function attachDebugger(tabId) {
  if (debuggerAttached.get(tabId)) return;

  try {
    await chrome.debugger.attach({ tabId }, '1.3');
    debuggerAttached.set(tabId, true);
    console.log(`[Kavik] CDP: Debugger attached to tab ${tabId}`);

    await chrome.debugger.sendCommand({ tabId }, 'DOM.enable');
    await chrome.debugger.sendCommand({ tabId }, 'Runtime.enable');
    await chrome.debugger.sendCommand({ tabId }, 'Page.enable');
    await chrome.debugger.sendCommand({ tabId }, 'Emulation.setDeviceMetricsOverride', {
      width: 0, height: 0, deviceScaleFactor: 0, mobile: false
    });
  } catch (e) {
    if (e.message && e.message.includes('Another debugger is already attached')) {
      console.log('[Kavik] CDP: Another debugger attached, trying to reuse...');
      debuggerAttached.set(tabId, true);
      try {
        await chrome.debugger.sendCommand({ tabId }, 'DOM.enable');
        await chrome.debugger.sendCommand({ tabId }, 'Runtime.enable');
        await chrome.debugger.sendCommand({ tabId }, 'Page.enable');
        return;
      } catch (e2) {
        debuggerAttached.delete(tabId);
        throw new Error('CDP debugger conflict. Run "kavik-tools exec detach" or close Chrome DevTools.');
      }
    }
    throw e;
  }
}

chrome.debugger.onDetach.addListener((source, reason) => {
  console.log(`[Kavik] CDP: Debugger detached from tab ${source.tabId}, reason: ${reason}`);
  debuggerAttached.delete(source.tabId);
});

chrome.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
  if (changeInfo.status === 'loading') {
    if (debuggerAttached.has(tabId)) {
      console.log(`[Kavik] CDP: Tab ${tabId} navigating, clearing debugger state`);
      debuggerAttached.delete(tabId);
    }
  }
});

// ============ CDP OPERATIONS ============

async function cdpScreenshot(tabId) {
  await attachDebugger(tabId);
  const { data } = await chrome.debugger.sendCommand({ tabId }, 'Page.captureScreenshot', {
    format: 'png'
  });
  return data;
}

async function cdpResize(tabId, width, height) {
  await attachDebugger(tabId);
  await chrome.debugger.sendCommand({ tabId }, 'Emulation.setDeviceMetricsOverride', {
    width: width,
    height: height,
    deviceScaleFactor: 1,
    mobile: width < 768
  });
  currentViewport = { width, height };
  console.log(`[Kavik] CDP: Viewport resized to ${width}x${height}`);
}

async function cdpEvaluate(tabId, expression, retryCount = 0) {
  await attachDebugger(tabId);

  try {
    const { result, exceptionDetails } = await chrome.debugger.sendCommand(
      { tabId }, 'Runtime.evaluate', { expression, returnByValue: true }
    );

    if (exceptionDetails) {
      throw new Error(exceptionDetails.exception?.description || 'Evaluation failed');
    }

    return result.value;
  } catch (e) {
    if (retryCount === 0 && (
      e.message?.includes('Target closed') ||
      e.message?.includes('Cannot find context') ||
      e.message?.includes('Debugger is not attached')
    )) {
      console.log(`[Kavik] CDP: Stale session, re-attaching...`);
      debuggerAttached.delete(tabId);
      return cdpEvaluate(tabId, expression, retryCount + 1);
    }
    throw e;
  }
}

async function cdpGetElementBox(tabId, selector) {
  await attachDebugger(tabId);

  const { root } = await chrome.debugger.sendCommand({ tabId }, 'DOM.getDocument');
  const { nodeId } = await chrome.debugger.sendCommand({ tabId }, 'DOM.querySelector', {
    nodeId: root.nodeId, selector
  });

  if (!nodeId) return null;

  const { model } = await chrome.debugger.sendCommand({ tabId }, 'DOM.getBoxModel', { nodeId });
  const content = model.content;

  return {
    x: content[0],
    y: content[1],
    width: content[2] - content[0],
    height: content[5] - content[1],
    centerX: (content[0] + content[2]) / 2,
    centerY: (content[1] + content[5]) / 2
  };
}

async function cdpClickAt(tabId, x, y) {
  await attachDebugger(tabId);

  const scrollOffset = await cdpEvaluate(tabId, `({ scrollX: window.scrollX, scrollY: window.scrollY })`);
  const viewportX = x - (scrollOffset?.scrollX || 0);
  const viewportY = y - (scrollOffset?.scrollY || 0);

  await chrome.debugger.sendCommand({ tabId }, 'Input.dispatchMouseEvent', {
    type: 'mouseMoved', x: viewportX, y: viewportY, button: 'none'
  });
  await chrome.debugger.sendCommand({ tabId }, 'Input.dispatchMouseEvent', {
    type: 'mousePressed', x: viewportX, y: viewportY, button: 'left', clickCount: 1
  });
  await chrome.debugger.sendCommand({ tabId }, 'Input.dispatchMouseEvent', {
    type: 'mouseReleased', x: viewportX, y: viewportY, button: 'left', clickCount: 1
  });

  console.log(`[Kavik] CDP: Clicked at (${x}, ${y})`);
}

async function cdpClickSelector(tabId, selector) {
  const box = await cdpGetElementBox(tabId, selector);
  if (!box) throw new Error(`Element not found: ${selector}`);
  await cdpClickAt(tabId, box.centerX, box.centerY);
  return { x: box.centerX, y: box.centerY };
}

async function cdpScroll(tabId, x, y, deltaX = 0, deltaY = 0) {
  await attachDebugger(tabId);
  await chrome.debugger.sendCommand({ tabId }, 'Input.dispatchMouseEvent', {
    type: 'mouseWheel', x, y, deltaX, deltaY
  });
}

async function cdpGetHtml(tabId, selector) {
  await attachDebugger(tabId);

  const { root } = await chrome.debugger.sendCommand({ tabId }, 'DOM.getDocument');

  if (selector) {
    const { nodeId } = await chrome.debugger.sendCommand({ tabId }, 'DOM.querySelector', {
      nodeId: root.nodeId, selector
    });
    if (!nodeId) throw new Error(`Element not found: ${selector}`);
    const { outerHTML } = await chrome.debugger.sendCommand({ tabId }, 'DOM.getOuterHTML', { nodeId });
    return outerHTML;
  } else {
    const { outerHTML } = await chrome.debugger.sendCommand({ tabId }, 'DOM.getOuterHTML', {
      nodeId: root.nodeId
    });
    return outerHTML;
  }
}

// ============ WEBSOCKET CONNECTION ============

function safeSend(message) {
  if (ws && ws.readyState === WebSocket.OPEN) {
    try {
      ws.send(typeof message === 'string' ? message : JSON.stringify(message));
      return true;
    } catch (e) {
      console.error('[Kavik] Send failed:', e);
      scheduleReconnect();
      return false;
    }
  }
  return false;
}

function connect() {
  if (ws && (ws.readyState === WebSocket.CONNECTING || ws.readyState === WebSocket.OPEN)) {
    return;
  }

  console.log('[Kavik] Connecting to WebSocket server...');

  try {
    ws = new WebSocket(WS_URL);
  } catch (e) {
    console.error('[Kavik] WebSocket constructor error:', e);
    scheduleReconnect();
    return;
  }

  ws.onopen = () => {
    console.log('[Kavik] Connected to WebSocket server');
    reconnectAttempts = 0;
    if (reconnectTimer) {
      clearTimeout(reconnectTimer);
      reconnectTimer = null;
    }
    safeSend({ clientType: 'extension' });
  };

  ws.onclose = () => {
    console.log('[Kavik] WebSocket connection closed');
    ws = null;
    scheduleReconnect();
  };

  ws.onerror = (error) => {
    console.error('[Kavik] WebSocket error:', error);
  };

  ws.onmessage = async (event) => {
    try {
      const request = JSON.parse(event.data);
      console.log('[Kavik] Received request:', request);

      const response = await handleCommand(request.id, request.command);

      if (response !== null) {
        safeSend({ id: request.id, response: response });
        console.log('[Kavik] Sent response:', response);
      }
    } catch (e) {
      console.error('[Kavik] Error handling message:', e);
    }
  };
}

function scheduleReconnect() {
  if (reconnectTimer) return;

  const delay = Math.min(1000 * Math.pow(2, reconnectAttempts), MAX_RECONNECT_DELAY);
  reconnectAttempts++;

  console.log(`[Kavik] Scheduling reconnect in ${delay}ms (attempt ${reconnectAttempts})`);

  reconnectTimer = setTimeout(() => {
    reconnectTimer = null;
    connect();
  }, delay);
}

// ============ COMMAND HANDLER ============

async function handleCommand(id, command) {
  const type = command.type;

  try {
    // Get the active tab (prefer localhost:4321, fallback to any)
    let tabs = await chrome.tabs.query({ url: 'http://localhost:4321/*' });
    if (tabs.length === 0) {
      tabs = await chrome.tabs.query({ active: true, currentWindow: true });
    }

    if (tabs.length === 0) {
      return { type: 'error', message: 'No tab found' };
    }

    const tab = tabs[0];

    switch (type) {
      case 'ping':
        return { type: 'pong' };

      case 'getStatus':
        return {
          type: 'status',
          connected: true,
          pageUrl: tab.url,
          viewportWidth: currentViewport.width,
          viewportHeight: currentViewport.height
        };

      case 'screenshot':
        try {
          const base64 = await cdpScreenshot(tab.id);
          return { type: 'screenshot', base64 };
        } catch (e) {
          return { type: 'error', message: `Screenshot failed: ${e.message}` };
        }

      case 'resize':
        try {
          await cdpResize(tab.id, command.width, command.height);
          return { type: 'success', data: { width: command.width, height: command.height } };
        } catch (e) {
          return { type: 'error', message: `Resize failed: ${e.message}` };
        }

      case 'navigate':
        try {
          await chrome.tabs.update(tab.id, { url: command.url });
          return { type: 'success', data: { url: command.url } };
        } catch (e) {
          return { type: 'error', message: `Navigate failed: ${e.message}` };
        }

      case 'click':
        try {
          const clickPos = await cdpClickSelector(tab.id, command.selector);
          return { type: 'success', data: clickPos };
        } catch (e) {
          return { type: 'error', message: e.message };
        }

      case 'clickAt':
        try {
          await cdpClickAt(tab.id, command.x, command.y);
          return { type: 'success', data: { x: command.x, y: command.y } };
        } catch (e) {
          return { type: 'error', message: e.message };
        }

      case 'scroll':
        try {
          const centerX = currentViewport.width ? currentViewport.width / 2 : 500;
          const centerY = currentViewport.height ? currentViewport.height / 2 : 400;
          const deltaY = command.toBottom ? 10000 : (command.delta || command.y || 0);
          await cdpScroll(tab.id, centerX, centerY, 0, deltaY);
          return { type: 'success', data: null };
        } catch (e) {
          return { type: 'error', message: e.message };
        }

      case 'getHtml':
        try {
          const html = await cdpGetHtml(tab.id, command.selector);
          return { type: 'html', html };
        } catch (e) {
          return { type: 'error', message: e.message };
        }

      case 'reload':
        try {
          debuggerAttached.delete(tab.id);
          await chrome.tabs.reload(tab.id);
          return { type: 'success', data: 'Page reloaded' };
        } catch (e) {
          return { type: 'error', message: e.message };
        }

      case 'detach':
        try {
          if (debuggerAttached.get(tab.id)) {
            await chrome.debugger.detach({ tabId: tab.id });
            debuggerAttached.delete(tab.id);
          }
          return { type: 'success', data: 'Debugger detached' };
        } catch (e) {
          debuggerAttached.delete(tab.id);
          return { type: 'success', data: `Cleared state (detach error: ${e.message})` };
        }

      default:
        return { type: 'error', message: `Unknown command: ${type}` };
    }
  } catch (e) {
    console.error('[Kavik] Command error:', e);
    return { type: 'error', message: e.message || String(e) };
  }
}

// ============ LIFECYCLE ============

function setupKeepAliveAlarm() {
  chrome.alarms.create('kavikKeepAlive', { periodInMinutes: 0.4 });
  console.log('[Kavik] Keep-alive alarm created');
}

chrome.alarms.onAlarm.addListener((alarm) => {
  if (alarm.name === 'kavikKeepAlive') {
    if (!ws || ws.readyState !== WebSocket.OPEN) {
      console.log('[Kavik] Alarm: WebSocket not connected, reconnecting...');
      connect();
    } else {
      safeSend({ type: 'keepAlive' });
    }
  }
});

chrome.runtime.onStartup.addListener(() => {
  console.log('[Kavik] Service worker started, connecting...');
  connect();
});

chrome.runtime.onInstalled.addListener(() => {
  console.log('[Kavik] Extension installed/updated, connecting...');
  connect();
  setupKeepAliveAlarm();
});

// Initialize
console.log('[Kavik] Service worker loading...');
connect();
setupKeepAliveAlarm();
