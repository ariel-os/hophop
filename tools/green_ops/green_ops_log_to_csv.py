# SPDX-FileCopyrightText: 2026 Parth Shailesh Thite
# SPDX-License-Identifier: MIT OR Apache-2.0

import argparse
import json
import os
from pathlib import Path
import sys
import tempfile

from converter import convert_stream


class ArgumentParser(argparse.ArgumentParser):
    def error(self, message):
        self.print_usage(sys.stderr)
        self.exit(1, f"{self.prog}: error: {message}\n")


def parse_arguments():
    parser = ArgumentParser(
        description="Convert GREEN_OPS log records into CSV and JSON summary files."
    )
    parser.add_argument("--input", required=True, type=Path)
    parser.add_argument("--csv-output", required=True, type=Path)
    parser.add_argument("--summary-output", required=True, type=Path)
    parser.add_argument("--run-id", required=True)
    parser.add_argument(
        "--source-kind", required=True, choices=("synthetic", "hardware")
    )
    return parser.parse_args()


def resolved_path(path):
    return path.expanduser().resolve()


def check_paths(input_path, csv_path, summary_path):
    if not input_path.is_file():
        raise ValueError(f"Input log is not a file: {input_path}")
    if csv_path == summary_path:
        raise ValueError("CSV and summary output paths must differ")
    if input_path in {csv_path, summary_path}:
        raise ValueError("Input path must differ from output paths")
    if csv_path.exists() or summary_path.exists():
        raise ValueError("Refusing to overwrite an existing output file")
    if not csv_path.parent.is_dir() or not summary_path.parent.is_dir():
        raise ValueError("Output parent directories must already exist")


def write_outputs(lines, csv_path, summary_path, run_id, source_kind):
    csv_temp = None
    summary_temp = None
    reserved_paths = []
    completed = False

    try:
        for output_path in (csv_path, summary_path):
            with output_path.open("x", encoding="utf-8"):
                pass
            reserved_paths.append(output_path)

        with tempfile.NamedTemporaryFile(
            mode="w", encoding="utf-8", newline="",
            dir=csv_path.parent, delete=False
        ) as csv_file:
            csv_temp = Path(csv_file.name)
            summary = convert_stream(
                lines, csv_file, run_id=run_id, source_kind=source_kind
            )

        with tempfile.NamedTemporaryFile(
            mode="w", encoding="utf-8",
            dir=summary_path.parent, delete=False
        ) as summary_file:
            summary_temp = Path(summary_file.name)
            json.dump(summary, summary_file, indent=2, sort_keys=True)
            summary_file.write("\n")

        os.replace(csv_temp, csv_path)
        os.replace(summary_temp, summary_path)
        completed = True
        return summary
    finally:
        for temporary in (csv_temp, summary_temp):
            if temporary is not None and temporary.exists():
                temporary.unlink()

        if not completed:
            for output_path in reserved_paths:
                output_path.unlink(missing_ok=True)


def main():
    arguments = parse_arguments()
    input_path = resolved_path(arguments.input)
    csv_path = resolved_path(arguments.csv_output)
    summary_path = resolved_path(arguments.summary_output)

    try:
        check_paths(input_path, csv_path, summary_path)
        with input_path.open(encoding="utf-8", errors="strict") as input_file:
            summary = write_outputs(
                input_file, csv_path, summary_path,
                arguments.run_id, arguments.source_kind,
            )
    except (OSError, UnicodeDecodeError, ValueError) as error:
        print(f"error: {error}", file=__import__("sys").stderr)
        return 1

    print(
        f"Converted {summary['parsed']} event records; "
        f"ignored {summary['ignored']}; rejected {summary['rejected']}."
    )
    return 2 if summary["rejected"] else 0


if __name__ == "__main__":
    raise SystemExit(main())
