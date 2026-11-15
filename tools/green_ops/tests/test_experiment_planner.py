# SPDX-FileCopyrightText: 2026 Parth Shailesh Thite
# SPDX-License-Identifier: MIT OR Apache-2.0

import json
from pathlib import Path
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from experiment_planner import build_experiment_plan, write_experiment_plan


def capabilities():
    return {
        "capabilities_version": 1,
        "adapter_id": "hophop",
        "feature_profile_id": "hophop-nrf9151-v1",
        "execution_kinds": ["simulation", "hardware"],
        "payload_size_bytes": [32, 64, 1501],
        "interval_ms": [5, 100, 500],
        "tx_power_dbm": [-10, 0, 23],
        "loss_models": ["none", "gilbert_elliott"],
    }


def request():
    return {
        "request_version": 1,
        "plan_id": "week3-demo",
        "adapter_id": "hophop",
        "feature_profile_id": "hophop-nrf9151-v1",
        "execution_kind": "simulation",
        "payload_size_bytes": [32, 64],
        "interval_ms": [100],
        "tx_power_dbm": [0],
        "loss_models": ["none", "gilbert_elliott"],
        "seeds": [42],
        "repetitions": 2,
    }


class ExperimentPlannerTests(unittest.TestCase):
    def test_expands_requested_combinations_as_planned_runs(self):
        plan = build_experiment_plan(capabilities(), request())
        self.assertEqual(plan["plan_version"], 1)
        self.assertEqual(plan["plan_id"], "week3-demo")
        self.assertEqual(plan["execution_kind"], "simulation")
        self.assertEqual(len(plan["experiments"]), 8)
        first = plan["experiments"][0]
        self.assertEqual(first["run_id"], "week3-demo-000001")
        self.assertEqual(first["state"], "planned")
        self.assertEqual(first["requested_settings"], {
            "payload_size_bytes": 32,
            "interval_ms": 100,
            "tx_power_dbm": 0,
            "loss_model": "none",
            "seed": 42,
            "repetition": 1,
        })
        self.assertNotIn("targets", first)
        self.assertNotIn("actual_settings", first)
        self.assertEqual(plan["experiments"][-1]["requested_settings"], {
            "payload_size_bytes": 64,
            "interval_ms": 100,
            "tx_power_dbm": 0,
            "loss_model": "gilbert_elliott",
            "seed": 42,
            "repetition": 2,
        })

    def test_same_inputs_produce_identical_plan_and_digest(self):
        first = build_experiment_plan(capabilities(), request())
        second = build_experiment_plan(capabilities(), request())
        self.assertEqual(first, second)
        self.assertRegex(first["configuration_sha256"], r"^[0-9a-f]{64}$")

    def test_keeps_safe_non_hophop_adapter_ids_generic(self):
        caps = capabilities()
        caps.update(adapter_id="platform-b", feature_profile_id="platform-b-v1")
        req = request()
        req.update(adapter_id="platform-b", feature_profile_id="platform-b-v1")
        self.assertEqual(build_experiment_plan(caps, req)["adapter_id"], "platform-b")

    def test_rejects_missing_unknown_and_nonobject_inputs(self):
        missing = request()
        del missing["seeds"]
        unknown = request()
        unknown["payload_bytes"] = [99]
        for caps, req in ((capabilities(), missing), (capabilities(), unknown), ([], request()),
                          (capabilities(), None)):
            with self.subTest(caps=caps, req=req), self.assertRaises(ValueError):
                build_experiment_plan(caps, req)

    def test_rejects_empty_duplicate_and_unsupported_choices(self):
        cases = [
            ("payload_size_bytes", []),
            ("payload_size_bytes", [32, 32]),
            ("payload_size_bytes", [33]),
            ("interval_ms", [99]),
            ("tx_power_dbm", [1]),
            ("loss_models", ["random"]),
            ("seeds", [True]),
            ("seeds", [-1]),
            ("repetitions", 0),
            ("repetitions", True),
        ]
        for field, value in cases:
            req = request()
            req[field] = value
            with self.subTest(field=field, value=value), self.assertRaises(ValueError):
                build_experiment_plan(capabilities(), req)

    def test_rejects_unhashable_choice_values_as_validation_errors(self):
        req = request()
        req["loss_models"] = [["none"]]
        with self.assertRaises(ValueError):
            build_experiment_plan(capabilities(), req)

    def test_rejects_mismatched_capability_and_request_identity(self):
        for field, value in (("adapter_id", "other"), ("feature_profile_id", "other-v1"),
                             ("execution_kind", "synthetic")):
            req = request()
            req[field] = value
            with self.subTest(field=field), self.assertRaises(ValueError):
                build_experiment_plan(capabilities(), req)

    def test_rejects_unsafe_plan_ids_and_ids_too_long_for_run_suffix(self):
        for value in ("=formula", "bad id", "a" * 58):
            req = request()
            req["plan_id"] = value
            with self.subTest(value=value), self.assertRaises(ValueError):
                build_experiment_plan(capabilities(), req)

    def test_writes_newline_terminated_canonical_json_without_overwriting(self):
        plan = build_experiment_plan(capabilities(), request())
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "plan.json"
            write_experiment_plan(path, plan)
            text = path.read_text(encoding="utf-8")
            self.assertTrue(text.endswith("\n"))
            self.assertEqual(json.loads(text), plan)
            original = text
            with self.assertRaises(FileExistsError):
                write_experiment_plan(path, plan)
            self.assertEqual(path.read_text(encoding="utf-8"), original)

    def test_hophop_fixture_is_valid_capability_input(self):
        fixture = Path(__file__).resolve().parents[1] / "fixtures" / "hophop-capabilities.json"
        caps = json.loads(fixture.read_text(encoding="utf-8"))
        plan = build_experiment_plan(caps, request())
        self.assertEqual(plan["adapter_id"], "hophop")


if __name__ == "__main__":
    unittest.main()
