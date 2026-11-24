# SPDX-FileCopyrightText: 2026 Parth Shailesh Thite
# SPDX-License-Identifier: MIT OR Apache-2.0

U32_MAX = (1 << 32) - 1
U64_MAX = (1 << 64) - 1

EVENT_FIELDS = {
    "rx_data": {
        "attempts": U32_MAX,
        "decoded": U32_MAX,
        "local_rx_ticks": U64_MAX,
    },
    "sync_beacon": {
        "count": U32_MAX,
    },
    "tx_success": {
        "count": U32_MAX,
        "scheduled_tx_ticks": U64_MAX,
    },
}


def parse_line(line: str) -> dict[str, int | str] | None:
    """Return a validated event, None for unrelated text, or raise ValueError."""
    tokens = line.split()

    if "GREEN_OPS" not in tokens:
        return None

    payload = tokens[tokens.index("GREEN_OPS") + 1:]
    if not payload:
        raise ValueError("Missing telemetry event")

    event = payload[0]
    if event not in EVENT_FIELDS:
        raise ValueError(f"Unknown telemetry event: {event}")

    schema = EVENT_FIELDS[event]
    fields = {}

    for token in payload[1:]:
        key, separator, value = token.partition("=")

        if not separator or key not in schema:
            raise ValueError(f"Invalid or unknown field: {token}")
        if key in fields:
            raise ValueError(f"Duplicate field: {key}")
        if not value or not value.isascii() or not value.isdecimal():
            raise ValueError(f"Expected unsigned ASCII decimal for {key}")

        number = int(value)
        if number > schema[key]:
            raise ValueError(f"Value exceeds allowed range for {key}")

        fields[key] = number

    missing = set(schema) - set(fields)
    if missing:
        raise ValueError(f"Missing fields: {', '.join(sorted(missing))}")

    return {"event": event, **fields}
