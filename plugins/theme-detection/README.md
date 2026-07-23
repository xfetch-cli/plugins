<div align="center">
  <h1> Theme Detection Plugin</h1>
  <p>Detects and displays current GTK and KDE Plasma theme settings in xfetch.</p>
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
      <td><code>xfetch-plugin-theme-detection</code></td>
    </tr>
    <tr>
      <td><strong>Dependencies</strong></td>
      <td><code>gsettings</code> (GTK), <code>plasmarc</code>/<code>kdeglobals</code> (KDE)</td>
    </tr>
  </table>
</div>

<br>

<h2>Build</h2>

<pre><code>cargo build --release --manifest-path plugins/theme-detection/Cargo.toml</code></pre>

<h2>Install</h2>

<pre><code>xfetch plugin install theme-detection</code></pre>

<h2>Configuration</h2>

<pre><code class="language-jsonc">{
  "info_plugins": [
    {
      "plugin": "theme-detection"
    }
  ],
  "modules": [
    "os",
    "kernel",
    "plugin:theme-detection",
    "shell",
    "cpu",
    "memory"
  ]
}</code></pre>

<p>The theme-detection plugin does not require any arguments.</p>

<h2>Output</h2>

<table>
  <thead>
    <tr>
      <th>Environment</th>
      <th>Example Output</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>GNOME / GTK</td>
      <td><pre> GTK Theme: Adwaita-dark (dark)<br>   Icons: Adwaita<br>   Cursor: Adwaita<br>   Font: Cantarell 11</pre></td>
    </tr>
    <tr>
      <td>KDE Plasma</td>
      <td><pre> GTK: Breeze (light)<br>   Plasma: breeze-dark<br>   Colors: BreezeDark<br>   Icons: breeze-dark<br>   Cursor: breeze_cursors</pre></td>
    </tr>
    <tr>
      <td>No theme detected</td>
      <td><code> Theme: not detected</code></td>
    </tr>
  </tbody>
</table>

<h2>How It Works</h2>

<ol>
  <li>xfetch sends a JSON request with <code>kind: "info_provider"</code>.</li>
  <li>The plugin reads GTK settings via <code>gsettings get org.gnome.desktop.interface</code>.</li>
  <li>KDE Plasma themes are read from <code>~/.config/plasmarc</code> and <code>~/.config/kdeglobals</code>.</li>
  <li>The plugin returns a JSON response with the formatted lines.</li>
  <li>xfetch displays them under the <code>plugin:theme-detection</code> module key.</li>
</ol>

<h2>Notes</h2>

<ul>
  <li>GTK detection works on any desktop using <code>gsettings</code> (GNOME, Budgie, Cinnamon, etc.).</li>
  <li>KDE detection reads from standard Plasma config files.</li>
  <li>If both GTK and KDE are detected, both are shown (GTK listed first).</li>
  <li>Uses <code>prefer-dark</code>/<code>prefer-light</code> color scheme from GNOME for dark/light detection.</li>
</ul>
