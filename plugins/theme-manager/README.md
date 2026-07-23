<div align="center">
  <h1>Theme Manager Plugin</h1>
  <p>Browse, search, install, and inspect xfetch themes from the remote registry.</p>
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
      <td><code>xfetch-plugin-theme-manager</code></td>
    </tr>
    <tr>
      <td><strong>Dependencies</strong></td>
      <td><code>curl</code> CLI</td>
    </tr>
    <tr>
      <td><strong>Registry</strong></td>
      <td><code>github.com/xfetch-cli/configs</code> (themes/index.json)</td>
    </tr>
  </table>
</div>

<br>

<h2>Build</h2>

<pre><code>cargo build --release --manifest-path plugins/theme-manager/Cargo.toml</code></pre>

<h2>Install</h2>

<pre><code>xfetch plugin install theme-manager</code></pre>

<h2>Actions</h2>

<h3>list</h3>

<p>Display all available themes from the registry:</p>

<pre><code class="language-jsonc">{
  "info_plugins": [
    { "plugin": "theme-manager" }
  ],
  "modules": ["plugin:theme-manager"]
}</code></pre>

<p>This is the default action when none is specified.</p>

<h3>search</h3>

<p>Search themes by name, description, author, or tags:</p>

<pre><code class="language-jsonc">{
  "info_plugins": [
    {
      "plugin": "theme-manager",
      "args": {
        "action": "search",
        "query": "dark"
      }
    }
  ],
  "modules": ["plugin:theme-manager"]
}</code></pre>

<h3>info</h3>

<p>Show detailed information about a specific theme:</p>

<pre><code class="language-jsonc">{
  "info_plugins": [
    {
      "plugin": "theme-manager",
      "args": {
        "action": "info",
        "name": "dracula"
      }
    }
  ],
  "modules": ["plugin:theme-manager"]
}</code></pre>

<h3>install</h3>

<p>Download and install a theme from the registry:</p>

<pre><code class="language-jsonc">{
  "info_plugins": [
    {
      "plugin": "theme-manager",
      "args": {
        "action": "install",
        "name": "dracula"
      }
    }
  ],
  "modules": ["plugin:theme-manager"]
}</code></pre>

<p>The theme file is saved to <code>~/.config/xfetch/themes/&lt;name&gt;.jsonc</code>.</p>

<h2>Args</h2>

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
      <td><code>action</code></td>
      <td><code>string</code></td>
      <td>No</td>
      <td>One of <code>list</code> (default), <code>search</code>, <code>info</code>, <code>install</code></td>
    </tr>
    <tr>
      <td><code>name</code></td>
      <td><code>string</code></td>
      <td>For <code>info</code> and <code>install</code></td>
      <td>Theme name (e.g., <code>dracula</code>, <code>nord</code>)</td>
    </tr>
    <tr>
      <td><code>query</code></td>
      <td><code>string</code></td>
      <td>For <code>search</code></td>
      <td>Search term matching name, description, author, or tags</td>
    </tr>
    <tr>
      <td><code>registry</code></td>
      <td><code>string</code></td>
      <td>No</td>
      <td>Custom registry URL (defaults to xfetch-cli/configs)</td>
    </tr>
  </tbody>
</table>

<h2>Output</h2>

<h3>list example</h3>

<pre>Theme Manager -- 6 themes available

  dracula  xscriptor  section  (#dark #dracula #popular)
       Dark magenta, red, and cyan palette inspired by the Dracula color scheme.

  nord  xscriptor  section  (#light #nord #arctic #minimal)
       Cool blue and arctic cyan palette based on the Nord color scheme.</pre>

<h3>install example</h3>

<pre>Theme 'dracula' installed successfully.
  Path: /home/user/.config/xfetch/themes/dracula.jsonc
  Author: xscriptor
  Layout: section

Activate with: xfetch theme set dracula
Or add to config.jsonc: "theme": "dracula"</pre>

<h2>How It Works</h2>

<ol>
  <li>xfetch sends a JSON request with <code>kind: "info_provider"</code> and the configured <code>args</code>.</li>
  <li>The plugin fetches the theme registry from the remote URL via <code>curl</code>.</li>
  <li>Depending on the <code>action</code>, it filters, displays, or downloads theme files.</li>
  <li>For <code>install</code>, the theme JSONC file is saved to <code>~/.config/xfetch/themes/</code>.</li>
  <li>The plugin returns a JSON response with the formatted lines.</li>
  <li>xfetch displays them under the <code>plugin:theme-manager</code> module key.</li>
</ol>

<h2>Notes</h2>

<ul>
  <li>Requires <code>curl</code> to be installed and available in <code>PATH</code>.</li>
  <li>Network connectivity is required for all actions.</li>
  <li>The registry URL can be overridden with the <code>registry</code> arg for self-hosted registries.</li>
  <li>After installing a theme, activate it with <code>xfetch theme set &lt;name&gt;</code> or by adding <code>"theme": "&lt;name&gt;"</code> to <code>config.jsonc</code>.</li>
</ul>
