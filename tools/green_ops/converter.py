# SPDX-FileCopyrightText: 2026 Parth Shailesh Thite
# SPDX-License-Identifier: MIT OR Apache-2.0

import csv
import re

from log_parser import parse_line


CSV_FIELDS = [
    "run_id", "source_kind", "source_line", "segment_id", "event",
    "rx_attempts", "rx_data_packets", "sync_beacons", "tx_successes",
    "local_rx_ticks", "scheduled_tx_ticks",
]

EVENT_COLUMNS = {
    "rx_data": {
        "attempts": "rx_attempts",
        "decoded": "rx_data_packets",
        "local_rx_ticks": "local_rx_ticks",
    },
    "sync_beacon": {
        "count": "sync_beacons",
    },
    "tx_success": {
        "count": "tx_successes",
        "scheduled_tx_ticks": "scheduled_tx_ticks",
    },
}

MAX_DIAGNOSTICS = 20
RUN_ID_PATTERN = re.compile(r"[A-Za-z0-9][A-Za-z0-9._-]{0,63}")


def convert_stream(lines, output, *, run_id, source_kind):
    """Write event CSV to output and return a summary dictionary."""
    if RUN_ID_PATTERN.fullmatch(run_id) is None:
        raise ValueError(
            "run_id must be 1-64 ASCII letters, numbers, dots, "
            "underscores, or hyphens, and must start with a letter or number"
        )
    if source_kind not in {"synthetic", "hardware"}:
        raise ValueError("source_kind must be synthetic or hardware")

    writer = csv.DictWriter(output, fieldnames=CSV_FIELDS)
    writer.writeheader()

    summary = {
        "run_id": run_id,
        "source_kind": source_kind,
        "parsed": 0,
        "ignored": 0,
        "rejected": 0,
        "counter_discontinuities": 0,
        "event_counts": {
            "rx_data": 0,
            "sync_beacon": 0,
            "tx_success": 0,
        },
        "diagnostics": [],
        "diagnostics_omitted": 0,
    }
    segment_id = 0
    last_counters = {}

    for source_line, line in enumerate(lines, start=1):
        try:
            event = parse_line(line)
        except ValueError as error:
            summary["rejected"] += 1
            if len(summary["diagnostics"]) < MAX_DIAGNOSTICS:
                summary["diagnostics"].append({
                    "source_line": source_line,
                    "error": str(error)[:300],
                })
            else:
                summary["diagnostics_omitted"] += 1
            continue

        if event is None:
            summary["ignored"] += 1
            continue

        event_name = event["event"]
        counters = {
            column: event[input_field]
            for input_field, column in EVENT_COLUMNS[event_name].items()
            if column in {
                "rx_attempts",
                "rx_data_packets",
                "sync_beacons",
                "tx_successes",
            }
        }

        if any(
            value < last_counters[column]
            for column, value in counters.items()
            if column in last_counters
        ):
            segment_id += 1
            summary["counter_discontinuities"] += 1
            last_counters.clear()

        last_counters.update(counters)

        row = {
            "run_id": run_id,
            "source_kind": source_kind,
            "source_line": source_line,
            "segment_id": segment_id,
            "event": event_name,
        }

        for input_field, column in EVENT_COLUMNS[event_name].items():
            row[column] = event[input_field]

        writer.writerow(row)
        summary["parsed"] += 1
        summary["event_counts"][event_name] += 1

    return summary
