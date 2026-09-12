<div align="center">
  <h1>Wasm Crypto Plugin</h1>
  <p>Spot prices for BTC, ETH and more via the sandboxed HTTP host call.</p>
</div>

<br>

<div align="center">
  <table>
    <tr>
      <td><strong>Kind</strong></td>
      <td><code>info_provider</code></td>
    </tr>
    <tr>
      <td><strong>Artifact</strong></td>
      <td><code>xfetch-plugin-wasm-crypto.wasm</code> (core module)</td>
    </tr>
    <tr>
      <td><strong>Runtime</strong></td>
      <td><code>wasm32-wasip1</code></td>
    </tr>
    <tr>
      <td><strong>Capabilities</strong></td>
      <td><code>https://api.coinbase.com/*</code></td>
    </tr>
  </table>
</div>

<br>

<h2>Build</h2>

<pre><code>rustup target add wasm32-wasip1
cargo build --release --target wasm32-wasip1 -p xfetch-plugin-wasm-crypto</code></pre>

<h2>Install</h2>

<pre><code>xfetch plugin install ./plugins/wasm-crypto</code></pre>

<h2>Configuration</h2>

<pre><code class="language-jsonc">{
  "info_plugins": [
    {
      "plugin": "wasm-crypto",
      "args": {
        "assets": ["BTC", "ETH", "SOL"],
        "currency": "USD"
      }
    }
  ],
  "modules": ["os", "kernel", "plugin:wasm-crypto", "shell", "cpu"]
}</code></pre>

<h3>Args</h3>

<table>
  <thead>
    <tr><th>Field</th><th>Type</th><th>Default</th><th>Description</th></tr>
  </thead>
  <tbody>
    <tr><td><code>assets</code></td><td>string[]</td><td><code>["BTC","ETH","SOL"]</code></td><td>Symbols to quote (up to five).</td></tr>
    <tr><td><code>currency</code></td><td>string</td><td><code>USD</code></td><td>Quote currency code; symbols for USD/EUR/GBP.</td></tr>
  </tbody>
</table>

<h2>Output</h2>

<pre><code>BTC: $77,331.71
ETH: $2,536.05
XRP: $1.37</code></pre>

<p>
  Network failures degrade to a per-asset <code>unavailable</code> line
  instead of failing the fetch. The manifest only allows the Coinbase origin;
  every other URL is denied by the sandbox.
</p>
