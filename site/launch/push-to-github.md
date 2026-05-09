# Push Phoenix to GitHub — step by step

**Cost:** $0
**Time:** ~3 minutes
**Result:** Public repo at `github.com/enesakb/phoenix`

---

## Method A — using GitHub CLI (`gh`) — recommended

### Step 1 — Install `gh` if you don't have it

```powershell
winget install GitHub.cli
```

After install, restart your PowerShell window so the new PATH takes effect.

### Step 2 — Authenticate

```powershell
gh auth login
```

Pick:
- **GitHub.com**
- **HTTPS**
- **Login with a web browser**

A code appears; copy it. Browser opens to github.com — paste the code, approve.

### Step 3 — Create the repo and push

```powershell
cd C:\Users\enesa\phoenix
gh repo create phoenix --public --source=. --remote=origin --push --description "Open-source forensic crypto wallet recovery"
```

That's it. The repo is at `github.com/enesakb/phoenix`. All commits are pushed. The five sprint tags (`week-1-foundation` through `week-4-6-extended`) are pushed too.

### Step 4 — Verify

```powershell
gh repo view --web
```

Browser opens to your repo. Check:
- README renders correctly
- Tags appear under "Releases"
- All commits are visible

---

## Method B — via the GitHub website (no CLI)

### Step 1 — Create empty repo on GitHub

1. Go to https://github.com → click "+" (top right) → **New repository**
2. **Repository name:** `phoenix`
3. **Description:** `Open-source forensic crypto wallet recovery`
4. **Public**
5. **Do NOT** check "Initialize with README" — we already have one
6. **Create repository**

### Step 2 — Push from your machine

GitHub shows two boxes after creation. Use the **"…or push an existing repository from the command line"** box:

```powershell
cd C:\Users\enesa\phoenix
git remote add origin https://github.com/enesakb/phoenix.git
git branch -M main
git push -u origin main
```

When prompted for credentials, use:
- **Username:** your GitHub username
- **Password:** a GitHub Personal Access Token (NOT your password)

To create a PAT:
1. github.com → settings → Developer settings → Personal access tokens → Tokens (classic) → Generate new token
2. Scope: just `repo`
3. Copy the token (starts with `ghp_`)
4. Use it as your "password" when git prompts

### Step 3 — Push tags too

```powershell
git push origin --tags
```

This pushes all five sprint tags so they appear under Releases.

---

## After pushing — turn on the right settings

### Make releases visible

1. github.com/enesakb/phoenix → **Releases** (right sidebar)
2. Click **Create a new release**
3. **Choose a tag** → `week-4-6-extended` (the latest)
4. **Release title:** `v0.5 — multi-word recovery + Hashcat builder`
5. **Description:** copy from the commit message body
6. Mark as **Pre-release**
7. **Publish release**

Repeat for `week-5-crypto-reconstruct` (this is the headline release — the one that actually recovers wallets in 72ms).

### Add topics for discoverability

1. Repo home → click the gear icon next to "About"
2. Topics: `wallet-recovery`, `bip39`, `bitcoin`, `ethereum`, `cryptocurrency`, `rust`, `tauri`, `forensics`, `open-source`
3. Website: `https://phoenix-recovery.pages.dev` (after Cloudflare deploy)
4. **Save changes**

### Enable Issues, Disable Wiki

- Settings → Features → Issues: ✓ on
- Settings → Features → Wiki: ☐ off (keep docs in repo)
- Settings → Features → Discussions: ✓ on (community Q&A)

### Branch protection (lightweight)

Settings → Branches → Add rule:
- Branch name: `main`
- Require pull request reviews before merging — ☐ (skip for solo dev)
- Require status checks to pass — ✓ (CI must be green)

---

## Troubleshooting

### "Authentication failed" on push
You used your password. GitHub no longer accepts passwords for git over HTTPS. Use a Personal Access Token (Method B Step 2 above).

### "Updates were rejected"
You probably created the repo with "Initialize with README" checked. Either:
- Delete the GitHub repo and recreate without README, OR
- Run `git pull origin main --allow-unrelated-histories`, resolve any conflicts, then `git push`

### "Repository name already taken"
Someone else has `enesakb/phoenix` (probably you, from a prior attempt). Either delete the old repo or pick a different name (e.g. `phoenix-recovery`).

### "Tags not showing in Releases"
Tags by default don't auto-create releases. Run Step 3 of Method B (`git push origin --tags`), then create the Releases manually as in "Make releases visible" above.

---

## What pushing gives you

- Public URL for anyone to see, fork, audit, contribute
- Required for Cloudflare Pages auto-deploy (next doc: `deploy-cloudflare.md`)
- Automatic CI runs (already configured in `.github/workflows/`)
- Star-count signal for HN/Reddit posts ("X stars on GitHub" makes a post 3x more credible)
- Collaboration target (issues, PRs, security disclosures)

Cost stays $0 indefinitely for public repos.
