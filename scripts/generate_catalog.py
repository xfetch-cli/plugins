#!/usr/bin/env python3
"""Generate the machine-readable addon catalog at the repository root.

Every addon lives in ``plugins/<name>/`` and is either a native plugin with a
``Cargo.toml`` or a WebAssembly plugin with an ``xfetch-plugin.json`` manifest.
The catalog only carries discovery metadata; installation still reads the
plugin manifest, so this file cannot grant capabilities by itself.

Usage:
    python3 scripts/generate_catalog.py
    python3 scripts/generate_catalog.py --check

``--check`` exits non-zero when ``index.json`` differs from what the generator
would write, which makes stale catalogs easy to catch in CI.
"""

from __future__ import annotations

import argparse
import json
import sys
import tomllib
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parent.parent
PLUGINS_DIR = ROOT / "plugins"
CATALOG_PATH = ROOT / "index.json"

SCHEMA_VERSION = 1
CATALOG_VERSION = "1.0.2"
NATIVE_PACKAGE_PREFIX = "xfetch-plugin-"
LOGO_ANIMATION_PLUGINS = frozenset({"animate-logo", "banner"})
GUEST_RUNTIMES = frozenset({"core", "component"})


class CatalogError(Exception):
    """A plugin directory cannot be turned into a catalog entry."""


def _read_json(path: Path) -> dict[str, Any]:
    try:
        with path.open("r", encoding="utf-8") as handle:
            data = json.load(handle)
    except (OSError, json.JSONDecodeError) as err:
        raise CatalogError(f"cannot read {path.relative_to(ROOT)}: {err}") from err
    if not isinstance(data, dict):
        raise CatalogError(f"{path.relative_to(ROOT)} must contain a JSON object")
    return data


def _read_cargo_package(path: Path) -> dict[str, Any]:
    try:
        with path.open("rb") as handle:
            data = tomllib.load(handle)
    except (OSError, tomllib.TOMLDecodeError) as err:
        raise CatalogError(f"cannot read {path.relative_to(ROOT)}: {err}") from err
    package = data.get("package")
    if not isinstance(package, dict):
        raise CatalogError(f"{path.relative_to(ROOT)} has no [package] table")
    return package


def _required_string(mapping: dict[str, Any], key: str, source: Path) -> str:
    value = mapping.get(key)
    if not isinstance(value, str) or not value.strip():
        raise CatalogError(
            f"{source.relative_to(ROOT)} is missing a non-empty '{key}'"
        )
    return value.strip()


def _wasm_entry(directory: Path) -> dict[str, Any]:
    manifest_path = directory / "xfetch-plugin.json"
    manifest = _read_json(manifest_path)

    name = _required_string(manifest, "name", manifest_path)
    if name != directory.name:
        raise CatalogError(
            f"{manifest_path.relative_to(ROOT)} names '{name}' but lives in "
            f"'{directory.name}'"
        )

    kind = _required_string(manifest, "kind", manifest_path)
    version = _required_string(manifest, "version", manifest_path)
    description = _required_string(manifest, "description", manifest_path)

    capabilities = manifest.get("capabilities")
    if not isinstance(capabilities, dict):
        raise CatalogError(
            f"{manifest_path.relative_to(ROOT)} is missing a 'capabilities' object"
        )

    guest_runtime = manifest.get("runtime")
    if guest_runtime not in GUEST_RUNTIMES:
        raise CatalogError(
            f"{manifest_path.relative_to(ROOT)} has unsupported runtime "
            f"{guest_runtime!r}; expected 'core' or 'component'"
        )

    return {
        "name": name,
        "description": description,
        "version": version,
        "kind": kind,
        "runtime": "wasm",
        "guest_runtime": guest_runtime,
        "path": f"plugins/{name}",
        "install": name,
        "capabilities": capabilities,
    }


def _native_entry(directory: Path) -> dict[str, Any]:
    cargo_path = directory / "Cargo.toml"
    package = _read_cargo_package(cargo_path)

    package_name = _required_string(package, "name", cargo_path)
    expected_name = f"{NATIVE_PACKAGE_PREFIX}{directory.name}"
    if package_name != expected_name:
        raise CatalogError(
            f"{cargo_path.relative_to(ROOT)} package name '{package_name}' does "
            f"not match directory '{directory.name}' (expected '{expected_name}')"
        )

    version = _required_string(package, "version", cargo_path)
    description = _required_string(package, "description", cargo_path)
    kind = "logo_animation" if directory.name in LOGO_ANIMATION_PLUGINS else "info_provider"

    return {
        "name": directory.name,
        "description": description,
        "version": version,
        "kind": kind,
        "runtime": "native",
        "path": f"plugins/{directory.name}",
        "install": directory.name,
    }


def _is_native_plugin(directory: Path) -> bool:
    cargo_path = directory / "Cargo.toml"
    if not cargo_path.is_file():
        return False
    try:
        with cargo_path.open("rb") as handle:
            package = tomllib.load(handle).get("package")
    except (OSError, tomllib.TOMLDecodeError):
        return False
    if not isinstance(package, dict):
        return False
    name = package.get("name")
    return isinstance(name, str) and name.startswith(NATIVE_PACKAGE_PREFIX)


def collect_entries() -> list[dict[str, Any]]:
    """Return catalog entries sorted by addon name."""
    if not PLUGINS_DIR.is_dir():
        raise CatalogError("plugins/ directory not found")

    entries: list[dict[str, Any]] = []
    for directory in sorted(PLUGINS_DIR.iterdir(), key=lambda path: path.name):
        if not directory.is_dir() or directory.name.startswith("."):
            continue
        if (directory / "xfetch-plugin.json").is_file():
            entries.append(_wasm_entry(directory))
        elif _is_native_plugin(directory):
            entries.append(_native_entry(directory))

    entries.sort(key=lambda entry: entry["name"])
    names = [entry["name"] for entry in entries]
    duplicates = sorted({name for name in names if names.count(name) > 1})
    if duplicates:
        raise CatalogError(f"duplicate catalog entries: {', '.join(duplicates)}")
    if not entries:
        raise CatalogError("no addons found under plugins/")
    return entries


def render_catalog(entries: list[dict[str, Any]]) -> str:
    catalog = {
        "schema_version": SCHEMA_VERSION,
        "catalog_version": CATALOG_VERSION,
        "entries": entries,
    }
    return json.dumps(catalog, indent=2, ensure_ascii=False) + "\n"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description="Generate or verify the repository addon catalog."
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="fail if index.json is missing or out of date instead of writing it",
    )
    args = parser.parse_args(argv)

    try:
        entries = collect_entries()
    except CatalogError as err:
        print(f"error: {err}", file=sys.stderr)
        return 1

    rendered = render_catalog(entries)

    if args.check:
        try:
            current = CATALOG_PATH.read_text(encoding="utf-8")
        except OSError as err:
            print(f"error: cannot read index.json: {err}", file=sys.stderr)
            return 1
        if current != rendered:
            print(
                "error: index.json is stale; run "
                "python3 scripts/generate_catalog.py",
                file=sys.stderr,
            )
            return 1
        print(f"index.json is up to date ({len(entries)} entries).")
        return 0

    try:
        CATALOG_PATH.write_text(rendered, encoding="utf-8")
    except OSError as err:
        print(f"error: cannot write index.json: {err}", file=sys.stderr)
        return 1
    print(f"Wrote index.json ({len(entries)} entries).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
