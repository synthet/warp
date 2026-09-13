---
name: sync-upstream-fork
description: Merge warpdotdev/warp into the synthet/warp fork and reapply the fork's local-first tweaks during conflict resolution. Use when syncing upstream, resolving merge conflicts against upstream commits, or when the user mentions upstream sync, merge upstream, rebase on warpdotdev, or fork drift.
capability: "upstream merge conflict resolution preserving fork boundary"
side_effect_level: local_write
approval_required: true
requires_tools: "git, cargo, python"
output_schema: "Merge commit on a sync branch plus a resolution report."
risk_class: high
---

# sync-upstream-fork

Merge upstream `warpdotdev/warp` into the `synthet/warp` fork without losing the fork's
local-first boundary — and without shipping code that git merged cleanly but that cannot
compile.

Read [`docs/architecture/synth-fork.md`](../../../docs/architecture/synth-fork.md) first. It is
the authority on what the fork strips; this skill is only the procedure for defending it.
Direction is fixed by [`fork-remotes`](../../rules/fork-remotes.md): upstream is a read-only
source, never a push target.

## The core problem

Upstream keeps refactoring the exact surfaces the fork neuters: billing, credits, Drive, hosted
AI, accounts. So the conflicts are rarely "which line wins." They are usually:

> Upstream changed the **shape** of a function the fork changed the **behavior** of.

Keep both. Take upstream's signature, keep the fork's body. A fork that takes its own side
wholesale drifts until it stops compiling against the new call sites; a fork that takes
upstream's side wholesale silently re-enables billing UI.

## Procedure

### 1. Fetch and size the merge

```bash
git fetch https://github.com/warpdotdev/warp.git master   # by URL; never edits .git/config
git rev-list --left-right --count master...FETCH_HEAD     # left=ours ahead, right=upstream ahead
git merge-tree --write-tree --name-only master FETCH_HEAD # dry-run conflict list, no tree changes
```

Report the conflict count to the user before starting. Merge on a branch, never on `master`:

```bash
git checkout -b sync/upstream-$(date +%Y-%m-%d)
git merge --no-commit --no-ff FETCH_HEAD
```

### 2. Classify every conflict before editing any of them

For each conflicted hunk, ask **what upstream did**, not what the text looks like:

| Upstream did | Resolution |
|---|---|
| Added a parameter / generic (e.g. `scope: &S`) | **Upstream signature + fork body.** Unused params get `_` prefixes |
| Renamed a function | **Rename the fork's version to match**, keep the fork's body, so upstream's call sites resolve |
| Moved code to a new file/module | Port the fork's version **into upstream's new location**; delete the old copy |
| Deleted the surface entirely | **Follow upstream and delete.** The fork's `#[allow(dead_code)]` stub usually references fields upstream also deleted |
| Only touched fork-owned docs | **Fork wins** (`README.md`, `FAQ.md`) |
| Restructured a UI surface the fork only cosmetically differed on | **Upstream wins** — then re-check step 4 |

Verify the "deleted upstream" case before deleting the fork's copy:

```bash
git grep -n "fn <name>" FETCH_HEAD -- app/src crates   # gone upstream?
grep -rn "\.<name>(" app/src crates --include=*.rs     # any caller left?
```

### 3. Hunt the silent conflicts — this is the step that bites

Git resolves textually. Every clean auto-merge that crosses a fork boundary is suspect.
None of these carry conflict markers:

**a. Renamed enum variants / types the fork still names.** Upstream renames
`SettingsSection::OzCloudAPIKeys` → `WarpCloudAgentAPIKeys`; the fork's `matches!` arms keep the
dead name and the file no longer compiles.

```bash
# every symbol the fork references that upstream no longer defines
git grep -n "OldName" -- app/src crates
git grep -c "OldName" FETCH_HEAD -- app/src crates    # 0 => renamed or deleted upstream
```

**b. A module split duplicating the fork's methods.** When upstream splits `foo.rs` into
`foo/{mod,a,b}.rs`, git maps the fork's flat file onto `mod.rs` — so every method the fork
touched is now defined **twice** in two `impl` blocks. Detect it mechanically:

```python
# list methods defined in more than one file of a split module
import re, pathlib, collections
defs = collections.defaultdict(list)
for f in sorted(pathlib.Path("app/src/<module>").glob("*.rs")):
    for m in re.finditer(r"^\s*pub(?:\(crate\))? fn (\w+)", f.read_text(encoding="utf-8"), re.M):
        defs[m.group(1)].append(f.name)
for k, v in sorted(defs.items()):
    if len(set(v)) > 1:
        print(k, v)
```

Then classify each collision by diffing the fork's copy against the **merge base**: identical to
base means the fork never touched it, so drop the duplicate and keep upstream's. Only the ones
that differ carry fork behavior and must be ported onto upstream's signature in upstream's new
file. In the 2026-09-12 sync this was 24 collisions of which only 4 were real.

**c. Types and helpers the fork's code depends on that upstream deleted.** A fork no-op returning
`Vec<CreditPackOption>` stops compiling when `CreditPackOption` is gone. Check the fork's
stripped-surface stubs against upstream's type inventory.

**d. Imports the fork dropped that upstream's new code needs.** Taking upstream's side of a hunk
can require an import the fork had removed; add it back.

### 4. Re-check the boundary after every "upstream wins"

This is the failure mode that ships a regression rather than a build break. Taking upstream's
restructured version can reintroduce a surface the fork deliberately removed — an upgrade CTA, a
"create an account" prompt, a credits banner — because the removal lived in the code you just
replaced.

After resolving, grep the merged tree for what the fork must never show:

```bash
grep -rn "Sign up\|Upgrade\|Compare plans\|Add credits\|create an account" \
  app/src --include=*.rs | grep -v "_test"
```

Anything that reappears must be re-neutered, keeping upstream's structure. Prefer deleting the
branch that renders it over inverting a condition, and leave a comment saying why.

### 5. Verify, and never claim more than you ran

```bash
cargo check --workspace --all-targets      # first; catches the silent conflicts
./script/format
cargo clippy --workspace --all-targets --all-features --tests -- -D warnings
cargo nextest run --no-fail-fast --workspace --exclude command-signatures-v2
```

`-D warnings` is why unused params from step 2 need `_` prefixes and why orphaned fork helpers
need `#[allow(dead_code)]` (per `synth-fork.md`: keep the surface, don't delete it — unless
upstream deleted it too).

Do not commit before `cargo check` is green. If the user asks to commit while it is still
running, say so plainly, commit to the **sync branch only**, and keep `master` untouched until
the suite passes.

### 6. Fold the fork's tests back

Fork tests that assert a stripped behavior (`Hidden`, `true`, `None`) conflict with upstream
tests asserting the real one. Keep the fork's expectations, adopt upstream's call signature:

```bash
# fork keeps its expected value; upstream's scope argument is threaded through
sed -i 's/has_any_ai_remaining(ctx)/has_any_ai_remaining(\&TeamlessScopeForTest, ctx)/g' <file>
```

Upstream tests that can only pass when the stripped behavior is live (e.g. "AI is blocked when
out of credits") stay deleted — they cannot hold in this fork. Say so in the commit message
rather than leaving them silently dropped.

## Commit message

Record the resolution policy, the silent conflicts found, and any boundary you had to actively
defend — a future sync hits the same surfaces. Conventional Commit scope `merge(upstream)`.

## Checklist

- [ ] Fetched by URL; `.git/config` untouched; merging on a sync branch, not `master`
- [ ] Every conflict classified by *what upstream did*, not by picking a side
- [ ] Renamed/moved/deleted symbols traced with `git grep` against `FETCH_HEAD`
- [ ] Split-module duplicate definitions detected and classified against the merge base
- [ ] Boundary grep run after every "upstream wins" resolution
- [ ] `cargo check` green **before** commit; format, clippy, nextest before merging to `master`
- [ ] Commit message names the silent conflicts and the dropped upstream tests
