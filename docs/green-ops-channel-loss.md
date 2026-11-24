# Green-Ops Gilbert–Elliott channel-loss scenarios

Green-Ops uses a two-state Gilbert–Elliott model for reproducible burst-loss
simulation. This is a software channel model. Its output is never measured RF
or hardware evidence.

Every materialized scenario contains exactly these seven fields:

```json
{
  "model": "gilbert_elliott",
  "preset": "moderate",
  "initial_state": "good",
  "good_to_bad_probability": 0.05,
  "bad_to_good_probability": 0.25,
  "good_state_loss_probability": 0.01,
  "bad_state_loss_probability": 0.70
}
```

The four probabilities must be finite Python floats in `[0.0, 1.0]`.
Booleans, integers, NaN, infinities and strings are rejected. The bad-state
loss probability cannot be lower than the good-state loss probability.
`initial_state` is `good` by default and may be explicitly set to `good` or
`bad`.

## Frozen presets

| Preset | Good→bad | Bad→good | Loss in good | Loss in bad |
|---|---:|---:|---:|---:|
| `none` | 0.0 | 1.0 | 0.0 | 0.0 |
| `mild` | 0.02 | 0.50 | 0.005 | 0.40 |
| `moderate` | 0.05 | 0.25 | 0.01 | 0.70 |
| `severe` | 0.10 | 0.15 | 0.03 | 0.90 |

Named presets do not accept manual probability fields. Use `preset: "custom"`
and provide all four probabilities to define a custom scenario. This prevents
an artifact labeled `moderate` from silently containing different numbers.

## Analytical metrics

`compute_theoretical_metrics(scenario)` returns:

- `steady_state_bad_probability`: `good_to_bad / (good_to_bad + bad_to_good)`.
  When both transitions are zero, it is 0.0 from an initial good state and 1.0
  from an initial bad state.
- `expected_loss_rate`: the state-probability-weighted loss probability.
- `average_bad_burst_length`: `1 / bad_to_good` when the bad state is
  reachable; 0.0 when it is unreachable from an initial good state; otherwise
  `null` when an entered bad state cannot be left.

These are mathematical properties of the configured model, not measured
network performance.

## Deterministic packet simulation

`simulate_packet_drops(scenario, seed=..., packet_count=...)` uses only a local
`random.Random(seed)`. Packet index 0 uses the exact configured initial state.
For every packet, the simulator first draws whether that packet is lost in the
current state and then draws the state transition for the next packet. Fixing
this order makes sequences reproducible across Green-Ops runs using the same
Python runtime contract.

The returned object contains `execution_kind: "simulation"`, the seed, packet
count and Boolean drop sequence. It never uses `measured` or `hardware` labels.

Run the full checks from the repository root:

```bash
python3 -W error::ResourceWarning -m unittest discover \
  -s tools/green_ops/tests -p 'test_*.py' -v
```
