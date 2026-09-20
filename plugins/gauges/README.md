# xfetch-plugin-gauges

Memory, swap, disk and battery usage rendered as text progress bars for
[xfetch](https://github.com/xfetch-cli/xfetch).

Every line keeps the native measurement and adds a bar next to it, so a
half-full disk reads like `[####......] 476.00 GiB / 952.87 GiB (50%)`.

## Usage

Install the plugin binary in the xfetch plugin dir, then add it to your config:

```jsonc
{
    "info_plugins": [
        {
            "plugin": "gauges",
            "args": {
                "style": "auto",
                "width": 10,
                "metrics": ["memory", "swap", "disk", "battery"]
            }
        }
    ],
    "modules": ["plugin:gauges"],
    "icons": { "plugin:gauges": "" }
}
```

## Args

| Field | Type | Default | Description |
|---|---|---|---|
| `style` | string | `"auto"` | Bar style: `auto` (slashes under 50%, hashes from 50%), `slashes` (`[////___]`), `hash` (`[###....]`) or `blocks` (`[███░░░]`). |
| `width` | number | `10` | Bar width in characters (clamped to 3–40). |
| `metrics` | array | all | Subset of `memory`, `swap`, `disk`, `battery`. |
| `include_tmpfs` | boolean | `false` | Also list tmpfs/ramdisk mounts in the disk section. |
| `glyphs` | string | `"nerd"` | Icon set: `nerd` (Nerd Font), `cjk` (存 换 盘 电) or `jp` (存 換 盤 電). |

## Example output

```
Mem  [////______] 10.96 GiB / 93.89 GiB (12%)
Swap [__________] 0.00 GiB / 4.00 GiB (0%)
Disk [###.......] 35.09 GiB / 952.87 GiB (4%) btrfs /
Disk [//////____] 7.20 GiB / 16.00 GiB (45%) ext4 /home
Batt [##########] 100% [Full] BAT0
```

## Platform support

| Platform | Source |
|---|---|
| Linux | `sysinfo` for memory/swap/disks, `/sys/class/power_supply` for battery. |
| macOS / Windows | Memory, swap and disks via `sysinfo`; battery lines are omitted. |
