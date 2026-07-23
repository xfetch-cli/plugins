<div align="center">
  <h1> Display Resolution Plugin</h1>
  <p>Displays monitor resolution and refresh rate in xfetch.</p>
</div>

<br>

<div align="center">
  <table>
    <tr>
      <td><strong>Kind</strong></td>
      <td><code>info_provider</code></td>
    </tr>
    <tr>
      <td><strong>Binary</strong></td>
      <td><code>xfetch-plugin-display-resolution</code></td>
    </tr>
    <tr>
      <td><strong>Dependencies</strong></td>
      <td><code>xrandr</code> (X11), <code>wlr-randr</code> (Wayland), <code>xdpyinfo</code> (X11 fallback)</td>
    </tr>
  </table>
</div>

<br>

<h2>Build</h2>

<pre><code>cargo build --release --manifest-path plugins/display-resolution/Cargo.toml</code></pre>

<h2>Install</h2>

<pre><code>xfetch plugin install display-resolution</code></pre>

<h2>Configuration</h2>

<pre><code class="language-jsonc">{
  "info_plugins": [
    {
      "plugin": "display-resolution"
    }
  ],
  "modules": [
    "os",
    "kernel",
    "plugin:display-resolution",
    "shell",
    "cpu",
    "memory"
  ]
}</code></pre>

<p>The display-resolution plugin does not require any arguments.</p>

<h2>Output</h2>

<table>
  <thead>
    <tr>
      <th>Platform</th>
      <th>Example Output</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Single monitor (X11)</td>
      <td><code> DP-1: 1920x1080 @ 144.00* (primary)</code></td>
    </tr>
    <tr>
      <td>Multiple monitors</td>
      <td><pre> eDP-1: 1920x1080 @ 60.00* (primary)<br>   HDMI-1: 3840x2160 @ 30.00</pre></td>
    </tr>
    <tr>
      <td>macOS</td>
      <td><code> Display: 2560 x 1600</code></td>
    </tr>
    <tr>
      <td>Not detected</td>
      <td><code> Display: unknown</code></td>
    </tr>
  </tbody>
</table>

<h2>How It Works</h2>

<ol>
  <li>xfetch sends a JSON request with <code>kind: "info_provider"</code>.</li>
  <li>On Linux, the plugin tries detection in this order: <code>xrandr</code> → <code>wlr-randr</code> → <code>xdpyinfo</code>.</li>
  <li>On macOS, the plugin uses <code>system_profiler SPDisplaysDataType</code>.</li>
  <li>On Windows, the plugin uses a PowerShell script with <code>GetDeviceCaps</code>.</li>
  <li>The plugin returns a JSON response with the formatted lines.</li>
  <li>xfetch displays them under the <code>plugin:display-resolution</code> module key.</li>
</ol>

<h2>Notes</h2>

<ul>
  <li>On Linux, requires at least one display utility (<code>xrandr</code>, <code>wlr-randr</code>, or <code>xdpyinfo</code>).</li>
  <li>On Windows, requires PowerShell.</li>
  <li>On macOS, requires <code>system_profiler</code> (included by default).</li>
  <li>Multiple monitors are listed with indentation.</li>
</ul>
