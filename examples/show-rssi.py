#!/usr/bin/env python3
# SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
# SPDX-License-Identifier: MIT OR Apache-2.0
# /// script
# dependencies = ["matplotlib"]
# ///

import array
import sys
import re
import warnings

import matplotlib.pyplot as plt
from matplotlib.widgets import Slider
import numpy as np

perchannel = {}
abs_min = 0
abs_max = -128

escapes = re.compile("\x1b\\[(.*?)m")

for line in open(sys.argv[1]):
    # Just tolerating, not expecting escapes, so that things also work when
    # cargo run is redirected and thus doesn't produce color output
    line = escapes.sub("", line)

    (_, recognized, tail) = line.partition("[INFO ] RSSI for ")
    if not recognized:
        continue

    (carrier, at, tail) = tail.partition(" at ")
    carrier = int(carrier)

    (timestamp, symbols, tail) = tail.partition(": [")
    # not used for anything yet
    timestamp = int(timestamp)

    (data, end, file_and_line) = tail.partition("]")

    # going through array to easily get the actually-signed-integer semantics
    values = array.array("b", bytes(int(i) for i in data.split(", ")))
    values = np.array(values, dtype='f')
    # an easy "turn every 0 into a NaN", which it actually represents well:
    # When getting RSSI from an RX operation, there are 0 while there's an
    # actual message incoming.
    with warnings.catch_warnings():
        warnings.simplefilter("ignore")
        values = values + 0 * (1 / values)

    abs_min = min(abs_min, min(values))
    abs_max = max(abs_max, max(values))

    perchannel.setdefault(carrier, []).append(values)

minband = min(perchannel.keys())
maxband = max(perchannel.keys())
# Sometimes we take measurements on every single band, sometimes we leave gaps;
# this helps the slider do something sensible.
deltaband = min(big - small for (big, small) in zip(sorted(perchannel.keys())[1:], sorted(perchannel.keys())[:-1]))

bands = list(range(minband, maxband + 1, deltaband))
percentiles = {q: [None for _ in bands] for q in [1, 5, 10, 25, 50, 75, 90, 95, 99]}

for (i, band) in enumerate(bands):
    for (q, qp) in percentiles.items():
        if band in perchannel:
            qp[i] = np.nanpercentile(perchannel[band], q)

print(f"Over all, {abs_min=} {abs_max=}")

fig = plt.figure()

top = fig.add_subplot(10, 1, (1, 7))
bottom = fig.add_subplot(10, 1, (8, 9))
ax_slider = fig.add_subplot(10, 1, 10, sharex=bottom)
# Works for showing labels left and right on a reasonable full-screen view
fig.subplots_adjust(top=0.99, bottom=0.01, left=0.05, right=0.95, hspace=0.3, wspace=0.3)

alldata = np.array(perchannel[1663])

# FIXME: scale better
heatmap = top.imshow(alldata, vmin=abs_min, vmax=abs_max)

for (q, qd) in percentiles.items():
    bottom.plot(bands, qd, label=str(q))
vertical, = bottom.plot([minband, minband], [abs_min, abs_max], label="selected")
bottom.legend()

slider = Slider(
    ax_slider,
    label="Absolute\nChannel\nNumber",
    valmin=minband,
    valmax=maxband,
    valstep=deltaband,
)

def update(val):
    if val in perchannel:
        heatmap.set_visible(True)
        heatmap.set_data(perchannel[val])
    else:
        heatmap.set_visible(False)
    vertical.set(xdata=[val, val])
    fig.canvas.draw_idle()


slider.on_changed(update)

plt.show()
