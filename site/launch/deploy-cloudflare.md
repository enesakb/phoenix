# Deploy Phoenix landing site to Cloudflare Pages — step by step

**Cost:** $0 (Cloudflare Pages free tier covers this 100x over)
**Time:** ~5 minutes
**Result:** Public URL like `phoenix-recovery.pages.dev` showing your landing site

---

## Prerequisites

- A Cloudflare account (free) — sign up at cloudflare.com if you don't have one
- The Phoenix repo pushed to GitHub (see `push-to-github.md` first if not done)

---

## Step 1 — Push the repo to GitHub

If not already done:

```powershell
cd C:\Users\enesa\phoenix
gh auth login              # if you don't have GitHub CLI, install: winget install GitHub.cli
gh repo create phoenix --public --source=. --remote=origin --push
```

If `gh` isn't available, do it via the web:

1. Go to github.com → "+" → "New repository"
2. Name: `phoenix`
3. Public
4. **Do NOT** initialize with README (we already have one)
5. After creation, GitHub shows "push existing repo" instructions — copy and paste

---

## Step 2 — Connect Cloudflare Pages to GitHub

1. Go to https://dash.cloudflare.com → **Workers & Pages** → **Create** → **Pages** → **Connect to Git**
2. Click **Authorize Cloudflare** when GitHub asks
3. Pick the **phoenix** repository
4. Click **Begin setup**

---

## Step 3 — Configure the build

In the Cloudflare setup screen:

| Field | Value |
|---|---|
| **Project name** | `phoenix-recovery` (becomes `phoenix-recovery.pages.dev`) |
| **Production branch** | `main` (or `master`, whichever you used) |
| **Framework preset** | None (we're shipping static HTML) |
| **Build command** | (leave empty) |
| **Build output directory** | `site` |
| **Root directory** | (leave empty / `/`) |

Click **Save and Deploy**.

---

## Step 4 — Wait ~30 seconds

Cloudflare clones the repo, copies `site/` to its CDN edge, and gives you the live URL.

You should see:

```
✓ Deployed to:  https://phoenix-recovery.pages.dev
                https://main.phoenix-recovery.pages.dev (preview)
```

Open it in a browser — the public landing page (`site/index.html`) should appear.

---

## Step 5 (optional) — Custom domain

If you want `phoenixrecover.io` or similar:

1. Buy the domain at Cloudflare Registrar (cheapest, ~$10-15/year, free WHOIS privacy)
2. In Cloudflare Pages → your project → **Custom domains** → **Set up a custom domain**
3. Type the domain → Cloudflare auto-configures DNS (you don't need to touch records)
4. SSL/TLS is automatic (Cloudflare-issued cert)

Suggested domain candidates (check availability at cloudflare.com/products/registrar):

- `phoenixrecover.io`
- `phoenix.tools`
- `recoverbip39.io`
- `forensicrecovery.app`
- `bipphoenix.com`

Avoid:
- Anything with "AI" (poisoned category name)
- Anything resembling existing scam domains (check Google for impersonators first)

---

## Step 6 — Wire CI for automatic deploys

This already happens automatically. Every push to `main` triggers a new deploy. Preview branches get their own preview URLs.

To turn this on/off, go to your Pages project → **Settings** → **Builds & deployments**.

---

## Step 7 — Update README links

Once deployed, edit `README.md` to point to the live URL:

```diff
- **Public landing** — open `site/index.html` in a browser
+ **Public landing** — https://phoenix-recovery.pages.dev
```

Commit and push:

```powershell
git add README.md
git commit -m "docs: link to live landing site"
git push
```

CF rebuilds automatically.

---

## Troubleshooting

### "Build failed"
Cloudflare expected a build step. Re-check Step 3 — `Build command` must be empty, `Build output directory` must be `site` (no leading slash).

### "404 on the deployed URL"
The output directory is wrong. Verify `site/index.html` exists in your repo (it does — we just created it). If you renamed `index.html` to something else, change `Build output directory` to match.

### "Old `status.html` showing instead of new landing"
Browser cache. Hard refresh with Ctrl+F5. Cloudflare's edge cache invalidates within ~30s.

### "I want a different URL than `phoenix-recovery.pages.dev`"
Project name controls subdomain. Rename the project in Cloudflare → Settings → General → Project name. Or buy a custom domain (Step 5).

---

## What this gives you

- Public URL anyone can visit
- HTTPS (free, auto-renewed)
- Global CDN (fast worldwide)
- Auto-deploys on every git push
- Preview URL for every PR (great for testing changes)
- $0/month, $0/year, no credit card on file

When you're ready to scale beyond static, Cloudflare Workers + Pages Functions cost ~$5/month at low-six-figure traffic. You won't need that until you have real users.
