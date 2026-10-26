// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

// could also use HashMap, but stable sorting is nice in all use cases
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

#[derive(serde::Deserialize)]
struct Root {
    index: BTreeMap<String, Symbol>,
}
#[derive(serde::Deserialize)]
struct Symbol {
    name: Option<String>,
    inner: Inner,
    docs: Option<String>,
}
#[derive(serde::Deserialize)]
enum Inner {
    #[serde(rename = "type_alias")]
    TypeAlias {},
    #[serde(rename = "constant")]
    Constant {},
    #[serde(untagged)]
    Other {},
}

/// Runs a `cargo doc` to generate the JSON output, if possible (enforcing a check or replacing the
/// file as guided by env vars).
///
/// Returns a path of a usable JSON file.
fn build_json() -> std::path::PathBuf {
    rustdoc_json::Builder::default()
        .toolchain("nightly")
        .manifest_path("../Cargo.toml")
        // Features are already set in that Cargo.toml.
        .package("nrfxlib-sys")
        .build()
        .unwrap()
}

// Sort by descending length so that most specific always matches first.
//
// This only has the lower-case variants.
const GROUP_PREFIXES: &[&str] = &[
    "nrf_modem_dect_phy",
    "nrf_modem_dect_mac",
    "nrf_modem_dect_dlc",
    "nrf_modem_dect_clock",
    "nrf_modem_dect_control",
    // Left in there mostly so that we see items that are not filed anywhere else
    "nrf_modem_dect",
];

fn name_and_prefix(item: &Symbol) -> Option<(String, &str)> {
    let name = item.name.clone()?;
    let &prefix = GROUP_PREFIXES.iter().find(|p| name.starts_with(*p))?;
    Some((name, prefix))
}

// Prefixes that don't follow the simple pattern of all variants having the type name as a
// prefix.
#[rustfmt::skip]
const ENUM_PREFIXES: &[(&str, &str)] = &[
    ("nrf_modem_dect_clock_sync_pulse_direction", "NRF_MODEM_DECT_CLOCK_SYNC_PULSE_"),
    ("nrf_modem_dect_control_systemmode", "NRF_MODEM_DECT_MODE_"),
    ("nrf_modem_dect_phy_err", "NRF_MODEM_DECT_PHY_"),
    ("nrf_modem_dect_phy_event_id", "NRF_MODEM_DECT_PHY_EVT_"),
    ("nrf_modem_dect_clock_sync_err", "NRF_MODEM_DECT_CLOCK_SYNC_"),
    ("nrf_modem_dect_clock_sync_event_id", "NRF_MODEM_DECT_CLOCK_SYNC_EVT_"),
    ("nrf_modem_dect_mac_err", "NRF_MODEM_DECT_MAC_STATUS_"),
    ("nrf_modem_dect_mac_band", "NRF_MODEM_DECT_MAC_"),
    ("nrf_modem_dect_mac_band_group_index", "NRF_MODEM_DECT_MAC_PHY_BAND_GROUP_"),
    ("nrf_modem_dect_mac_release_cause", "NRF_MODEM_DECT_MAC_RELEASE_CAUSE_"),
    ("nrf_modem_dect_dlc_flow_control_status", "NRF_MODEM_DECT_DLC_FLOW_CTRL_STATUS_"),
];

#[allow(clippy::too_many_lines)]
fn main() {
    let json = build_json();

    println!("Intermediate JSON artifact built at {}", json.display());

    // No strong need for String other than that .get() acts up when looking up short-lived strings
    let enum_prefixes: BTreeMap<String, &'static str> = ENUM_PREFIXES
        .iter()
        .map(|&(k, v)| (k.to_owned(), v))
        .collect();

    let mut groupstrings: BTreeMap<_, _> =
        GROUP_PREFIXES.iter().map(|p| (p, String::new())).collect();

    // Being a type alias is a strong indication that this is actually enum shaped, and we'll
    // collect all things that use this prefix here so we can link them.
    //
    // Maps all names of type aliases to (later populated) lists of items that are constants of
    // that type and are prefixed with the alias name.
    let mut constant_prefix_is_type_alias = BTreeMap::default();
    // All names that are not documented in top-level (well, grouped) because they are documented
    // somewhere further in.
    let mut not_toplevel = BTreeSet::default();

    let root: Root = serde_json::from_reader(std::fs::File::open(json).unwrap()).unwrap();

    // First pass: extract things we later compare to
    for item in root.index.values() {
        let Some(ref name) = item.name else {
            continue;
        };
        if matches!(item.inner, Inner::TypeAlias { .. }) {
            constant_prefix_is_type_alias.insert(name, BTreeSet::<String>::new());
        }
    }
    assert!(!constant_prefix_is_type_alias.is_empty());
    // Second pass: compare items to things found in the first pass
    for item in root.index.values() {
        let Some((name, _prefix)) = name_and_prefix(item) else {
            continue;
        };
        // Conditions are rather strict: it starts this way, continues with a capital letter
        // (otherwise nrf_modem_dect_mac_band_group_index_NRF… gets slotted in with
        // nrf_modem_dect_mac_band), and for good measure we only pick constants
        // (nrf_modem_dect_mac_band_set would be ruled out by the uppercase rule as well though)
        if let Some((_, variants)) =
            constant_prefix_is_type_alias
                .iter_mut()
                .find(|(alias, _variants)| {
                    name.starts_with(**alias)
                        && name
                            .get(alias.len() + 1..)
                            .is_some_and(|rest| rest.chars().next().unwrap().is_uppercase())
                        && matches!(item.inner, Inner::Constant {})
                })
        {
            variants.insert(name.clone());
            not_toplevel.insert(name);
        }
    }
    // Third pass: output
    for item in root.index.values() {
        let Some((name, prefix)) = name_and_prefix(item) else {
            continue;
        };
        if not_toplevel.contains(&name) {
            continue;
        }
        let stripped = &name.strip_prefix(prefix).unwrap()[1..];
        let groupstring = groupstrings.get_mut(&prefix).unwrap();
        if let Some(variants) = constant_prefix_is_type_alias.get(&name) {
            writeln!(groupstring, "/// Possible values for [`{stripped}`]").unwrap();
            writeln!(groupstring, "pub mod {stripped}_values {{").unwrap();
            for v in variants {
                // First remove the prefix; that is easy because that comes from bindgen.
                let mut v_prestripped = &v.strip_prefix(&name).unwrap()[1..];
                if v_prestripped == "DECT_MAC_RELEASE_CAUSE_RACH_RESOURCE_FAILURE" {
                    // A singular bug, I'd guess.
                    v_prestripped = "NRF_MODEM_DECT_MAC_RELEASE_CAUSE_RACH_RESOURCE_FAILURE";
                }
                // Prefixes are often not systematic, picking them from the list
                let expected_prefix = enum_prefixes.get(&name).map_or_else(
                    || format!("{}_", name.to_ascii_uppercase()),
                    |n| String::from(*n),
                );
                let v_stripped = v_prestripped
                    .strip_prefix(&expected_prefix)
                    .ok_or_else(|| format!("enum_prefixes prefix stripping failed for {v_prestripped}: tried to strip {expected_prefix} matching the rule for {name}"))
                    .unwrap();
                let v_stripped = if v_stripped.chars().next().unwrap().is_ascii_digit() {
                    format!("_{v_stripped}")
                } else {
                    v_stripped.into()
                };
                writeln!(groupstring, "pub use nrfxlib_sys::{v} as {v_stripped};").unwrap();
            }
            writeln!(groupstring, "}}").unwrap();
            if let Some(docs) = &item.docs {
                // Splitting things off because beyond the first line we'll easily get doxygen
                // comments and then things get messy.
                let doc_firstline = docs.split('\n').next().unwrap();
                // Repeating original docs so they are visible as the 1st-line summary
                writeln!(groupstring, "#[doc = r####\"{doc_firstline}\"####]").unwrap();
                writeln!(groupstring, "///").unwrap();
            }
            writeln!(groupstring, "/// See [`{stripped}_values`] for variants.").unwrap();
            if let Some(_docs) = &item.docs {
                // But we can't go full doc(no_inline), for then we wouldn't get documentation at
                // all.
                writeln!(groupstring, "///").unwrap();
                writeln!(groupstring, "/// Full original documentation:").unwrap();
                writeln!(groupstring, "///").unwrap();
            }
        }
        writeln!(groupstring, "#[doc(alias = \"{name}\")]").unwrap();
        writeln!(groupstring, "pub use nrfxlib_sys::{name} as {stripped};").unwrap();
    }

    let mut generated = String::new();
    writeln!(
        generated,
        "// SPDX-FileCopyrightText: Derived file from the nrfxlib-sys inputs"
    )
    .unwrap();
    writeln!(
        generated,
        "// SPDX-License-Identifier: LicenseRef-Nordic-5-Clause"
    )
    .unwrap();

    for (prefix, code) in groupstrings {
        let prefix_shortened = prefix.strip_prefix("nrf_modem_").unwrap();
        if !code.is_empty() {
            writeln!(generated, "pub mod {prefix_shortened} {{\n{code}}}").unwrap();
        }
    }

    let built_file = "../src/built.rs";

    std::fs::write(built_file, generated).unwrap();
}
