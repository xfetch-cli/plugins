<div align="center">
  <h1>Wasm Proc Plugin</h1>
  <p>Load average, memory usage and uptime read from <code>/proc</code>.</p>
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
      <td><code>wasm-proc.wasm</code> (core module, under 2 KiB)</td>
    </tr>
    <tr>
      <td><strong>Runtime</strong></td>
      <td><code>wasm32-wasip1</code></td>
    </tr>
    <tr>
      <td><strong>Capabilities</strong></td>
      <td><code>fs: /proc</code> (read-only)</td>
    </tr>
    <tr>
      <td><strong>Toolchain</strong></td>
      <td>clang with the WebAssembly backend and <code>wasm-ld</code></td>
    </tr>
  </table>
</div>

<br>

<h2>Build</h2>

<pre><code>sh build.sh</code></pre>

<p>
  Freestanding C: no wasi-libc. The guest declares the WASI preview 1
  imports it needs (<code>path_open</code>, <code>fd_read</code>,
  <code>fd_close</code>, <code>fd_write</code>) directly with clang import
  attributes.
</p>

<h2>Install</h2>

<pre><code>xfetch plugin install ./plugins/wasm-proc</code></pre>

<h2>Configuration</h2>

<pre><code class="language-jsonc">{
  "info_plugins": [
    { "plugin": "wasm-proc" }
  ],
  "modules": ["os", "kernel", "plugin:wasm-proc", "shell", "cpu"]
}</code></pre>

<h2>Output</h2>

<pre><code>load: 2.39 2.33 1.63
uptime: 6h 40m
memory: 8% used (7.9 GiB / 93.8 GiB)</code></pre>

<p>
  The manifest preopens <code>/proc</code> read-only; the guest cannot read
  anything else. Linux and WSL only, since it depends on procfs.
</p>
