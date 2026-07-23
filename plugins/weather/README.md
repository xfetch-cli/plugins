<div align="center">
  <h1> Weather Plugin</h1>
  <p>Displays current weather conditions in xfetch.</p>
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
      <td><code>xfetch-plugin-weather</code></td>
    </tr>
    <tr>
      <td><strong>Dependencies</strong></td>
      <td><code>curl</code> CLI</td>
    </tr>
    <tr>
      <td><strong>API</strong></td>
      <td><a href="https://wttr.in">wttr.in</a> (free, no key required)</td>
    </tr>
  </table>
</div>

<br>

<h2>Build</h2>

<pre><code>cargo build --release --manifest-path plugins/weather/Cargo.toml</code></pre>

<h2>Install</h2>

<pre><code>xfetch plugin install weather</code></pre>

<h2>Configuration</h2>

<pre><code class="language-jsonc">{
  "info_plugins": [
    {
      "plugin": "weather",
      "args": {
        "location": "London"
      }
    }
  ],
  "modules": [
    "os",
    "kernel",
    "plugin:weather",
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
      <td><code>location</code></td>
      <td><code>string</code></td>
      <td>No</td>
      <td>City name, coordinates, or airport code. Auto-detects if omitted.</td>
    </tr>
    <tr>
      <td><code>format</code></td>
      <td><code>string</code></td>
      <td>No</td>
      <td>Custom wttr.in format string. Default: <code>%c+%t+%w+%h+%p</code></td>
    </tr>
  </tbody>
</table>

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
      <td>Weather fetched</td>
      <td><pre>☀ +15°C Clear<br>   Humidity: 60%<br>   Wind: ↑15 km/h<br>   Precipitation: 0%</pre></td>
    </tr>
    <tr>
      <td>Network error</td>
      <td><code> Weather: could not fetch</code></td>
    </tr>
  </tbody>
</table>

<h2>How It Works</h2>

<ol>
  <li>xfetch sends a JSON request with <code>kind: "info_provider"</code> and the configured <code>location</code>.</li>
  <li>The plugin calls <code>curl</code> to fetch weather from wttr.in.</li>
  <li>The plugin parses the CSV-like response (condition, temperature, wind, humidity, precipitation).</li>
  <li>The plugin returns a JSON response with the formatted lines.</li>
  <li>xfetch displays them under the <code>plugin:weather</code> module key.</li>
</ol>

<h2>Notes</h2>

<ul>
  <li>Requires <code>curl</code> to be installed and available in <code>PATH</code>.</li>
  <li>Uses the free <a href="https://wttr.in">wttr.in</a> API — no API key required.</li>
  <li>Network connectivity is required.</li>
  <li>If no location is specified, wttr.in auto-detects location by IP.</li>
</ul>
