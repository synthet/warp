# GitHub Actions (synthet/warp)

This fork keeps a **single** workflow: [`ci.yml`](./ci.yml) (`Warp CI`). It runs on
`github.repository_owner == 'synthet'` using GitHub-hosted runners (`ubuntu-latest`,
`macos-14`).

Upstream Warp workflows (release cutting, repo-sync, Oz/docubot automation, Docker Hub
agent-dev image, warpdotdev/docs dispatch, etc.) are **not** carried in this fork. They
depended on Warp-internal secrets, Namespace runners, GCP, and `warpdotdev/*` reusable
workflows. Removing them avoids skipped scheduled runs, failing `update-*-local` jobs, and
noisy `notify-docs` / `publish-agent-dev-image` failures on every `master` push.

When merging `warpdotdev/warp` `master`, expect `.github/workflows/` to gain upstream files
again. Reconcile by keeping `ci.yml` fork overrides (owner + runners, no repo-sync lint) and
deleting reintroduced upstream-only workflows unless you intentionally adopt them.
