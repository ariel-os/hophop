# SPDX-FileCopyrightText: 2026 Parth Shailesh Thite
# SPDX-License-Identifier: MIT OR Apache-2.0

import math
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from channel_loss import (
    build_loss_scenario,
    compute_theoretical_metrics,
    simulate_packet_drops,
    validate_loss_scenario,
)


PRESETS = {
    "none": (0.0, 1.0, 0.0, 0.0),
    "mild": (0.02, 0.50, 0.005, 0.40),
    "moderate": (0.05, 0.25, 0.01, 0.70),
    "severe": (0.10, 0.15, 0.03, 0.90),
}


class ScenarioTests(unittest.TestCase):
    def test_named_presets_materialize_all_fields_exactly(self):
        fields = ("good_to_bad_probability", "bad_to_good_probability",
                  "good_state_loss_probability", "bad_state_loss_probability")
        for preset, values in PRESETS.items():
            scenario = build_loss_scenario(preset=preset)
            self.assertEqual(scenario["model"], "gilbert_elliott")
            self.assertEqual(scenario["preset"], preset)
            self.assertEqual(scenario["initial_state"], "good")
            self.assertEqual(tuple(scenario[field] for field in fields), values)
            self.assertEqual(set(scenario), {"model", "preset", "initial_state", *fields})

    def test_custom_requires_and_preserves_all_probabilities(self):
        values = dict(good_to_bad_probability=0.1, bad_to_good_probability=0.2,
                      good_state_loss_probability=0.3, bad_state_loss_probability=0.4)
        scenario = build_loss_scenario(preset="custom", initial_state="bad", **values)
        self.assertEqual(scenario["preset"], "custom")
        self.assertEqual(scenario["initial_state"], "bad")
        for field, value in values.items():
            self.assertEqual(scenario[field], value)

        incomplete = dict(values)
        del incomplete["bad_state_loss_probability"]
        with self.assertRaises(ValueError):
            build_loss_scenario(preset="custom", **incomplete)

    def test_named_presets_reject_manual_probability_overrides(self):
        with self.assertRaises(ValueError):
            build_loss_scenario(preset="moderate", good_to_bad_probability=0.05)

    def test_rejects_invalid_probability_types_values_and_ordering(self):
        base = build_loss_scenario(preset="moderate")
        invalid = [True, 0, -0.1, 1.1, math.nan, math.inf, -math.inf, "0.1", None]
        for field in PRESETS_FIELDS:
            for value in invalid:
                scenario = dict(base)
                scenario[field] = value
                with self.subTest(field=field, value=value), self.assertRaises(ValueError):
                    validate_loss_scenario(scenario)
        inverted = dict(base, good_state_loss_probability=0.8,
                        bad_state_loss_probability=0.2)
        with self.assertRaises(ValueError):
            validate_loss_scenario(inverted)

    def test_rejects_invalid_initial_state_and_schema(self):
        for state in ("unknown", "GOOD", 1, None, []):
            with self.subTest(state=state), self.assertRaises(ValueError):
                build_loss_scenario(preset="none", initial_state=state)
        scenario = build_loss_scenario(preset="none")
        scenario["extra"] = 1
        with self.assertRaises(ValueError):
            validate_loss_scenario(scenario)

        scenario = build_loss_scenario(preset="none")
        scenario["preset"] = []
        with self.assertRaises(ValueError):
            validate_loss_scenario(scenario)

    def test_theoretical_metrics_for_moderate_preset(self):
        metrics = compute_theoretical_metrics(build_loss_scenario(preset="moderate"))
        self.assertAlmostEqual(metrics["steady_state_bad_probability"], 1 / 6)
        self.assertAlmostEqual(metrics["expected_loss_rate"], 0.125)
        self.assertAlmostEqual(metrics["average_bad_burst_length"], 4.0)

    def test_zero_transition_metrics_follow_initial_state(self):
        values = dict(good_to_bad_probability=0.0, bad_to_good_probability=0.0,
                      good_state_loss_probability=0.1, bad_state_loss_probability=0.8)
        good = compute_theoretical_metrics(build_loss_scenario(
            preset="custom", initial_state="good", **values))
        bad = compute_theoretical_metrics(build_loss_scenario(
            preset="custom", initial_state="bad", **values))
        self.assertEqual(good, {"steady_state_bad_probability": 0.0,
                               "expected_loss_rate": 0.1,
                               "average_bad_burst_length": 0.0})
        self.assertEqual(bad, {"steady_state_bad_probability": 1.0,
                              "expected_loss_rate": 0.8,
                              "average_bad_burst_length": None})

    def test_unreachable_bad_state_has_zero_average_burst_length(self):
        metrics = compute_theoretical_metrics(build_loss_scenario(preset="none"))
        self.assertEqual(metrics["steady_state_bad_probability"], 0.0)
        self.assertEqual(metrics["expected_loss_rate"], 0.0)
        self.assertEqual(metrics["average_bad_burst_length"], 0.0)


PRESETS_FIELDS = (
    "good_to_bad_probability", "bad_to_good_probability",
    "good_state_loss_probability", "bad_state_loss_probability",
)


class SimulationTests(unittest.TestCase):
    def test_seeded_simulation_is_repeatable_and_strictly_labeled(self):
        scenario = build_loss_scenario(preset="severe")
        first = simulate_packet_drops(scenario, seed=1, packet_count=8)
        second = simulate_packet_drops(scenario, seed=1, packet_count=8)
        self.assertEqual(first, second)
        self.assertEqual(first["execution_kind"], "simulation")
        self.assertEqual(first["drops"], [False, False, False, False, False, True, True, False])
        self.assertNotIn("measured", first)
        self.assertNotIn("hardware", first)

    def test_packet_zero_uses_exact_initial_state(self):
        values = dict(good_to_bad_probability=0.0, bad_to_good_probability=0.0,
                      good_state_loss_probability=0.0, bad_state_loss_probability=1.0)
        good = build_loss_scenario(preset="custom", initial_state="good", **values)
        bad = build_loss_scenario(preset="custom", initial_state="bad", **values)
        self.assertEqual(simulate_packet_drops(good, seed=9, packet_count=1)["drops"], [False])
        self.assertEqual(simulate_packet_drops(bad, seed=9, packet_count=1)["drops"], [True])

    def test_rejects_invalid_seed_and_packet_count(self):
        scenario = build_loss_scenario(preset="none")
        for seed, count in ((True, 1), (-1, 1), (1, True), (1, -1)):
            with self.subTest(seed=seed, count=count), self.assertRaises(ValueError):
                simulate_packet_drops(scenario, seed=seed, packet_count=count)


if __name__ == "__main__":
    unittest.main()
