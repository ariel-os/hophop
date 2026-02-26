#!/usr/bin/env python3
# SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
# SPDX-License-Identifier: MIT OR Apache-2.0
# /// script
# dependencies = ["matplotlib"]
# ///

import re
import sys
import argparse
import itertools
import collections

import matplotlib.pyplot as plt
import numpy as np

PATTERN = re.compile("Sync received\\. Local ID ([0-9]+) time ([0-9]+)\\. Remote ID ([0-9]+) time ([0-9]+)\\.")

p = argparse.ArgumentParser()
p.add_argument("infile", nargs="+", help="Output files of a ping program")
p.add_argument("--plot", nargs=2, type=int, help="Sender IDs to plot in an X/Y diagram")
p.add_argument("--global-undrift", action='store_true', help="Use full-data-set average clock drift to correct local slope")
args = p.parse_args()

# (sender, sendtime): [(receiver, receivetime)]
transmissions = {}

for file in args.infile:
    for line in open(file):
        if received := PATTERN.search(line):
            (local_id, local_time, sender_id, sender_time) = [int(i) for i in received.groups()]
            transmissions.setdefault((sender_id, sender_time), []).append((local_id, local_time))

senders = collections.Counter(id for (id, _time) in transmissions.keys())
receivers = collections.Counter(id for (id, _time) in itertools.chain.from_iterable(transmissions.values()))

for party in sorted(senders.keys() | receivers):
    print(f"Device {party}: sent {senders[party]} received {receivers[party]}")

if not args.plot:
    print("Add a --plot argument (eg. --plot 1234 5678) for a visual representation of the events; preferably, both should have sent and received pings.")
    sys.exit()

# for (direction, points) in direction_points.items():
#     print(f"Analyzing {direction}")
#     last_delta = None
#     last_sent = None
#     for (sent, received) in points:
#         delta = received - sent
#         print(sent, received, delta)
#         if last_delta is not None:
#             drift = last_delta - delta
#             elapsed = sent - last_sent
#             print(f"Drift {drift} ticks over {elapsed/69120000:.2f} s, {1_000_000 * drift / elapsed:.3f} ppm")
#         last_delta = delta
#         last_sent = sent

party_a, party_b = args.plot

points_a2b_x = []
points_b2a_x = []
points_a2b_y = []
points_b2a_y = []
# FIXME: On the long run, we'll want to distinguish between who was the sender,
# so that we can learn the direction they sent from.
seen_by_both_x = []
seen_by_both_y = []

for (sender_id, sender_time), receive_events in transmissions.items():
    if sender_id in (party_a, party_b):
        # Just take down the red and blue points
        for (recipient_id, recipient_time) in receive_events:
            if recipient_id not in (party_a, party_b):
                continue

            average_time = (sender_time + recipient_time) / 2
            delta_time = recipient_time - sender_time

            if sender_id == party_a and recipient_id == party_b:
                points_a2b_x.append(average_time)
                points_a2b_y.append(delta_time)
            else:
                points_b2a_x.append(average_time)
                points_b2a_y.append(-delta_time)
    else:
        # Events seen by both become other colors
        times_a = [recipient_time for (recipient_id, recipient_time) in receive_events if recipient_id == party_a]
        times_b = [recipient_time for (recipient_id, recipient_time) in receive_events if recipient_id == party_b]
        if times_a and times_b:
            time_a = times_a[0]
            time_b = times_b[0]

            average = (time_a + time_b) / 2
            delta = time_b - time_a
            seen_by_both_x.append(average)
            seen_by_both_y.append(delta)

plot_groups = [
    ('r+', points_a2b_x, points_a2b_y, f"From {party_a} to {party_b}"),
    ('b+', points_b2a_x, points_b2a_y, f"From {party_b} to {party_a}"),
    ('kx', seen_by_both_x, seen_by_both_y, f"Seen by both {party_a} and {party_b}"),
]
# This is not taking into account any relative movement, but let's start simple.
avg_drift = np.mean([np.polyfit(g[1], g[2], deg=1)[0] for g in plot_groups if g[1]])
print(f"Relative full-experiment clock drift between the clocks is {avg_drift * 1000000:.2g}ppm")

# Correcting
if args.global_undrift:
    for (_color, x, y, _label) in plot_groups:
        y[:] = [_y - _x * avg_drift for (_x, _y) in zip(x, y)]

fig, ax = plt.subplots()
for (color, x, y, label) in plot_groups:
    if not x:
        continue
    ax.plot(x, y, color, label=label)
    ax.set_xlabel(f"Average time between {party_a} and {party_b}")
    if args.global_undrift:
        ax.set_ylabel(f"Time difference between {party_a} and {party_b}, correctd by {avg_drift * 1000000:.2g}PPM drift")
    else:
        ax.set_ylabel(f"Time difference between {party_a} and {party_b}")
    ax.grid(True)
    # Not attempting anything smarter with full-file regressions -- the smart
    # thing would be to work locally.
    ax.legend()
    poly = np.polyfit(x, y, deg=1)
    minmax = np.linspace(min(x), max(x), 100)
    ax.plot(minmax, np.polyval(poly, minmax), color="k")
plt.show()
