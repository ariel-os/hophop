# SPDX-FileCopyrightText: 2026 Parth Shailesh Thite
# SPDX-License-Identifier: MIT OR Apache-2.0

from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from log_parser import parse_line


class ParseLineTests(unittest.TestCase):
    def test_ignores_unrelated_firmware_message(self):
        self.assertIsNone(parse_line("INFO radio initialized"))

    def test_reads_prefixed_receive_event_as_typed_fields(self):
        line = (
            "INFO GREEN_OPS rx_data "
            "attempts=3 decoded=2 local_rx_ticks=123456"
        )
        self.assertEqual(parse_line(line), {
            "event": "rx_data",
            "attempts": 3,
            "decoded": 2,
            "local_rx_ticks": 123456,
        })


if __name__ == "__main__":
    unittest.main()
