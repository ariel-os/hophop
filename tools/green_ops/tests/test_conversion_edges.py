# SPDX-FileCopyrightText: 2026 Parth Shailesh Thite
# SPDX-License-Identifier: MIT OR Apache-2.0

import csv
import io
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from converter import convert_stream


class ConversionEdgeTests(unittest.TestCase):
    def convert(self, lines):
        output = io.StringIO()
        summary = convert_stream(
            lines, output, run_id="edge-test", source_kind="synthetic"
        )
        rows = list(csv.DictReader(io.StringIO(output.getvalue())))
        return rows, summary

    def test_counter_decrease_starts_segment_and_clears_old_baselines(self):
        rows, summary = self.convert([
            "GREEN_OPS tx_success count=10 scheduled_tx_ticks=100",
            "GREEN_OPS rx_data attempts=9 decoded=8 local_rx_ticks=100",
            "GREEN_OPS rx_data attempts=1 decoded=1 local_rx_ticks=10",
            "GREEN_OPS tx_success count=1 scheduled_tx_ticks=10",
        ])
        self.assertEqual(
            [row["segment_id"] for row in rows], ["0", "0", "1", "1"]
        )
        self.assertEqual(summary["counter_discontinuities"], 1)
        self.assertEqual(summary["parsed"], 4)

    def test_equal_counts_and_lower_timestamps_do_not_start_segment(self):
        rows, summary = self.convert([
            "GREEN_OPS tx_success count=2 scheduled_tx_ticks=100",
            "GREEN_OPS tx_success count=2 scheduled_tx_ticks=100",
            "GREEN_OPS tx_success count=3 scheduled_tx_ticks=0",
        ])
        self.assertEqual(
            [row["segment_id"] for row in rows], ["0", "0", "0"]
        )
        self.assertEqual(summary["counter_discontinuities"], 0)

    def test_each_counter_can_trigger_discontinuity(self):
        pairs = [
            ("rx_data attempts=5 decoded=2 local_rx_ticks=0",
             "rx_data attempts=4 decoded=2 local_rx_ticks=1"),
            ("rx_data attempts=5 decoded=2 local_rx_ticks=0",
             "rx_data attempts=6 decoded=1 local_rx_ticks=1"),
            ("sync_beacon count=5", "sync_beacon count=4"),
            ("tx_success count=5 scheduled_tx_ticks=0",
             "tx_success count=4 scheduled_tx_ticks=1"),
        ]
        for before, after in pairs:
            with self.subTest(before=before):
                rows, summary = self.convert([
                    f"GREEN_OPS {before}", f"GREEN_OPS {after}"
                ])
                self.assertEqual(rows[1]["segment_id"], "1")
                self.assertEqual(summary["counter_discontinuities"], 1)

    def test_invalid_provenance_is_rejected_before_writing(self):
        cases = [
            ("", "synthetic"),
            ("   ", "synthetic"),
            ("demo", "unknown"),
            ("demo", ""),
        ]
        for run_id, source_kind in cases:
            with self.subTest(run_id=run_id, source_kind=source_kind):
                output = io.StringIO()
                with self.assertRaises(ValueError):
                    convert_stream(
                        [], output, run_id=run_id, source_kind=source_kind
                    )
                self.assertEqual(output.getvalue(), "")

    def test_rejects_unsafe_run_ids_before_writing(self):
        unsafe_run_ids = [
            "=formula",
            "+formula",
            "-formula",
            "@formula",
            "line\nbreak",
            "a" * 65,
        ]
        for run_id in unsafe_run_ids:
            with self.subTest(run_id=run_id):
                output = io.StringIO()
                with self.assertRaises(ValueError):
                    convert_stream(
                        [], output,
                        run_id=run_id,
                        source_kind="synthetic",
                    )
                self.assertEqual(output.getvalue(), "")

    def test_hardware_provenance_is_preserved(self):
        output = io.StringIO()
        summary = convert_stream(
            ["GREEN_OPS sync_beacon count=1"],
            output, run_id="lab-001", source_kind="hardware",
        )
        rows = list(csv.DictReader(io.StringIO(output.getvalue())))
        self.assertEqual(rows[0]["source_kind"], "hardware")
        self.assertEqual(summary["source_kind"], "hardware")

    def test_diagnostics_are_bounded_but_all_rejections_counted(self):
        rows, summary = self.convert(["GREEN_OPS unknown"] * 25)
        self.assertEqual(rows, [])
        self.assertEqual(summary["rejected"], 25)
        self.assertEqual(len(summary["diagnostics"]), 20)
        self.assertEqual(summary["diagnostics_omitted"], 5)


if __name__ == "__main__":
    unittest.main()
