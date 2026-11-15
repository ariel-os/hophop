# SPDX-FileCopyrightText: 2026 Parth Shailesh Thite
# SPDX-License-Identifier: MIT OR Apache-2.0

"""Build reproducible Green-Ops experiment plans without executing them."""

import hashlib
import json
from pathlib import Path
import re


IDENTIFIER = re.compile(r"[A-Za-z0-9][A-Za-z0-9._-]{0,63}")
CAPABILITY_FIELDS = {
    "capabilities_version", "adapter_id", "feature_profile_id",
    "execution_kinds", "payload_size_bytes", "interval_ms", "tx_power_dbm",
    "loss_models",
}
REQUEST_FIELDS = {
    "request_version", "plan_id", "adapter_id", "feature_profile_id",
    "execution_kind", "payload_size_bytes", "interval_ms", "tx_power_dbm",
    "loss_models", "seeds", "repetitions",
}


def _identifier(value, field, *, maximum=64):
    if not isinstance(value, str) or len(value) > maximum or IDENTIFIER.fullmatch(value) is None:
        raise ValueError(f"{field} must be a safe ASCII identifier of 1–{maximum} characters")


def _integer(value, field, *, minimum=None):
    if type(value) is not int or (minimum is not None and value < minimum):
        raise ValueError(f"{field} must be an integer{' at least ' + str(minimum) if minimum is not None else ''}")


def _unique_nonempty_list(value, field, item_validator):
    if not isinstance(value, list) or not value:
        raise ValueError(f"{field} must be a non-empty list")
    for item in value:
        item_validator(item, field)
    if len(set(value)) != len(value):
        raise ValueError(f"{field} must not contain duplicates")


def _validate_capabilities(data):
    if not isinstance(data, dict) or set(data) != CAPABILITY_FIELDS:
        raise ValueError("capabilities must contain exactly the documented fields")
    if type(data["capabilities_version"]) is not int or data["capabilities_version"] != 1:
        raise ValueError("capabilities_version must be integer 1")
    for field in ("adapter_id", "feature_profile_id"):
        _identifier(data[field], field)
    _unique_nonempty_list(data["execution_kinds"], "execution_kinds", _identifier)
    for field in ("payload_size_bytes", "interval_ms"):
        _unique_nonempty_list(data[field], field, lambda value, name: _integer(value, name, minimum=1))
    _unique_nonempty_list(data["tx_power_dbm"], "tx_power_dbm", _integer)
    _unique_nonempty_list(data["loss_models"], "loss_models", _identifier)


def _validate_request(data):
    if not isinstance(data, dict) or set(data) != REQUEST_FIELDS:
        raise ValueError("request must contain exactly the documented fields")
    if type(data["request_version"]) is not int or data["request_version"] != 1:
        raise ValueError("request_version must be integer 1")
    _identifier(data["plan_id"], "plan_id", maximum=57)
    for field in ("adapter_id", "feature_profile_id", "execution_kind"):
        _identifier(data[field], field)
    for field in ("payload_size_bytes", "interval_ms"):
        _unique_nonempty_list(data[field], field, lambda value, name: _integer(value, name, minimum=1))
    _unique_nonempty_list(data["tx_power_dbm"], "tx_power_dbm", _integer)
    _unique_nonempty_list(data["loss_models"], "loss_models", _identifier)
    _unique_nonempty_list(data["seeds"], "seeds", lambda value, name: _integer(value, name, minimum=0))
    _integer(data["repetitions"], "repetitions", minimum=1)


def _canonical_sha256(value):
    encoded = json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True).encode("ascii")
    return hashlib.sha256(encoded).hexdigest()


def build_experiment_plan(capabilities, request):
    """Return a deterministic plan for requested, capability-supported settings."""
    _validate_capabilities(capabilities)
    _validate_request(request)
    for field in ("adapter_id", "feature_profile_id"):
        if request[field] != capabilities[field]:
            raise ValueError(f"request {field} does not match capabilities")
    if request["execution_kind"] not in capabilities["execution_kinds"]:
        raise ValueError("requested execution_kind is unsupported")
    for field in ("payload_size_bytes", "interval_ms", "tx_power_dbm", "loss_models"):
        unsupported = set(request[field]) - set(capabilities[field])
        if unsupported:
            raise ValueError(f"unsupported {field}: {sorted(unsupported)!r}")

    experiments = []
    for payload_size_bytes in request["payload_size_bytes"]:
        for interval_ms in request["interval_ms"]:
            for tx_power_dbm in request["tx_power_dbm"]:
                for loss_model in request["loss_models"]:
                    for seed in request["seeds"]:
                        for repetition in range(1, request["repetitions"] + 1):
                            experiments.append({
                                "run_id": f"{request['plan_id']}-{len(experiments) + 1:06d}",
                                "state": "planned",
                                "requested_settings": {
                                    "payload_size_bytes": payload_size_bytes,
                                    "interval_ms": interval_ms,
                                    "tx_power_dbm": tx_power_dbm,
                                    "loss_model": loss_model,
                                    "seed": seed,
                                    "repetition": repetition,
                                },
                            })

    configuration = {
        "adapter_id": request["adapter_id"],
        "feature_profile_id": request["feature_profile_id"],
        "execution_kind": request["execution_kind"],
        "payload_size_bytes": request["payload_size_bytes"],
        "interval_ms": request["interval_ms"],
        "tx_power_dbm": request["tx_power_dbm"],
        "loss_models": request["loss_models"],
        "seeds": request["seeds"],
        "repetitions": request["repetitions"],
    }
    return {
        "plan_version": 1,
        "plan_id": request["plan_id"],
        "adapter_id": request["adapter_id"],
        "feature_profile_id": request["feature_profile_id"],
        "execution_kind": request["execution_kind"],
        "configuration_sha256": _canonical_sha256(configuration),
        "experiments": experiments,
    }


def write_experiment_plan(path, plan):
    """Write a plan once as stable, newline-terminated UTF-8 JSON."""
    with Path(path).open("x", encoding="utf-8", newline="\n") as stream:
        json.dump(plan, stream, indent=2, sort_keys=True)
        stream.write("\n")
