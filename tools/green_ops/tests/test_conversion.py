# SPDX-FileCopyrightText: 2026 Parth Shailesh Thite
# SPDX-License-Identifier: MIT OR Apache-2.0

import csv
import io
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from converter import convert_stream


class ConversionTests(unittest.TestCase):
    def test_preserves_provenance_and_leaves_absent_fields_empty(self):
        lines = [
            "INFO boot complete",
            "GREEN_OPS rx_data attempts=3 decoded=2 local_rx_ticks=123",
            "GREEN_OPS tx_success count=1 scheduled_tx_ticks=456",
        ]
        output = io.StringIO()
        summary = convert_stream(
            lines, output, run_id="demo-001", source_kind="synthetic"
        )
        rows = list(csv.DictReader(io.StringIO(output.getvalue())))

        self.assertEqual(len(rows), 2)
        self.assertEqual(rows[0], {
            "run_id": "demo-001",
            "source_kind": "synthetic",
            "source_line": "2",
            "segment_id": "0",
            "event": "rx_data",
            "rx_attempts": "3",
            "rx_data_packets": "2",
            "sync_beacons": "",
            "tx_successes": "",
            "local_rx_ticks": "123",
            "scheduled_tx_ticks": "",
        })
        self.assertEqual(rows[1]["source_line"], "3")
        self.assertEqual(rows[1]["tx_successes"], "1")
        self.assertEqual(rows[1]["scheduled_tx_ticks"], "456")
        self.assertEqual(rows[1]["local_rx_ticks"], "")
        self.assertEqual(rows[1]["rx_attempts"], "")
        self.assertEqual(summary["parsed"], 2)
        self.assertEqual(summary["ignored"], 1)
        self.assertEqual(summary["rejected"], 0)
        self.assertEqual(summary["event_counts"], {
            "rx_data": 1, "sync_beacon": 0, "tx_success": 1,
        })

    def test_reports_malformed_line_and_keeps_valid_records(self):
        output = io.StringIO()
        summary = convert_stream(
            [
                "GREEN_OPS rx_data attempts=-1 decoded=0 local_rx_ticks=0",
                "GREEN_OPS sync_beacon count=4",
            ],
            output, run_id="demo-002", source_kind="synthetic",
        )
        rows = list(csv.DictReader(io.StringIO(output.getvalue())))
        self.assertEqual(len(rows), 1)
        self.assertEqual(rows[0]["source_line"], "2")
        self.assertEqual(rows[0]["sync_beacons"], "4")
        self.assertEqual(summary["parsed"], 1)
        self.assertEqual(summary["rejected"], 1)
        self.assertEqual(summary["diagnostics"][0]["source_line"], 1)
        self.assertTrue(summary["diagnostics"][0]["error"])

    def test_empty_input_produces_header_and_zero_counts(self):
        output = io.StringIO()
        summary = convert_stream(
            [], output, run_id="empty", source_kind="synthetic"
        )
        reader = csv.DictReader(io.StringIO(output.getvalue()))
        self.assertIn("source_line", reader.fieldnames)
        self.assertEqual(list(reader), [])
        self.assertEqual(summary["parsed"], 0)
        self.assertEqual(summary["ignored"], 0)
        self.assertEqual(summary["rejected"], 0)


if __name__ == "__main__":
    unittest.main()
