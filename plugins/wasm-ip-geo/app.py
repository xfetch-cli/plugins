"""wasm-ip-geo: public IP information through the sandboxed HTTP host call.

Compiled to a WebAssembly component with `componentize-py`. The manifest
allowlists only `https://ipapi.co/*`, so the guest cannot call any other
origin. Failures degrade to an explanatory line instead of breaking the
fetch.
"""

import json

from wit_world.imports import host
from wit_world.imports.types import HttpRequest

API_URL = "https://ipapi.co/json/"


class WitWorld:
    """Info provider: one JSON request in, one JSON response out."""

    def run(self, request: str) -> str:
        payload = json.loads(request)
        args = payload.get("args") or {}
        fields = args.get("fields") or ["ip", "location", "org", "timezone"]

        host.log("info", f"wasm-ip-geo: querying {API_URL}")
        response = host.fetch(
            HttpRequest(
                method="GET",
                url=API_URL,
                headers=[("Accept", "application/json")],
                body=None,
                timeout_ms=6000,
            )
        )

        if response.status != 200:
            return _json_lines([f"ip: HTTP {response.status} from ipapi.co"])

        try:
            data = json.loads(response.body)
        except (UnicodeDecodeError, json.JSONDecodeError):
            return _json_lines(["ip: malformed response from ipapi.co"])

        lines = []
        if data.get("error"):
            return _json_lines([f"ip: {data.get('reason', 'lookup failed')}"])

        if "ip" in fields and data.get("ip"):
            lines.append(f"ip: {data['ip']} ({data.get('version', 'v4')})")

        if "location" in fields:
            city = data.get("city") or "?"
            region = data.get("region_code") or data.get("region") or "?"
            country = data.get("country_name") or data.get("country_code") or "?"
            lines.append(f"location: {city}, {region} - {country}")

        if "org" in fields and data.get("org"):
            lines.append(f"network: {data['org']}")

        if "timezone" in fields and data.get("timezone"):
            utc_offset = data.get("utc_offset") or "?"
            lines.append(f"timezone: {data['timezone']} (UTC{utc_offset})")

        if not lines:
            lines.append("ip: no data")

        return _json_lines(lines)


def _json_lines(lines):
    return json.dumps({"lines": lines})
