---
title: Native Warp editor — Zed × Warp Hybrid
description: Warp’s embedded coding surface. Why Level 3 should not transplant Zed’s editor into Warp.
---

# Native Warp editor

← [Zed × Warp](../README.md)

Warp is no longer “terminal only.” It has an embedded coding surface for agents and review. That is why [Level 3](../level-3.md) should **not** transplant Zed’s editor into Warp.

## Where it lives `(Warp)`

| Layer | Path |
|-------|------|
| Buffer / rich-text core | `crates/editor/` (`warp_editor`) |
| Prompt input editor (not the IDE) | `app/src/editor/` |
| Native **code** editor UI | `app/src/code/` |
| Tree-sitter languages | `crates/languages/` (~35 grammars) |
| LSP client | `crates/lsp/` — rust-analyzer, gopls, pyright, TypeScript LS, clangd |
| Vim | `crates/vim/` |
| Code review / git | `app/src/code_review/` |
| Open in Zed/VS Code/… | [External editor](external-editor.md) |

Two different editor stacks: prompt `app/src/editor` vs code `app/src/code`. Both are WarpUI-shaped.

Present: multi-file buffers, highlighting, hover/goto/references/format, find/replace, vim, diffs, review comments.

Missing vs Zed: debugger, tasks, deep multi-root project model, broad LSP ecosystem.

**Verdict:** solid agentic-IDE surface, not a drop-in Zed replacement. Keep Zed as the serious editor ([Level 1](../level-1.md)).

## See also

- [Level 3](../level-3.md)
- [What not to build](../what-not-to-build.md)
