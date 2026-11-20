<!-- SPDX-FileCopyrightText: 2026 Parth Shailesh Thite -->
<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->

# Green-Ops initial Ping telemetry

This optional instrumentation adds task-local cumulative packet counters and raw radio timestamps to hophop's Ping example. Baseline: `1f91745e3c76a902d7d56e2468453ec06838a724`. Development branch: `green-ops/week1-telemetry`.

## Event definitions

| Field | Meaning |
| --- | --- |
| `rx_attempts` | Calls about to be made to `dect.rx`; includes attempts that return no usable packet. |
| `rx_data_packets` | Received packets whose `pdc()` result is successful. This does not imply successful MAC parsing or recognition of a timestamp beacon. |
| `sync_beacons` | Packets matching the existing beacon parser and timestamp-element conditions; counted once per packet even if multiple matching elements occur. |
| `tx_successes` | Transmit calls that returned success. This does not prove peer reception. |
| `last_rx_timestamp_ticks` | Most recently observed local radio receive timestamp, in raw ticks. |
| `last_tx_scheduled_timestamp_ticks` | Scheduled timestamp supplied to the last successful transmit call, in raw ticks; not an independently measured transmission timestamp. |

Counters saturate at `u32::MAX`. Timestamps continue updating even when counters saturate. `None` means no event has been recorded; `Some(0)` is a valid timestamp. Counters reset when the task restarts. State is owned by one task and needs no shared atomic counters or heap allocation.

## Build

Run from `/home/thite/projects/green-ops/hophop` in Ubuntu/WSL2.

Enable profiling:

```bash
laze build -g -b nrf9151-dk -a hophop-examples -s green-ops-profile -D LOG=info
```

Disable profiling explicitly:

```bash
laze build -g -b nrf9151-dk -a hophop-examples -d green-ops-profile -D LOG=info
```

The feature is disabled by default. Its optional laze module contributes `green-ops-profile` to Ariel's `FEATURES` list. Rust `cfg` attributes guard the telemetry module, state, updates and logs. No hardware is required for compilation. These commands do not flash or execute firmware.

## Tests

Host telemetry tests, independently of the embedded examples package:

```bash
mkdir -p build/green-ops-tests
rustc --edition=2024 -D warnings --test examples/src/green_ops_telemetry.rs -o build/green-ops-tests/telemetry-tests
build/green-ops-tests/telemetry-tests
```

Existing host-compatible workspace tests:

```bash
cargo test --workspace --exclude hophop --exclude nrfxlib-sys-pretty
cargo test --workspace --all-features --exclude hophop --exclude nrfxlib-sys-pretty
```

Standalone embedded compatibility check:

```bash
mkdir -p build/green-ops-tests
cat > build/green-ops-tests/no_std_check.rs <<'RS'
#![no_std]
#[path = "../../examples/src/green_ops_telemetry.rs"]
pub mod telemetry;
RS
rustc --edition=2024 --crate-type lib --target thumbv8m.main-none-eabihf -D warnings build/green-ops-tests/no_std_check.rs -o build/green-ops-tests/libtelemetry_check.rlib
```

The wrapper and compiled binaries are temporary build artifacts. Host tests exercise bookkeeping; the embedded compatibility check proves compilation, not hardware execution. The eight initial tests cover counter increments and saturation, zero/max timestamp values, and beacon/TX isolation from other event fields.

## Logs and limitations

Profiling logs begin with `GREEN_OPS rx_data`, `GREEN_OPS sync_beacon`, or `GREEN_OPS tx_success`. Counts are cumulative. RX attempt counts appear when decoded RX data is logged; an attempt-only timeout produces no dedicated telemetry record. This is not a periodic 100 ms sampler.

The original button-controlled radio behavior and packet format are preserved. Logging adds work and may perturb timing; overhead has not been measured. Full integration error paths and duplicate-element behavior have been inspected in source but not exercised on boards. Existing upstream error behavior, including panics on some radio failures, is retained.

There is no automatic request/reply RTT workload, physical power prediction, 18-feature dataset, or ML/SHAP engine in this increment. Do not interpret differences between separate devices' timestamps as RTT. Hardware logs, physical power and latency accuracy require later lab validation.

## Recorded validation

User-provided terminal evidence shows eight telemetry tests passing with `-D warnings`, successful ARM no_std compilation, successful firmware builds with profiling enabled and disabled, and passing default/all-features host-compatible workspace tests. Enabled/disabled Cargo arguments respectively include/omit the profiling feature. Unused ESP dependency-patch warnings occurred in firmware builds; dependencies were not updated in response.

## Resume work

From PowerShell run `wsl -d Ubuntu`; inside Ubuntu run `cd ~/projects/green-ops/hophop`. Saved source and installed tools survive a normal shutdown. Run `explorer.exe .` to view the project in Windows Explorer. Local Git commits preserve history; keep a separate remote or external backup for device-loss protection.
