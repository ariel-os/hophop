# SPDX-FileCopyrightText: 2026 Parth Shailesh Thite
# SPDX-License-Identifier: MIT OR Apache-2.0

import copy
import json
from pathlib import Path
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from run_metadata import load_run_metadata, validate_run_metadata


def example():
    return {
        "metadata_version": 1,
        "run_id": "demo-001",
        "adapter_id": "hophop",
        "feature_profile_id": "hophop-nrf9151-v1",
        "execution_kind": "synthetic",
        "firmware_revision": None,
        "firmware_dirty": None,
        "device_id": None,
        "target_provenance": {
            "target_power_draw_mw": "missing",
            "target_ping_rtt_latency_ms": "missing",
        },
    }


class RunMetadataTests(unittest.TestCase):
    def test_synthetic_metadata_is_accepted_without_mutation(self):
        data = example()
        original = copy.deepcopy(data)
        self.assertIsNone(validate_run_metadata(data))
        self.assertEqual(data, original)

    def test_other_adapter_and_profile_are_accepted(self):
        data = example()
        data.update(adapter_id="another-platform", feature_profile_id="custom-v1")
        self.assertIsNone(validate_run_metadata(data))

    def test_hardware_metadata_with_revision_and_device_is_accepted(self):
        data = example()
        data.update(execution_kind="hardware", firmware_revision="a" * 40,
                    firmware_dirty=False, device_id="board-01")
        data["target_provenance"]["target_power_draw_mw"] = "measured"
        self.assertIsNone(validate_run_metadata(data))

    def test_simulated_targets_are_accepted_for_simulation(self):
        data = example()
        data["execution_kind"] = "simulation"
        data["target_provenance"] = dict.fromkeys(data["target_provenance"], "simulated")
        self.assertIsNone(validate_run_metadata(data))

    def test_missing_unknown_and_nonobject_inputs_are_rejected(self):
        missing = example()
        del missing["run_id"]
        extra = example()
        extra["runid"] = "typo"
        for data in (missing, extra, [], None):
            with self.subTest(data=data), self.assertRaises(ValueError):
                validate_run_metadata(data)

    def test_invalid_scalar_fields_are_rejected(self):
        cases = {
            "metadata_version": [True, 1.0, 2, "1"],
            "run_id": [None, "", "=formula", "bad\nvalue", "a" * 65],
            "adapter_id": [12, "bad id"],
            "feature_profile_id": ["../profile", False],
            "execution_kind": ["lab", [], None],
            "firmware_revision": ["cd229c1", "z" * 40, 1],
            "firmware_dirty": [0, "false"],
            "device_id": ["", 1],
        }
        for field, values in cases.items():
            for value in values:
                data = example()
                data[field] = value
                with self.subTest(field=field, value=value), self.assertRaises(ValueError):
                    validate_run_metadata(data)

    def test_hardware_requires_identifiable_firmware_and_device(self):
        for field in ("firmware_revision", "firmware_dirty", "device_id"):
            data = example()
            data.update(execution_kind="hardware", firmware_revision="b" * 40,
                        firmware_dirty=False, device_id="board-01")
            data[field] = None
            with self.subTest(field=field), self.assertRaises(ValueError):
                validate_run_metadata(data)

    def test_revision_and_dirty_flag_must_be_supplied_together(self):
        for changes in ({"firmware_revision": "a" * 40}, {"firmware_dirty": False}):
            data = example()
            data.update(changes)
            with self.subTest(changes=changes), self.assertRaises(ValueError):
                validate_run_metadata(data)

    def test_nonhardware_cannot_claim_measured_targets(self):
        for kind in ("synthetic", "simulation"):
            for target in example()["target_provenance"]:
                data = example()
                data["execution_kind"] = kind
                data["target_provenance"][target] = "measured"
                with self.subTest(kind=kind, target=target), self.assertRaises(ValueError):
                    validate_run_metadata(data)

    def test_target_names_and_provenance_are_strict(self):
        for targets in ({}, [], {"power": "measured"},
                        dict.fromkeys(example()["target_provenance"], "ground-truth")):
            data = example()
            data["target_provenance"] = targets
            with self.subTest(targets=targets), self.assertRaises(ValueError):
                validate_run_metadata(data)

    def test_json_round_trip(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "run.json"
            path.write_text(json.dumps(example()), encoding="utf-8")
            self.assertEqual(load_run_metadata(path), example())

    def test_loader_rejects_duplicate_keys_invalid_json_and_nonfinite_values(self):
        valid = json.dumps(example())
        cases = ["{", valid.replace('"metadata_version": 1',
                 '"metadata_version": 2, "metadata_version": 1'),
                 valid.replace('"firmware_dirty": null', '"firmware_dirty": NaN')]
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "run.json"
            for content in cases:
                path.write_text(content, encoding="utf-8")
                with self.subTest(content=content), self.assertRaises(ValueError):
                    load_run_metadata(path)


if __name__ == "__main__":
    unittest.main()
