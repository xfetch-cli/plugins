<div align="center">
  <h1> User Info Plugin</h1>
  <p>Displays user account information (name, UID, GID, home, shell, groups) in xfetch.</p>
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
      <td><code>xfetch-plugin-user-info</code></td>
    </tr>
    <tr>
      <td><strong>Dependencies</strong></td>
      <td><code>id</code>, <code>getent</code>, <code>groups</code></td>
    </tr>
  </table>
</div>

<br>

<h2>Build</h2>

<pre><code>cargo build --release --manifest-path plugins/user-info/Cargo.toml</code></pre>

<h2>Install</h2>

<pre><code>xfetch plugin install user-info</code></pre>

<h2>Configuration</h2>

<pre><code class="language-jsonc">{
  "info_plugins": [
    {
      "plugin": "user-info"
    }
  ],
  "modules": [
    "os",
    "kernel",
    "plugin:user-info",
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
      <td><code>show_groups</code></td>
      <td><code>bool</code></td>
      <td>No</td>
      <td>Show user group memberships (max 10). Default: <code>false</code>.</td>
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
      <td><pre> John (john)<br>   uid: 1000  gid: 1000<br>   /home/john<br>   zsh</pre></td>
    </tr>
    <tr>
      <td>With groups enabled</td>
      <td><pre> John (john)<br>   uid: 1000  gid: 1000<br>   /home/john<br>   zsh<br>   groups: wheel, users, docker</pre></td>
    </tr>
  </tbody>
</table>

<h2>How It Works</h2>

<ol>
  <li>xfetch sends a JSON request with <code>kind: "info_provider"</code>.</li>
  <li>The plugin reads <code>USER</code>/<code>LOGNAME</code> env vars or runs <code>whoami</code>.</li>
  <li>UID/GID are collected via <code>id -u</code> / <code>id -g</code>.</li>
  <li>GECOS (full name) is looked up via <code>getent passwd</code>.</li>
  <li>Groups are listed via <code>groups</code> (if enabled).</li>
  <li>The plugin returns a JSON response with the formatted lines.</li>
  <li>xfetch displays them under the <code>plugin:user-info</code> module key.</li>
</ol>
