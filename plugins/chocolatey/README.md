<div align="center">
  <h1> Chocolatey Plugin</h1>
  <p>Counts packages installed via Chocolatey (Windows package manager).</p>
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
      <td><code>xfetch-plugin-chocolatey</code></td>
    </tr>
    <tr>
      <td><strong>Dependencies</strong></td>
      <td><code>choco</code> CLI (Windows only)</td>
    </tr>
  </table>
</div>

<br>

<h2>Build</h2>

<pre><code>cargo build --release --manifest-path plugins/chocolatey/Cargo.toml</code></pre>

<h2>Install</h2>

<pre><code>xfetch plugin install chocolatey</code></pre>

<h2>Configuration</h2>

<pre><code class="language-jsonc">{
  "info_plugins": [
    {
      "plugin": "chocolatey"
    }
  ],
  "modules": [
    "os",
    "kernel",
    "plugin:chocolatey",
    "shell",
    "cpu",
    "memory"
  ]
}</code></pre>

<p>The chocolatey plugin does not require any arguments. It counts packages with <code>choco list -r</code> (one <code>name|version</code> row per installed package, no banner or summary line).</p>

<h2>Platform Support</h2>

<p>
  <strong>Windows only.</strong> Chocolatey is a Windows package manager; on
  Linux and macOS the plugin responds with <code>Chocolatey: not installed</code>.
</p>

<h2>Output</h2>

<table>
  <thead>
    <tr>
      <th>State</th>
      <th>Example Output</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Packages installed</td>
      <td><code> 20 (chocolatey)</code></td>
    </tr>
    <tr>
      <td>Choco not installed</td>
      <td><code>Chocolatey: not installed</code></td>
    </tr>
    <tr>
      <td>No packages</td>
      <td><code>Chocolatey: no packages installed</code></td>
    </tr>
  </tbody>
</table>

<h2>How It Works</h2>

<ol>
  <li>xfetch sends a JSON request with <code>kind: "info_provider"</code>.</li>
  <li>The plugin runs <code>choco list -r</code> and counts the rows.</li>
  <li>If choco is missing or nothing is installed, it reports gracefully.</li>
  <li>The plugin returns a JSON response with the formatted line.</li>
  <li>xfetch displays it under the <code>plugin:chocolatey</code> module key.</li>
</ol>

<h2>Notes</h2>

<ul>
  <li>Chocolatey 2.4+ removed <code>--local-only</code>; <code>-r</code> (<code>--limit-output</code>) works on all versions.</li>
  <li>The work runs with a 10 s <code>with_timeout</code> budget — a slow first run responds with <code>Chocolatey: timed out</code> instead of hanging xfetch.</li>
</ul>
