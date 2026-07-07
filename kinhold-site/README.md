# KINHOLD.IO site (recovered static build)

This folder contains the recovered, production-ready static build of the
KINHOLD.IO "Trust Operating System" marketing/commerce site, migrated off the
Replit deployment (`stripe-linker-timlippy38.replit.app`).

## What it is

- A **static Vite/React single-page app** (prebuilt bundle in `assets/`).
- Purchases use **Stripe Payment Links** (`buy.stripe.com/...`) — a pure
  client-side redirect, so **no backend or Stripe secret key is required** to
  host this site. Checkout works from any static host.
- The `/api/*` paths referenced in the bundle are not implemented on the origin
  (they return 404), so nothing server-side is needed.

> Note: the current payment links are Stripe **test-mode** links. If a link
> shows "page not found", activate/replace the Payment Links in the Stripe
> dashboard — this is unrelated to hosting.

## Layout

- `index.html` — app entry
- `assets/` — hashed JS/CSS bundle
- `favicon.svg`, `robots.txt`
- `404.html` — SPA fallback for GitHub Pages
- `_redirects` — SPA fallback for Cloudflare Pages (`/* /index.html 200`)
- `wrangler.toml` — Cloudflare Pages project config

## Deploy (free hosts)

### Cloudflare Pages (recommended)
```bash
cd kinhold-site
npx wrangler pages deploy . --project-name kinhold-site
```
Then add `kinhold.io` + `www.kinhold.io` as custom domains on the Pages project
and point DNS (via Cloudflare, keeping GoDaddy as registrar).

### GitHub Pages
Push this folder to a repo's Pages source; `404.html` provides SPA routing.
Add a `CNAME` file containing `kinhold.io` and set the A/AAAA (or CNAME) records
at the DNS provider.

## Local preview
```bash
cd kinhold-site
python3 -m http.server 8099
# open http://localhost:8099/
```
