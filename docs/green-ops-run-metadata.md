# Green-Ops run metadata (component version 1)

This reusable component records run identity and the origin of power/RTT labels.
It does not change the Week 2 event converter or its CSV format. It does not yet
validate workload limits, clocks, power calibration, feature capabilities or
artifact hashes. Acceptance is not evidence of training readiness or truthful
physical measurement. Those checks belong to subsequent manifest components.

All fields shown below are required; unknown fields are rejected to catch typos.
Identifiers use the existing run-ID rule: 1–64 ASCII letters, numbers, dots,
underscores or hyphens, starting with a letter or number.

```json
{
  "metadata_version": 1,
  "run_id": "demo-001",
  "adapter_id": "hophop",
  "feature_profile_id": "hophop-nrf9151-v1",
  "execution_kind": "synthetic",
  "firmware_revision": null,
  "firmware_dirty": null,
  "device_id": null,
  "target_provenance": {
    "target_power_draw_mw": "missing",
    "target_ping_rtt_latency_ms": "missing"
  }
}
```

- `execution_kind`: `synthetic` fixtures, modeled `simulation`, or `hardware` execution.
- `firmware_revision`: full 40- or 64-digit hexadecimal Git object ID, or null when no firmware is used. Abbreviated hashes are not accepted.
- `firmware_dirty`: boolean, or null together with a null revision. A dirty flag records a limitation; reproducing dirty firmware will additionally require an archived patch/build artifact.
- `device_id`: a safe local device identifier, required for hardware runs. Adapter and profile IDs are not restricted to hophop; registration and capability validation are future work.
- Hardware runs require a revision, boolean dirty flag and device ID. They may still have missing labels.
- Both target keys are required. Provenance is `missing`, `measured`, `estimated` or `simulated`. Only hardware execution may claim measured physical targets. A hardware run may have estimated power rather than measured power; estimates never become ground truth merely because hardware was used.

API: `validate_run_metadata(data)` returns None or raises ValueError without
modifying data. `load_run_metadata(path)` reads UTF-8 JSON, rejects duplicate
keys and nonfinite constants, validates and returns the dictionary. Filesystem
and decoding errors propagate. No files are written by either function.

The example deliberately has missing targets. It neither asserts that all 18
features are available nor fabricates measurements for training.

Run checks from the repository root:

```bash
python3 -W error::ResourceWarning -m unittest discover \
  -s tools/green_ops/tests -p 'test_*.py' -v
```
