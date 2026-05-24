Like `../examples`, this contains applications built on hophop;
those here are more standalone in that they come with their own Cargo and laze configuration.

The intention of those examples is more to serve as a practical tool and less to illustrate or research
(like the examples do).
This line is rather wobbly at the time of writing.

---

Currently, this contains one application: [`embedded-pt`](./embedded-pt/).
Other envisioned applications are
`bridge-pt` (connecting IP traffic to a slipmux terminal rather than into the Ariel OS / embassy-net local network stack),
and `bridge-ft` (serving as a sink).
On the long run, those might become unified.
