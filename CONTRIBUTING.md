<h1>Contributing Plugins</h1>

<p>
  Thanks for contributing to the <strong>xfetch</strong> plugin ecosystem.
  This repository contains official plugins and the reference documentation for
  building new ones.
</p>

<h2>Workflow</h2>

<ol>
  <li>Fork the repository and create a feature branch.</li>
  <li>Create or update a plugin directory at the repository root.</li>
  <li>Run <code>cargo test --workspace</code>.</li>
  <li>Document the plugin in its own <code>README.md</code> and in <a href="./README.md">README.md</a>.</li>
  <li>Open a pull request with usage details and any required external dependencies.</li>
</ol>

<h2>Plugin Rules</h2>

<ul>
  <li>Use the binary naming convention <code>xfetch-plugin-&lt;name&gt;</code>.</li>
  <li>Keep plugins focused on a single responsibility.</li>
  <li>Write errors to stderr and exit with a non-zero status on failure.</li>
  <li>Prefer stable, actively maintained dependencies and keep them minimal.</li>
</ul>

<h2>Protocol Guide</h2>

<p>
  The full protocol, discovery order, and testing workflow are documented in
  <a href="./docs/README.md">docs/README.md</a>.
</p>
