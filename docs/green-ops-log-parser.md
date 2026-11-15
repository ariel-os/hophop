<!-- SPDX-FileCopyrightText: 2026 Parth Shailesh Thite -->
<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->

# Green-Ops saved-log parser

This laptop-only tool converts saved `GREEN_OPS` firmware logs into a validated event CSV and a JSON summary. It can process synthetic fixtures now and hardware logs later without changing their raw counter or timestamp values.

## Run

From the hophop repository:

```bash
python3 tools/green_ops/green_ops_log_to_csv.py \
  --input INPUT.log \
  --csv-output events.csv \
  --summary-output summary.json \
  --run-id UNIQUE_RUN_ID \
  --source-kind synthetic
```

Use `--source-kind hardware` only for logs captured from a physical device. One input file should represent one device and one logical run. Output files must not already exist. The tool refuses to overwrite them or use the input path as an output. Input must be valid UTF-8.

Run IDs must contain 1 to 64 ASCII characters. The first character
must be a letter or number; subsequent characters may also include
periods, underscores, and hyphens. This prevents control characters
and spreadsheet-formula prefixes from entering the CSV provenance.

The CLI exclusively reserves both output paths. If reservation,
encoding, conversion, or publication fails, it removes paths created
by that invocation. This cleanup covers caught errors; the two files
are not a filesystem transaction and abrupt termination may require
manual cleanup.

Exit codes:

- `0`: conversion completed without rejected telemetry records.
- `1`: file, encoding, argument, provenance, or output-safety error.
- `2`: output files were produced, but one or more telemetry records were rejected. Inspect `diagnostics` in the summary.

## Recognized events

- `rx_data`: cumulative receive attempts, cumulative decoded PDC packets, and the latest local receive timestamp.
- `sync_beacon`: cumulative recognized timestamp-beacon packets.
- `tx_success`: cumulative successful transmit API calls and their latest scheduled transmit timestamp.

Counters are unsigned 32-bit decimal values. Timestamps are unsigned 64-bit decimal values. Missing, duplicate, unknown, signed, non-ASCII, or out-of-range fields are rejected.

`rx_data` is counted after successful PDC decoding but before successful MAC parsing is established. `tx_success` means the transmit API returned success; it does not prove reception by another device. Scheduled transmit ticks are not an independently measured physical transmission time.

## CSV schema

Each parsed event becomes one row with run and source provenance, its original one-based source line, inferred segment identifier, event name, and only the fields present in that event. Fields absent from an event remain empty. They are never filled with zero or copied from an earlier event.

A decrease in any cumulative counter starts a new inferred segment and clears previous counter baselines. This identifies a discontinuity only; it does not prove a reboot. Equal counters and decreasing timestamps do not start segments. Rates or deltas must not be calculated across segments.

## Summary

The JSON summary contains parsed, ignored, and rejected line counts; event record counts; counter-discontinuity count; provenance; and up to 20 rejection diagnostics. Further rejections remain counted in `diagnostics_omitted`.

Event counts describe log records processed by the parser. They are not independent measurements of packets sent over the radio.

## Synthetic demonstration

The repository fixture is explicitly synthetic:

```bash
mkdir -p build/green-ops-demo

python3 tools/green_ops/green_ops_log_to_csv.py \
  --input tools/green_ops/fixtures/synthetic-demo.log \
  --csv-output build/green-ops-demo/events.csv \
  --summary-output build/green-ops-demo/summary.json \
  --run-id synthetic-demo-001 \
  --source-kind synthetic
```

The fixture is for software verification and demonstration. Its values are not hardware measurements and must not be used for energy, latency, packet-loss, or regulatory conclusions.

## Tests

```bash
python3 -m unittest discover \
  -s tools/green_ops/tests \
  -p 'test_*.py' -v
```

Tests cover event schemas and numeric boundaries, malformed records, source provenance, blank CSV fields, bounded diagnostics, counter discontinuities, output collision safety, malformed-record exit status, and invalid UTF-8.

## Current limitations

This increment does not collect live serial output, automate the radio workload, calculate RTT, sample fixed 100 ms windows, estimate physical power, or generate the planned 18-feature ML dataset. Hardware logs and measurement calibration require later access to the nRF9151 boards and PPK2.
