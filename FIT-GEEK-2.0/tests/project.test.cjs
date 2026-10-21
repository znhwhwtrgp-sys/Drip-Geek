const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { spawnSync } = require('node:child_process');
const root = path.resolve(__dirname, '..');
const html = fs.readFileSync(path.join(root, 'index.html'), 'utf8');

test('embedded frontend JavaScript parses', () => {
  const scripts = [...html.matchAll(/<script\b[^>]*>([\s\S]*?)<\/script>/gi)].map(m => m[1]).filter(s => s.trim());
  assert.ok(scripts.length, 'expected inline application script');
  for (const script of scripts) {
    const result = spawnSync(process.execPath, ['--check', '-'], { input: script, encoding: 'utf8' });
    assert.equal(result.status, 0, result.stderr || 'JavaScript syntax check failed');
  }
});

test('product branding and core feature entry points remain present', () => {
  assert.match(html, /FIT-GEEK/);
  for (const token of ['function home(', 'function outfitsPage(', 'function calendarPage(', 'function statsPage(', 'function morePage(', 'function exportData(', 'function importData(']) {
    assert.ok(html.includes(token), `missing feature/function: ${token}`);
  }
});

test('save implementation waits for transaction and surfaces failures', () => {
  assert.match(html, /tx\.oncomplete=res/);
  assert.match(html, /tx\.onabort/);
  assert.match(html, /throw e\}\}/);
  assert.match(html, /lastCommitted/);
});

test('Tauri desktop configuration is valid JSON and least-privilege capability exists', () => {
  const conf = JSON.parse(fs.readFileSync(path.join(root, 'src-tauri/tauri.conf.json'), 'utf8'));
  const caps = JSON.parse(fs.readFileSync(path.join(root, 'src-tauri/capabilities/default.json'), 'utf8'));
  assert.equal(conf.productName, 'FIT-GEEK 2.0');
  assert.equal(conf.app.windows[0].title, 'FIT-GEEK 2.0');
  assert.deepEqual(caps.permissions, ['core:default']);
});


test('native SQLite state persistence and validated write commands are wired to frontend', () => {
  const rust = fs.readFileSync(path.join(root, 'src-tauri/src/main.rs'), 'utf8');
  assert.match(rust, /CREATE TABLE IF NOT EXISTS app_state/);
  assert.match(rust, /fn storage_load/);
  assert.match(rust, /fn storage_save/);
  assert.match(rust, /unchecked_transaction/);
  assert.match(rust, /automatic-before-save/);
  assert.match(html, /invoke\('storage_save',\{data\}\)/);
  assert.match(html, /invoke\('storage_load'\)/);
});
