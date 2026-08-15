---
title: Wiki mirror
description: This folder is identical in Synth Warp and Synth Zed. Edit one copy, then copy the whole folder to the sibling checkout.
---

# Wiki mirror

This folder is **byte-identical** in both checkouts:

| Checkout | Path |
|----------|------|
| Synth Warp | `D:/Projects/warp/docs/zed-warp/` |
| Synth Zed | `D:/Projects/zed/docs/zed-warp/` |

Internal links (`level-1.md`, `surfaces/…`) work in both trees. Code outside this folder is **not** linked relatively (those paths would 404 in the other repo). Cite it as:

- `(Warp) app/src/…`
- `(Zed) crates/…`

After editing, copy the whole directory to the sibling. Do not let the two copies drift.

## See also

- [Zed × Warp Hybrid](README.md)
