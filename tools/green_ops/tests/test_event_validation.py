# SPDX-FileCopyrightText: 2026 Parth Shailesh Thite
# SPDX-License-Identifier: MIT OR Apache-2.0

from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from log_parser import parse_line


class EventValidationTests(unittest.TestCase):
    def test_reads_beacon(self):
        self.assertEqual(
            parse_line("GREEN_OPS sync_beacon count=7"),
            {"event": "sync_beacon", "count": 7},
        )

    def test_reads_transmit(self):
        self.assertEqual(
            parse_line("INFO GREEN_OPS tx_success count=2 scheduled_tx_ticks=42"),
            {"event": "tx_success", "count": 2, "scheduled_tx_ticks": 42},
        )

    def test_accepts_unsigned_boundaries(self):
        cases = [
            ("rx_data", "attempts=0 decoded=0 local_rx_ticks=0"),
            ("rx_data",
             "attempts=4294967295 decoded=4294967295 "
             "local_rx_ticks=18446744073709551615"),
            ("sync_beacon", "count=4294967295"),
            ("tx_success", "count=0 scheduled_tx_ticks=0"),
            ("tx_success",
             "count=4294967295 scheduled_tx_ticks=18446744073709551615"),
        ]
        for event, fields in cases:
            with self.subTest(event=event, fields=fields):
                self.assertEqual(
                    parse_line(f"GREEN_OPS {event} {fields}")["event"], event
                )

    def test_rejects_invalid_unsigned_values(self):
        cases = [
            "attempts=-1 decoded=0 local_rx_ticks=0",
            "attempts=4294967296 decoded=0 local_rx_ticks=0",
            "attempts=0 decoded=4294967296 local_rx_ticks=0",
            "attempts=0 decoded=0 local_rx_ticks=18446744073709551616",
            "attempts=+1 decoded=0 local_rx_ticks=0",
            "attempts=1_000 decoded=0 local_rx_ticks=0",
            "attempts=١ decoded=0 local_rx_ticks=0",
            "attempts=1.5 decoded=0 local_rx_ticks=0",
            "attempts= decoded=0 local_rx_ticks=0",
        ]
        for fields in cases:
            with self.subTest(fields=fields):
                with self.assertRaises(ValueError):
                    parse_line(f"GREEN_OPS rx_data {fields}")

    def test_rejects_malformed_records(self):
        cases = [
            "GREEN_OPS",
            "GREEN_OPS unknown count=1",
            "GREEN_OPS rx_data attempts=1 decoded=1",
            "GREEN_OPS rx_data attempts=1 decoded=1 local_rx_ticks=0 extra=2",
            "GREEN_OPS rx_data attempts=1 attempts=2 decoded=1 local_rx_ticks=0",
            "GREEN_OPS rx_data attempts=1 decoded=1 local_rx_ticks=0 trailing",
            "GREEN_OPS sync_beacon",
            "GREEN_OPS tx_success count=1",
        ]
        for line in cases:
            with self.subTest(line=line):
                with self.assertRaises(ValueError):
                    parse_line(line)

    def test_requires_exact_marker_token(self):
        self.assertIsNone(parse_line("NOT_GREEN_OPS rx_data attempts=1"))
        self.assertIsNone(parse_line("GREEN_OPS_EXTRA rx_data attempts=1"))


if __name__ == "__main__":
    unittest.main()
