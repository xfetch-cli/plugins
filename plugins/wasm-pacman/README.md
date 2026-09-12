<div align="center">
  <h1>Wasm Pacman Plugin</h1>
  <p>Package counts (repository vs AUR/foreign) from the local pacman database.</p>
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
      <td><code>wasm-pacman.wasm</code> (core module)</td>
    </tr>
    <tr>
      <td><strong>Runtime</strong></td>
      <td><code>wasm32-wasip1</code> via <code>GOOS=wasip1</code></td>
    </tr>
    <tr>
      <td><strong>Capabilities</strong></td>
      <td><code>exec: pacman</code></td>
    </tr>
    <tr>
      <td><strong>Toolchain</strong></td>
      <td>Go &gt;= 1.24 (host bridge via <code>//go:wasmimport</code>)</td>
    </tr>
  </table>
</div>

<br>

<h2>Build</h2>

<pre><code>GOOS=wasip1 GOARCH=wasm go build -o dist/wasm-pacman.wasm .</code></pre>

<h2>Install</h2>

<pre><code>xfetch plugin install ./plugins/wasm-pacman</code></pre>

<h2>Configuration</h2>

<pre><code class="language-jsonc">{
  "info_plugins": [
    {
      "plugin": "wasm-pacman",
      "args": { "samples": 3 }
    }
  ],
  "modules": ["os", "kernel", "plugin:wasm-pacman", "shell", "cpu"]
}</code></pre>

<h3>Args</h3>

<table>
  <thead>
    <tr><th>Field</th><th>Type</th><th>Default</th><th>Description</th></tr>
  </thead>
  <tbody>
    <tr>
      <td><code>samples</code></td>
      <td>number</td>
      <td><code>3</code></td>
      <td>Foreign package names appended as a sample line.</td>
    </tr>
  </tbody>
</table>

<h2>Output</h2>

<pre><code>packages: 1084 installed (1066 repo, 18 AUR/foreign)
foreign: android-sdk-cmdline-tools-latest, android-sdk-platform-tools, ...</code></pre>

<p>
  The Go guest talks to the host through the JSON bridge implemented with
  <code>//go:wasmimport</code> and <code>//go:wasmexport</code>; the manifest
  only allows the <code>pacman</code> program and no shell is involved.
</p>
