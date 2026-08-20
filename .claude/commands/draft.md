---
capability: "draft agent asset workflow"
side_effect_level: local_write
approval_required: false
requires_tools: "See asset body for tool requirements."
output_schema: "Markdown report or documented command output."
risk_class: medium
---

> **Claude Code:** Same intent as Cursor `/draft`. When customizing, keep in sync with `.cursor/commands/draft.md`.

# /draft — Turn a work item into the written artifact it needs

Use at intake, before `/spec`. You are given a work item — a GitHub issue, an action item, a TODO
comment, a backlog line, a meeting note, or a one-sentence request. Your job is to decide **which
artifact this item actually deserves**, write it in this repo's format, and hand off. Nothing more.

Over-documenting a small item is a failure in the same way under-documenting a design-bearing one is.

## Inputs

- The raw item text, **verbatim** — paste it back at the top of your output.
- Its source and ID: `gh issue view <N>`, a `specs/<REF>/` path, `file.rs:line` for a TODO, or "ad hoc".
- Repo conventions: `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `docs/CANONICAL_SOURCES.md`.
- Any code the item points at. **Read it before writing about it.**

Work comes from GitHub issues and existing specs under `specs/`. Do not invent items. If the item has
no source, say so and treat it as ad hoc.

## Step 1 — Restate and triage

Restate the item in one sentence as an **outcome**, not an activity:
"<user or system> can <observable behavior>" — not "refactor the X module".

Then classify. State the class and one sentence of justification.

| Class | Signal | Artifact to produce |
|---|---|---|
| **Trivial** | Typo, dead link, obvious one-liner, single unambiguous edit | None. Say "no doc needed", state the change, stop. |
| **Small** | Clear behavior, one crate or module, no design choices | `/plan` in-session only |
| **Medium** | Behavior needs pinning down, or acceptance is arguable | `PRODUCT.md` → `/plan` |
| **Design-bearing** | Two or more viable approaches; a new interface, schema, or data flow; **crosses `app/` ↔ `crates/warp_tui` ↔ `crates/warp_core` / `crates/warpui`**; **needs a `FeatureFlag`**; **touches terminal-model locking**; or is hard to reverse | `PRODUCT.md` → `TECH.md` → `/plan` |
| **Epic** | Multiple independent work streams, more than about a week | `/decompose` first, then re-run `/draft` per subtask |

The bolded signals are Warp-specific escalators. They make a change design-bearing regardless of how
small the diff looks — a two-line change that crosses the GUI/TUI boundary or adds a feature flag still
earns a `TECH.md`.

If the class is genuinely borderline, **pick the lighter one** and name what would push it heavier.

## Step 2 — Ask only blocking questions

At most **five**, and only where different answers produce materially different work. For each: the
question, why it matters, your recommended default, and the consequence of that default.

Everything else becomes a written **Assumption**. Proceed; do not block. If more than five material
ambiguities survive, stop and hand off to `/clarify` rather than guessing in bulk.

## Step 3 — Ground before writing

- Every claim about current behavior cites a `path.rs:line` or the output of a command you ran.
- If you did not look, write `unverified` next to the claim rather than asserting it.
- **Don't invent contracts.** Check `docs/CANONICAL_SOURCES.md` before naming an API path, config key,
  schema name, or status value.

## Step 4 — Write the artifacts

Specs live at `specs/<REF>/`. Pick `<REF>` in this order: upstream ticket ID (`APP-3892`, `CODE-1822`,
`QUALITY-643`, `REMOTE-1454`) → `GH<N>` for a fork issue → kebab slug for unticketed work
(`tui-viewport`, `mermaid-markdown-in-plans`). Use uppercase `PRODUCT.md` / `TECH.md`; when adding to a
directory that already exists, match the casing of its sibling files.

### PRODUCT.md — the spec

Models: `specs/APP-3892/PRODUCT.md`, `specs/GH11738/product.md`.

1. **Summary** — one paragraph.
2. **Problem** — what breaks, is slow, or is impossible now, with the evidence.
3. **Goals** — bulleted, each observable.
4. **Non-goals** — what this deliberately does not do.
5. **Figma / design references** — link, or "none provided".
6. **User experience** — what the user sees and does, state by state. No implementation.
7. **Success criteria** — a **numbered list**, one testable assertion per line (see below).
8. **Validation** — how a human confirms each criterion holds.
9. **Open questions** — marked `OPEN QUESTION` with an owner, or "None".

**Success criteria are the acceptance contract.** Write each as a single EARS-style assertion — one
`shall`-equivalent claim, an unambiguous subject, and no vague verb ("handle", "support", "improve")
without an observable outcome. Split any line containing two assertions.

**Success criterion N is the `AC-n` that `/tasks`, `/analyze`, and the `validate-implementation` skill
consume.** The numbered list is the on-disk form used across `specs/`; the `AC-n` label is how downstream
commands refer to it. Do not introduce an `AC-` prefix into the file. Some existing specs name this
section **Product invariants** (`specs/GH11738/product.md`) — emit "Success criteria" for new files, and
keep the existing heading when editing a file that already uses the other name.

### TECH.md — the design doc

Models: `specs/APP-3892/TECH.md`, `specs/GH11738/tech.md`. Only for design-bearing items.

1. **Problem / Context** — what exists today and why it does not suffice.
2. **Relevant code** — bulleted `path.rs (lines)` citations, each with what lives there.
3. **Current state** — the behavior and invariants the change has to preserve or break.
4. **Options considered** — at least two, plus "do nothing". For each: sketch, cost, benefit, what it
   forecloses. Be honest about the one you reject.
5. **Decision** — the chosen option and the deciding factor. One paragraph, not a recap of the table.
6. **Proposed changes** — numbered, each naming its files and the concrete edit.
7. **End-to-end flow** — the runtime path once the change lands.
8. **Criteria-to-test map** — a table mapping every `PRODUCT.md` success criterion number to its primary
   coverage. Required, not optional; the model is `specs/GH11738/tech.md`. No orphans in either
   direction — every criterion has coverage, every test traces to a criterion.
9. **Testing and validation** — unit/view tests, GUI integration, manual passes, and the exact commands.
10. **Risks and mitigations** — each with a trigger and a mitigation. "Unknown" is valid only if you name
    who resolves it.
11. **Parallelization** — which numbered changes are independent, and how they split across agents. The
    model is `specs/APP-3892/TECH.md`. Say "serial" explicitly when they do not split.
12. **Follow-ups** — or "None".

### Plan

For Small and Medium items the plan stays in-session: follow `.claude/commands/plan.md` — goal, files to
touch, ordered approach, constitution check, failing test stubs derived from the success criteria,
`/tasks` handoff decision, rollback. Do not write it into `specs/` unless the user asks; `TECH.md` already
carries the design.

## Warp constitution check

Before presenting any artifact, confirm it respects:

- `AGENTS.md` — feature flags, terminal-model locking, exhaustive matching, comment and style conventions.
- `.agent/SAFETY.md` and `.claude/rules/` — secrets, destructive tools, minimal diffs.
- **Front-end scope** — state whether the change is GUI (`app/`), TUI (`crates/warp_tui`), shared
  (`crates/warp_core`, `crates/warpui`), or several. A shared-crate change is design-bearing by default.
- **Verification commands the artifact should name** — `cargo nextest run -p <crate>` for the narrow scope,
  `./script/format`, `cargo clippy --workspace --all-targets --all-features --tests -- -D warnings`, and
  `./script/presubmit` for the full gate.

## Batch mode

Given several items at once (a todo list, an issue sweep), emit the restate + class table for **all** of
them first and get confirmation before writing anything. A twelve-item list must not become twelve specs
without a decision point.

## Output format

```
## Item
<verbatim item> — source: <ref>

## Restated outcome
<one sentence>

## Class: <Trivial | Small | Medium | Design-bearing | Epic>
<one-sentence justification>

## Blocking questions
<at most 5, or "none">

## Assumptions
<explicit list, or "none">

---
<the artifact(s), in order: PRODUCT.md → TECH.md → plan>
---

## Out-of-scope follow-up
<items to file separately, or "none">

## Next command
<one of: /clarify, /plan, /tasks, /decompose, /implement — and why>
```

## Rules

- **No solution in the spec, no requirements in the plan.** "We should use X" in a `PRODUCT.md` belongs in
  `TECH.md`.
- **Sized honestly.** A `TECH.md` for a two-line fix is waste. Downgrade and say so rather than padding.
- **Scope-preserving.** Anything you notice that is out of scope goes in **Out-of-scope follow-up**, never
  into the artifact.
- **Traceable.** Design decision → success criterion → task → test. Nothing orphaned in either direction.
- **Minimal diffs.** The artifact describes the smallest change that satisfies the criteria.

## Done when

- The class is justified and the artifact set matches it — no ceremony, no gaps.
- Every success criterion is one observable numbered assertion; none is ambiguous.
- Every design decision names the option it beat and why.
- The criteria-to-test map covers every criterion with no orphaned tests.
- Another engineer could execute without asking you anything.
- Unknowns appear as `OPEN QUESTION` or **Assumption**, never as confident prose.

## Note

File a backlog issue via the `backlog-queue` skill for anything in **Out-of-scope follow-up** before it is
forgotten. Do not open issues for the item you just drafted if it already has one.
