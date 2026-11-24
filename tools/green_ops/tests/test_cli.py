# SPDX-FileCopyrightText: 2026 Parth Shailesh Thite
# SPDX-License-Identifier: MIT OR Apache-2.0

import csv
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import green_ops_log_to_csv as cli

SCRIPT = Path(__file__).resolve().parents[1] / "green_ops_log_to_csv.py"


class CliTests(unittest.TestCase):
    def run_cli(self, *arguments):
        return subprocess.run(
            [sys.executable, str(SCRIPT), *arguments],
            text=True,
            capture_output=True,
        )

    def test_argument_errors_return_one_without_outputs(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            input_log = directory / "input.log"
            output_csv = directory / "events.csv"
            summary_json = directory / "summary.json"
            input_log.write_text("INFO boot\n", encoding="utf-8")

            result = self.run_cli(
                "--input", str(input_log),
                "--csv-output", str(output_csv),
                "--summary-output", str(summary_json),
                "--run-id", "demo-argument-error",
                "--source-kind", "invalid",
            )

            self.assertEqual(result.returncode, 1)
            self.assertFalse(output_csv.exists())
            self.assertFalse(summary_json.exists())

    def test_write_outputs_refuses_late_output_collision(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            output_csv = directory / "events.csv"
            summary_json = directory / "summary.json"
            summary_json.write_text("KEEP", encoding="utf-8")

            with self.assertRaises(FileExistsError):
                cli.write_outputs(
                    [],
                    output_csv,
                    summary_json,
                    "race-test",
                    "synthetic",
                )

            self.assertFalse(output_csv.exists())
            self.assertEqual(
                summary_json.read_text(encoding="utf-8"),
                "KEEP",
            )

    def test_write_outputs_rolls_back_if_second_publish_fails(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            output_csv = directory / "events.csv"
            summary_json = directory / "summary.json"
            real_replace = cli.os.replace

            def fail_summary_publish(source, destination):
                if Path(destination) == summary_json:
                    raise OSError("simulated summary publication failure")
                real_replace(source, destination)

            with mock.patch.object(
                cli.os,
                "replace",
                side_effect=fail_summary_publish,
            ):
                with self.assertRaises(OSError):
                    cli.write_outputs(
                        [],
                        output_csv,
                        summary_json,
                        "rollback-test",
                        "synthetic",
                    )

            self.assertFalse(output_csv.exists())
            self.assertFalse(summary_json.exists())

    def test_writes_csv_and_summary_for_synthetic_fixture(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            input_log = directory / "input.log"
            output_csv = directory / "events.csv"
            summary_json = directory / "summary.json"
            input_log.write_text(
                "INFO boot\n"
                "GREEN_OPS rx_data attempts=1 decoded=1 local_rx_ticks=10\n"
                "GREEN_OPS sync_beacon count=1\n",
                encoding="utf-8",
            )

            result = self.run_cli(
                "--input", str(input_log),
                "--csv-output", str(output_csv),
                "--summary-output", str(summary_json),
                "--run-id", "demo-001",
                "--source-kind", "synthetic",
            )

            self.assertEqual(result.returncode, 0, result.stderr)
            with output_csv.open(encoding="utf-8") as csv_file:
                rows = list(csv.DictReader(csv_file))
            summary = json.loads(summary_json.read_text(encoding="utf-8"))
            self.assertEqual(len(rows), 2)
            self.assertEqual(rows[0]["run_id"], "demo-001")
            self.assertEqual(rows[0]["source_kind"], "synthetic")
            self.assertEqual(summary["parsed"], 2)
            self.assertEqual(summary["rejected"], 0)

    def test_rejections_produce_outputs_and_nonzero_exit(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            input_log = directory / "input.log"
            output_csv = directory / "events.csv"
            summary_json = directory / "summary.json"
            input_log.write_text(
                "GREEN_OPS unknown\nGREEN_OPS tx_success count=1 scheduled_tx_ticks=9\n",
                encoding="utf-8",
            )

            result = self.run_cli(
                "--input", str(input_log),
                "--csv-output", str(output_csv),
                "--summary-output", str(summary_json),
                "--run-id", "demo-002",
                "--source-kind", "synthetic",
            )

            self.assertEqual(result.returncode, 2)
            self.assertTrue(output_csv.exists())
            summary = json.loads(summary_json.read_text(encoding="utf-8"))
            self.assertEqual(summary["parsed"], 1)
            self.assertEqual(summary["rejected"], 1)

    def test_refuses_existing_or_colliding_paths_without_overwriting(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            input_log = directory / "input.log"
            input_log.write_text("INFO boot\n", encoding="utf-8")
            existing_csv = directory / "events.csv"
            existing_csv.write_text("KEEP", encoding="utf-8")
            summary_json = directory / "summary.json"

            result = self.run_cli(
                "--input", str(input_log),
                "--csv-output", str(existing_csv),
                "--summary-output", str(summary_json),
                "--run-id", "demo-003",
                "--source-kind", "synthetic",
            )
            self.assertNotEqual(result.returncode, 0)
            self.assertEqual(existing_csv.read_text(encoding="utf-8"), "KEEP")
            self.assertFalse(summary_json.exists())

            result = self.run_cli(
                "--input", str(input_log),
                "--csv-output", str(input_log),
                "--summary-output", str(summary_json),
                "--run-id", "demo-003",
                "--source-kind", "synthetic",
            )
            self.assertNotEqual(result.returncode, 0)
            self.assertEqual(input_log.read_text(encoding="utf-8"), "INFO boot\n")

    def test_invalid_utf8_fails_without_creating_outputs(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            input_log = directory / "input.log"
            output_csv = directory / "events.csv"
            summary_json = directory / "summary.json"
            input_log.write_bytes(b"\xff\n")

            result = self.run_cli(
                "--input", str(input_log),
                "--csv-output", str(output_csv),
                "--summary-output", str(summary_json),
                "--run-id", "demo-004",
                "--source-kind", "synthetic",
            )

            self.assertNotEqual(result.returncode, 0)
            self.assertFalse(output_csv.exists())
            self.assertFalse(summary_json.exists())


if __name__ == "__main__":
    unittest.main()
