<div align="center">
  <h1> Timezone Plugin</h1>
  <p>Displays current time, date, and timezone information in xfetch.</p>
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
      <td><code>xfetch-plugin-timezone</code></td>
    </tr>
    <tr>
      <td><strong>Dependencies</strong></td>
      <td><code>date</code>, <code>timedatectl</code> (optional)</td>
    </tr>
  </table>
</div>

<br>

<h2>Build</h2>

<pre><code>cargo build --release --manifest-path plugins/timezone/Cargo.toml</code></pre>

<h2>Install</h2>

<pre><code>xfetch plugin install timezone</code></pre>

<h2>Configuration</h2>

<pre><code class="language-jsonc">{
  "info_plugins": [
    {
      "plugin": "timezone"
    }
  ],
  "modules": [
    "os",
    "kernel",
    "plugin:timezone",
    "shell",
    "cpu",
    "memory"
  ]
}</code></pre>

<h3>Args</h3>

<table>
  <thead>
    <tr>
      <th>Field</th>
      <th>Type</th>
      <th>Required</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>format</code></td>
      <td><code>string</code></td>
      <td>No</td>
      <td><code>date</code> format string. Default: <code>%Z %z</code> (timezone name + UTC offset).</td>
    </tr>
  </tbody>
</table>

<h2>Output</h2>

<table>
  <thead>
    <tr>
      <th>Example Output</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><pre> Wednesday, 23 July 2026  14:30<br>   America/New York (EST -05:00)</pre></td>
    </tr>
  </tbody>
</table>

<h2>How It Works</h2>

<ol>
  <li>xfetch sends a JSON request with <code>kind: "info_provider"</code>.</li>
  <li>The plugin detects the timezone from <code>/etc/timezone</code>, <code>/etc/localtime</code>, or <code>timedatectl</code>.</li>
  <li>The plugin runs <code>date</code> to get the current time and UTC offset.</li>
  <li>The plugin returns a JSON response with the formatted lines.</li>
  <li>xfetch displays them under the <code>plugin:timezone</code> module key.</li>
</ol>

<h2>Notes</h2>

<ul>
  <li>Timezone detection order: <code>/etc/timezone</code> → <code>/etc/localtime</code> symlink → <code>timedatectl</code>.</li>
  <li>The <code>format</code> arg is passed directly to <code>date</code> — use standard <code>date</code> format specifiers.</li>
  <li>Works on Linux, macOS, and most Unix-like systems.</li>
</ul>
