# Security Policy

## Reporting Security Vulnerabilities

If you discover a security vulnerability in the **plugin ecosystem** (e.g. a plugin that
executes untrusted commands, misparses output, or leaks data), please report it responsibly
by contacting:

**Email:** `x@xscriptor.com`

### What to Include

When reporting a security issue, please provide:

1. **Description** — A clear explanation of the vulnerability
2. **Type** — What kind of security issue is it? (e.g., command injection, output parsing, data leak, supply-chain)
3. **Steps to Reproduce** — Detailed steps to trigger the vulnerability
4. **Impact** — How severe is the issue? What could an attacker do?
5. **Affected Versions** — Which plugin and xfetch versions are affected?
6. **Proposed Fix** (optional) — If you have a suggestion for how to fix it

### Guidelines

- **Do not** open public GitHub issues for security vulnerabilities
- **Do not** disclose the vulnerability publicly until a fix is released
- **Do** give the maintainers reasonable time to address the issue before public disclosure
- Typically, we aim to respond within **7 days** and release a fix within **30 days** for critical issues

## Scope

Plugins are user-installed binaries (`xfetch plugin install`) that xfetch spawns to enrich the
fetch output. Anything that lets a plugin escape its intended role is in scope:

- Command execution: plugins run subprocesses and parse their output — injection paths, unsafe
  shell usage, or environment-dependent behavior that could turn into code execution.
- The `xfetch-plugin-api` wire protocol: malformed requests/responses, JSON parsing of untrusted
  output, missing timeouts (a plugin must never hang or crash xfetch — enforced via
  `with_timeout`).
- Supply chain: the build/install path (`cargo`, the api dependency) and any attempt to ship a
  binary that does more than it documents.
- Data handling: plugins must not read or exfiltrate data beyond what their README documents.

Report anything that breaks these guarantees regardless of severity.
