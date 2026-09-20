# xfetch-plugin-banner

Logo-animation plugin for [xfetch](https://github.com/xfetch-cli/xfetch) that
prints a short Chinese/Japanese proverb banner under (or above) the ASCII logo,
without modifying the logo file.

## Usage

```jsonc
{
    "logo_animation": {
        "plugin": "banner",
        "style": "banner",
        "timeout_secs": 3
    }
}
```

## Styles

| `style` | Behaviour |
|---|---|
| `banner` | Random proverb, below the logo. |
| `banner-above` | Random proverb, above the logo. |
| `banner-static` | Always the first proverb, below. |
| `banner-N` | The N-th proverb (1-based), below. |

Set `frames_path` to a text file to use its first `===`-separated frame as the
banner text instead of the built-in list.

The banner is centred to the logo width, counting CJK characters as two
columns. Requires a terminal with a CJK font (see the repo notes) and a TTY.

## Proverb list

A mix of Chinese and Japanese sayings (`千里之行，始于足下`, `七転び八起き`,
`一期一会`, ...). The plugin picks one per run.
