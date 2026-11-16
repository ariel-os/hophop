# SPDX-FileCopyrightText: 2026 Parth Shailesh Thite
# SPDX-License-Identifier: MIT OR Apache-2.0

"""Gilbert–Elliott scenario validation, analysis and simulation."""

import math
import random


PROBABILITY_FIELDS = (
    "good_to_bad_probability",
    "bad_to_good_probability",
    "good_state_loss_probability",
    "bad_state_loss_probability",
)
SCENARIO_FIELDS = {"model", "preset", "initial_state", *PROBABILITY_FIELDS}
PRESETS = {
    "none": (0.0, 1.0, 0.0, 0.0),
    "mild": (0.02, 0.50, 0.005, 0.40),
    "moderate": (0.05, 0.25, 0.01, 0.70),
    "severe": (0.10, 0.15, 0.03, 0.90),
}


def _validate_probability(value, field):
    if type(value) is not float or not math.isfinite(value) or not 0.0 <= value <= 1.0:
        raise ValueError(f"{field} must be a finite float within [0.0, 1.0]")


def validate_loss_scenario(scenario):
    """Validate a fully materialized scenario without modifying it."""
    if not isinstance(scenario, dict) or set(scenario) != SCENARIO_FIELDS:
        raise ValueError("loss scenario must contain exactly the seven documented fields")
    if scenario["model"] != "gilbert_elliott":
        raise ValueError("model must be gilbert_elliott")
    if not isinstance(scenario["preset"], str) or scenario["preset"] not in {*PRESETS, "custom"}:
        raise ValueError("preset must be none, mild, moderate, severe or custom")
    if not isinstance(scenario["initial_state"], str) or scenario["initial_state"] not in {"good", "bad"}:
        raise ValueError("initial_state must be good or bad")
    for field in PROBABILITY_FIELDS:
        _validate_probability(scenario[field], field)
    if scenario["bad_state_loss_probability"] < scenario["good_state_loss_probability"]:
        raise ValueError("bad-state loss probability must not be lower than good-state loss probability")
    if scenario["preset"] in PRESETS:
        expected = dict(zip(PROBABILITY_FIELDS, PRESETS[scenario["preset"]]))
        if any(scenario[field] != expected[field] for field in PROBABILITY_FIELDS):
            raise ValueError("named preset probabilities must match their frozen definition")


def build_loss_scenario(
    *, preset, initial_state="good", good_to_bad_probability=None,
    bad_to_good_probability=None, good_state_loss_probability=None,
    bad_state_loss_probability=None,
):
    """Return a complete named or custom Gilbert–Elliott scenario."""
    if preset not in {*PRESETS, "custom"}:
        raise ValueError("unknown loss preset")
    supplied = {
        "good_to_bad_probability": good_to_bad_probability,
        "bad_to_good_probability": bad_to_good_probability,
        "good_state_loss_probability": good_state_loss_probability,
        "bad_state_loss_probability": bad_state_loss_probability,
    }
    if preset == "custom":
        if any(value is None for value in supplied.values()):
            raise ValueError("custom preset requires all four probabilities")
        probabilities = supplied
    else:
        if any(value is not None for value in supplied.values()):
            raise ValueError("named presets do not accept manual probability overrides")
        probabilities = dict(zip(PROBABILITY_FIELDS, PRESETS[preset]))
    scenario = {
        "model": "gilbert_elliott",
        "preset": preset,
        "initial_state": initial_state,
        **probabilities,
    }
    validate_loss_scenario(scenario)
    return scenario


def compute_theoretical_metrics(scenario):
    """Compute analytical stationary loss and bad-burst metrics."""
    validate_loss_scenario(scenario)
    good_to_bad = scenario["good_to_bad_probability"]
    bad_to_good = scenario["bad_to_good_probability"]
    transition_sum = good_to_bad + bad_to_good
    if transition_sum == 0.0:
        steady_bad = 0.0 if scenario["initial_state"] == "good" else 1.0
    else:
        steady_bad = good_to_bad / transition_sum
    expected_loss = (
        (1.0 - steady_bad) * scenario["good_state_loss_probability"]
        + steady_bad * scenario["bad_state_loss_probability"]
    )
    if good_to_bad == 0.0 and scenario["initial_state"] == "good":
        average_bad_burst = 0.0
    elif bad_to_good > 0.0:
        average_bad_burst = 1.0 / bad_to_good
    else:
        average_bad_burst = None
    return {
        "steady_state_bad_probability": steady_bad,
        "expected_loss_rate": expected_loss,
        "average_bad_burst_length": average_bad_burst,
    }


def simulate_packet_drops(scenario, *, seed, packet_count):
    """Sample loss in the current state, then transition after each packet."""
    validate_loss_scenario(scenario)
    if type(seed) is not int or seed < 0:
        raise ValueError("seed must be a non-negative integer")
    if type(packet_count) is not int or packet_count < 0:
        raise ValueError("packet_count must be a non-negative integer")
    generator = random.Random(seed)
    state = scenario["initial_state"]
    drops = []
    for _ in range(packet_count):
        drops.append(generator.random() < scenario[f"{state}_state_loss_probability"])
        transition = scenario[
            "good_to_bad_probability" if state == "good" else "bad_to_good_probability"
        ]
        if generator.random() < transition:
            state = "bad" if state == "good" else "good"
    return {
        "execution_kind": "simulation",
        "seed": seed,
        "packet_count": packet_count,
        "drops": drops,
    }
