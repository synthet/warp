---
description: Synth Warp is a fork — all writes target synthet/warp; warpdotdev/warp is read-only upstream.
alwaysApply: true
---

# Fork remotes & PR targets (always on)

**Our repository is `synthet/warp`. `warpdotdev/warp` is the read-only upstream source.**

| Repo | Role | Direction |
|------|------|-----------|
| `synthet/warp` | **ours** — every branch, PR, merge, push, issue, and release | read + write |
| `warpdotdev/warp` | upstream source | **read only** — fetch and merge *in*, never push or PR *out* |

## Never

- **Never open a PR against `warpdotdev/warp`**, or any base outside `synthet/warp`.
- **Never push** a branch to, merge into, or otherwise mutate `warpdotdev/warp` — no issues, no
  comments, no releases. We have no write intent there even if the token happens to allow it.
- Never treat a `warpdotdev` URL the user pastes as a merge target. Reading one to *review* code is
  fine; the resulting change still lands in `synthet/warp`.

## Always

- Branch from and merge into `synthet/warp:master`.
- Sync upstream **into** the fork, never the reverse:

  ```bash
  git fetch https://github.com/warpdotdev/warp.git master   # no remote needed → FETCH_HEAD
  git merge FETCH_HEAD                                      # resolve fork-specific conflicts
  ```

  Fetching by URL avoids editing `.git/config`, which [`safety-and-secrets.md`](./safety-and-secrets.md)
  forbids. If a named `upstream` remote exists, verify it points at `warpdotdev/warp` before trusting
  it — in this checkout `upstream` has pointed at `synthet/warp`, making `upstream/master` a synonym
  for `origin/master` rather than the source repo.

## `gh` defaults are the trap

From a fork, `gh pr create` defaults `--base` to the **parent** repo. Always be explicit:

```bash
gh pr create --repo synthet/warp --base master    # correct
gh pr create                                      # WRONG — bases on warpdotdev/warp
```

Set it once per checkout with `gh repo set-default synthet/warp`. The same default applies to
`gh issue create`, `gh pr list`, and `gh api` paths — name `synthet/warp` explicitly.

This is not hypothetical: a bare `gh pr create` opened warpdotdev/warp#15974, which proposed the
entire Synth fork (737 files, +45,610/−5,324, 28 commits) to upstream maintainers instead of the
intended five-line docs change.

## Before any remote write

State the target repo out loud and confirm it is `synthet/warp`. Per
[`CLAUDE.md`](../../CLAUDE.md), GitHub mutations need explicit task intent **and target
verification** — this rule is that verification step.

Working if: every PR, push, and issue you create is on `synthet/warp`, and upstream only ever
appears as a fetch source.
