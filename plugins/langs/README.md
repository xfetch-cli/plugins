# xfetch-plugin-langs

Shows the languages of a GitHub repository (or aggregated across a user/org)
for [xfetch](https://github.com/xfetch-cli/xfetch), with a Nerd Font glyph per
language and, optionally, its brand colour.

## Usage

```jsonc
{
    "info_plugins": [
        { "plugin": "langs", "args": { "repo": "xfetch-cli/xfetch" } }
    ],
    "modules": ["plugin:langs"]
}
```

Or aggregate the primary language of every repository of a user/org:

```jsonc
{
    "info_plugins": [
        { "plugin": "langs", "args": { "owner": "xfetch-cli", "limit": 4 } }
    ],
    "modules": ["plugin:langs"]
}
```

## Args

| Field | Type | Default | Description |
|---|---|---|---|
| `repo` | string | — | Repository as `owner/name` (also accepts a full GitHub URL). Shows the exact byte breakdown from `/languages`. |
| `owner` | string | — | User or org. Aggregates the primary language of its repositories (one API call). |
| `limit` | number | `5` | How many languages to show. |
| `color` | boolean | `true` | Tint each language with its brand colour (truecolor ANSI). |
| `include_forks` | boolean | `false` | Include forks when aggregating an `owner`. |
| `info` | boolean | `true` | Also print repository/owner metadata (see below). |
| `token` | string | — | GitHub token; also read from `GITHUB_TOKEN` / `GH_TOKEN`. Raises the API limit from 60 to 5000 req/h. |

## What it shows

Repository mode prints metadata first, then the language breakdown:

- `Description`, `Stars`, `Forks`, `Open issues`, `License`, `Size`,
  `Branch`, `Updated`, `Topics`, `Homepage`, plus `Archived` / `Private` when set.
- Languages with their byte share (` Rust 89.3%`).

Owner mode prints a short summary (name, bio, public repos, followers) and then
the primary language of each repository (` Rust ×5 (50%)`).

> Without a token GitHub allows 60 requests/hour, and each run does 2 calls;
> set `GITHUB_TOKEN`/`GH_TOKEN` (or the `token` arg) for regular use.

## Output

```
 Rust 96.4%
 Python 2.1%
 Shell 1.1%

 Rust ×12 (60%)
 Python ×3 (15%)
 Shell ×2 (10%)
```

Each line starts with a Nerd Font glyph, which the core lifts as the row icon;
the rest is the language name (brand-tinted) and its share.

If a layout measures widths and the tint shifts the alignment, set
`"color": false`.
