<div align="center">
  <h1>Wasm IP Geo Plugin</h1>
  <p>Public IP, location, network and timezone through the sandboxed HTTP host call.</p>
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
      <td><code>wasm-ip-geo.wasm</code> (component)</td>
    </tr>
    <tr>
      <td><strong>Runtime</strong></td>
      <td>Component model, world <code>plugin</code></td>
    </tr>
    <tr>
      <td><strong>Capabilities</strong></td>
      <td><code>https://ipapi.co/*</code></td>
    </tr>
    <tr>
      <td><strong>Toolchain</strong></td>
      <td><code>componentize-py</code> &gt;= 0.25</td>
    </tr>
  </table>
</div>

<br>

<h2>Build</h2>

<pre><code>python3 -m venv .venv
. .venv/bin/activate
pip install -r requirements.txt
componentize-py -d ../../../api/wit -w plugin componentize app -p . -o dist/wasm-ip-geo.wasm</code></pre>

<h2>Install</h2>

<pre><code>xfetch plugin install ./plugins/wasm-ip-geo</code></pre>

<h2>Configuration</h2>

<pre><code class="language-jsonc">{
  "info_plugins": [
    {
      "plugin": "wasm-ip-geo",
      "args": { "fields": ["ip", "location", "org", "timezone"] }
    }
  ],
  "modules": ["os", "kernel", "plugin:wasm-ip-geo", "shell", "cpu"]
}</code></pre>

<h3>Args</h3>

<table>
  <thead>
    <tr><th>Field</th><th>Type</th><th>Default</th><th>Description</th></tr>
  </thead>
  <tbody>
    <tr>
      <td><code>fields</code></td>
      <td>string[]</td>
      <td>all four</td>
      <td>Any of <code>ip</code>, <code>location</code>, <code>org</code>, <code>timezone</code>.</td>
    </tr>
  </tbody>
</table>

<h2>Output</h2>

<pre><code>ip: 203.0.113.42 (IPv4)
location: Madrid, MD - Spain
network: Example Telecom S.A.
timezone: Europe/Madrid (UTC+0200)</code></pre>

<p>
  The manifest only allows <code>ipapi.co</code>; the guest cannot reach any
  other origin. Lookup failures degrade to an explanatory line.
</p>
