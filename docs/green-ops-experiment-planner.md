# Green-Ops experiment planner

The experiment planner produces deterministic JSON describing proposed runs.
It neither runs hophop nor creates data, measurements, targets or trained
models. Every generated experiment has `state: "planned"`.

The Python API is:

```python
from experiment_planner import build_experiment_plan, write_experiment_plan

plan = build_experiment_plan(capabilities, request)
write_experiment_plan("plan.json", plan)
```

`capabilities` declares an adapter’s named software planning envelope:

- `capabilities_version`: integer `2`.
- `adapter_id`, `feature_profile_id`: safe 1–64 character identifiers.
- `execution_kinds`: supported modes, currently `simulation` and/or `hardware`.
- `payload_size_bytes`, `interval_ms`, `tx_power_dbm`, `loss_presets`: non-empty, duplicate-free supported values.

`request` has `request_version: 2`, a `plan_id`, matching adapter/profile IDs,
one supported execution kind, selected non-empty values for every sweep field,
non-negative integer seeds and a positive integer repetition count. The plan
ID is limited to 57 characters so generated run IDs such as
`week3-demo-000001` remain within the project’s 64-character identifier rule.

`loss_scenarios` contains named or custom Gilbert–Elliott selectors described
in `green-ops-channel-loss.md`. The planner resolves every selector and embeds
all seven scenario fields in each experiment. Generated experiments remain
`state: "planned"`; their presence does not mean a simulation or hardware run
occurred.

The output order follows the declared request order: payload, interval, TX
power, loss scenario, seed, then repetition. Equal inputs yield equal plans and
the `configuration_sha256` binds the requested configuration. The writer uses
exclusive creation and refuses to overwrite an existing file.

`hophop-capabilities.json` is a first adapter fixture for testing software
planning. Its values are proposed test settings from the project brief, not a
claim that every setting has been verified on an nRF9151 or is executable by
the current hophop Ping example. The later hophop execution adapter must
replace unsupported planned values with a documented, tested capability set;
it must never silently clamp them.

Run the checks from the repository root:

```bash
python3 -W error::ResourceWarning -m unittest discover \
  -s tools/green_ops/tests -p 'test_*.py' -v
```
