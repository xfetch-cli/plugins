<h1>xfetch Plugins</h1>

<p>
  Official plugins for <strong>xfetch</strong> live in this repository. Each plugin is a
  standalone executable that communicates with the core binary over stdin/stdout using
  the JSON plugin protocol.
</p>

<p>
  Keeping plugins outside the core repository allows the plugin ecosystem to evolve
  independently while preserving the same installation and runtime model.
</p>

<h2>Install a Plugin</h2>

<p>From the default remote repository:</p>

<pre><code>xfetch plugin install animate-logo</code></pre>

<p>
  By default, <code>xfetch plugin install &lt;name&gt;</code> fetches plugins from
  <code>https://github.com/xfetch-cli/plugins.git</code>.
</p>

<h2>Available Plugins</h2>

<table>
  <thead>
    <tr>
      <th>Plugin</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>animate-logo</code></td>
      <td>Animated ASCII logos with sweep, wave, rainbow, sparkle, breathing, and frame modes.</td>
    </tr>
    <tr>
      <td><code>docker</code></td>
      <td>Displays Docker container statistics using the local Docker CLI.</td>
    </tr>
    <tr>
      <td><code>github-stats</code></td>
      <td>Displays GitHub user statistics such as stars, repos, pull requests, and followers.</td>
    </tr>
  </tbody>
</table>

<h2>Developing Locally</h2>

<p>
  This repository is a Cargo workspace, so you can build every plugin together:
</p>

<pre><code class="language-bash">cargo test --workspace</code></pre>

<p>
  Official plugin implementations are grouped under the repository folder
  <code>plugins/&lt;name&gt;</code>, which keeps the root clean as the ecosystem grows.
</p>

<p>
  The shared wire protocol used by the core and plugins is maintained in
  <a href="https://github.com/xfetch-cli/api">xfetch-cli/api</a>.
</p>

<h2>Installed Binary Directory</h2>

<ul>
  <li><strong>Linux/macOS:</strong> <code>~/.config/xfetch/plugins/</code></li>
  <li><strong>Windows:</strong> <code>%APPDATA%/xfetch/plugins/</code></li>
</ul>

<h2>Authoring Plugins</h2>

<p>
  See <a href="./docs/README.md">docs/README.md</a> for the protocol specification,
  naming conventions, testing workflow, and contribution guidelines for new plugins.
</p>
