<div align="center">
  <h1> Music Player Plugin</h1>
  <p>Displays currently playing track from MPD and/or Spotify in xfetch.</p>
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
      <td><code>xfetch-plugin-music-player</code></td>
    </tr>
    <tr>
      <td><strong>Dependencies</strong></td>
      <td><code>mpc</code> (MPD), <code>playerctl</code> (Spotify)</td>
    </tr>
  </table>
</div>

<br>

<h2>Build</h2>

<pre><code>cargo build --release --manifest-path plugins/music-player/Cargo.toml</code></pre>

<h2>Install</h2>

<pre><code>xfetch plugin install music-player</code></pre>

<h2>Configuration</h2>

<pre><code class="language-jsonc">{
  "info_plugins": [
    {
      "plugin": "music-player"
    }
  ],
  "modules": [
    "os",
    "kernel",
    "plugin:music-player",
    "shell",
    "cpu",
    "memory"
  ]
}</code></pre>

<p>The music-player plugin does not require any arguments. It detects MPD via <code>mpc status</code> and Spotify via <code>playerctl</code>.</p>

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
      <td>MPD playing</td>
      <td><pre> MPD: Song Title - Artist<br>  ▶ playing</pre></td>
    </tr>
    <tr>
      <td>Spotify playing</td>
      <td><code> Spotify: Artist - Song Title</code></td>
    </tr>
    <tr>
      <td>Both active</td>
      <td><pre> Music Players:<br>   MPD: Song Title - Artist<br>   Spotify: Artist - Song</pre></td>
    </tr>
    <tr>
      <td>No player detected</td>
      <td><code> Music: no active player</code></td>
    </tr>
  </tbody>
</table>

<h2>How It Works</h2>

<ol>
  <li>xfetch sends a JSON request with <code>kind: "info_provider"</code>.</li>
  <li>The plugin checks MPD via <code>mpc status</code> and Spotify via <code>playerctl -p spotify metadata</code>.</li>
  <li>If no player is active, it reports it gracefully.</li>
  <li>The plugin returns a JSON response with the formatted lines.</li>
  <li>xfetch displays them under the <code>plugin:music-player</code> module key.</li>
</ol>

<h2>Notes</h2>

<ul>
  <li>MPD detection requires <code>mpc</code> CLI in <code>PATH</code> and a running MPD daemon.</li>
  <li>Spotify detection requires <code>playerctl</code> and a running Spotify client.</li>
  <li>Both players are checked independently; if both are active, all info is shown.</li>
</ul>
