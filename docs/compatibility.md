# Platform Compatibility

<p>
  Platform support per plugin. <code>✓</code> = works, <code>~</code> =
  works with caveats (see notes), <code>✗</code> = not supported (the plugin
  still responds gracefully with a fallback line).
</p>

<table>
  <thead>
    <tr>
      <th>Plugin</th>
      <th>Linux</th>
      <th>macOS</th>
      <th>Windows</th>
      <th>Notes</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>animate-logo</code></td>
      <td>✓</td><td>✓</td><td>✓</td>
      <td>Pure computation, no OS dependencies.</td>
    </tr>
    <tr>
      <td><code>chocolatey</code></td>
      <td>✗</td><td>✗</td><td>✓</td>
      <td>Requires the <code>choco</code> CLI, which only exists on Windows; elsewhere it responds with a fallback line.</td>
    </tr>
    <tr>
      <td><code>display-resolution</code></td>
      <td>✓</td><td>✓</td><td>✓</td>
      <td>X11/Wayland/xrandr on Linux, native on Windows and macOS.</td>
    </tr>
    <tr>
      <td><code>docker</code></td>
      <td>✓</td><td>✓</td><td>✓</td>
      <td>Requires the Docker CLI (Docker Desktop on Windows/macOS).</td>
    </tr>
    <tr>
      <td><code>github-stats</code></td>
      <td>✓</td><td>✓</td><td>✓</td>
      <td>Requires <code>curl</code> and network access.</td>
    </tr>
    <tr>
      <td><code>music-player</code></td>
      <td>✓</td><td>~</td><td>✗</td>
      <td>Uses <code>mpc</code>/<code>playerctl</code> (MPRIS/D-Bus). On Windows there is no equivalent CLI.</td>
    </tr>
    <tr>
      <td><code>temperature</code></td>
      <td>✓</td><td>✗</td><td>~</td>
      <td>Linux: hwmon thermal zones. Windows: ACPI via wmic/PowerShell — depends on the hardware exposing a thermal zone (desktops often don't).</td>
    </tr>
    <tr>
      <td><code>theme-detection</code></td>
      <td>✓</td><td>✗</td><td>✓</td>
      <td>Linux: GTK (gsettings) + KDE Plasma. Windows: registry (light/dark + accent).</td>
    </tr>
    <tr>
      <td><code>theme-manager</code></td>
      <td>✓</td><td>✓</td><td>✓</td>
      <td>Requires <code>curl</code> and network access for remote registries.</td>
    </tr>
    <tr>
      <td><code>timezone</code></td>
      <td>✓</td><td>✓</td><td>✓</td>
      <td>Windows uses PowerShell (<code>Get-TimeZone</code>); the <code>format</code> arg is Linux/macOS only.</td>
    </tr>
    <tr>
      <td><code>user-info</code></td>
      <td>✓</td><td>✓</td><td>✓</td>
      <td>Uses <code>whoami</code>/<code>groups</code> (Windows has both).</td>
    </tr>
    <tr>
      <td><code>weather</code></td>
      <td>✓</td><td>✓</td><td>✓</td>
      <td>Requires <code>curl</code> and network access.</td>
    </tr>
  </tbody>
</table>

<h2>Legend</h2>

<ul>
  <li><code>✓</code> — works on the platform.</li>
  <li><code>~</code> — works with caveats (extra dependencies or hardware requirements).</li>
  <li><code>✗</code> — not supported; the plugin responds with a graceful fallback line.</li>
</ul>
