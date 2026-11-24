# SPDX-FileCopyrightText: 2026 Parth Shailesh Thite
# SPDX-License-Identifier: MIT OR Apache-2.0

"""Run identity and target provenance, independent of any firmware adapter.

This is one component of the future experiment manifest, not a declaration
that a run contains sufficient metadata or measurements for training.
"""

import json
from pathlib import Path
import re


IDENTIFIER = re.compile(r"[A-Za-z0-9][A-Za-z0-9._-]{0,63}")
REVISION = re.compile(r"(?:[0-9a-fA-F]{40}|[0-9a-fA-F]{64})")
FIELDS = {
    "metadata_version", "run_id", "adapter_id", "feature_profile_id",
    "execution_kind", "firmware_revision", "firmware_dirty", "device_id",
    "target_provenance",
}
TARGETS = {"target_power_draw_mw", "target_ping_rtt_latency_ms"}


def _identifier(value, field):
    if not isinstance(value, str) or IDENTIFIER.fullmatch(value) is None:
        raise ValueError(f"{field} must be a safe ASCII identifier of 1–64 characters")


def validate_run_metadata(data):
    """Raise ValueError for invalid metadata; otherwise return None unchanged."""
    if not isinstance(data, dict) or set(data) != FIELDS:
        raise ValueError("run metadata must contain exactly the documented fields")
    if type(data["metadata_version"]) is not int or data["metadata_version"] != 1:
        raise ValueError("metadata_version must be integer 1")
    for field in ("run_id", "adapter_id", "feature_profile_id"):
        _identifier(data[field], field)

    kind = data["execution_kind"]
    if not isinstance(kind, str) or kind not in {"synthetic", "simulation", "hardware"}:
        raise ValueError("execution_kind must be synthetic, simulation or hardware")

    revision, dirty = data["firmware_revision"], data["firmware_dirty"]
    if revision is not None and (
        not isinstance(revision, str) or REVISION.fullmatch(revision) is None
    ):
        raise ValueError("firmware_revision must be null or a full Git object ID")
    if dirty is not None and type(dirty) is not bool:
        raise ValueError("firmware_dirty must be null or boolean")
    if (revision is None) != (dirty is None):
        raise ValueError("firmware_revision and firmware_dirty must be supplied together")

    device = data["device_id"]
    if device is not None:
        _identifier(device, "device_id")
    if kind == "hardware" and (revision is None or device is None):
        raise ValueError("hardware runs require firmware revision, dirty flag and device ID")

    targets = data["target_provenance"]
    if not isinstance(targets, dict) or set(targets) != TARGETS:
        raise ValueError("target_provenance must name the power and RTT targets exactly")
    for target, provenance in targets.items():
        if not isinstance(provenance, str) or provenance not in {
            "missing", "measured", "estimated", "simulated"
        }:
            raise ValueError(f"invalid provenance for {target}")
        if provenance == "measured" and kind != "hardware":
            raise ValueError(f"{target}: only hardware runs may claim measured targets")


def _unique_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def _reject_constant(value):
    raise ValueError(f"nonfinite JSON value: {value}")


def load_run_metadata(path):
    """Read strict UTF-8 JSON and validate; propagate read/decoding errors."""
    with Path(path).open(encoding="utf-8") as stream:
        data = json.load(stream, object_pairs_hook=_unique_object,
                         parse_constant=_reject_constant)
    validate_run_metadata(data)
    return data
