# rust-journey 🦀

My path through [The Rust Programming Language](https://doc.rust-lang.org/book/).

**Progress site:** https://ahmed-marzook.github.io/rust-journey/

## Layout

```
chapter-01/ … chapter-21/   code + notes for each chapter of the Book
site/                       page template for the progress website
scripts/build-site.mjs      builds the site from the chapter READMEs + git history
```

## Updating progress

It's automatic. Just:

1. Tick sections in the chapter's `README.md` as you finish them: `- [ ]` → `- [x]`
2. Commit with a meaningful message. Commits that touch `chapter-*` become journal entries.
3. Push to `main`. GitHub Actions rebuilds and redeploys the site.

PowerShell:
```powershell
git add -A; git commit -m "Finish guessing game"; git push
```

Bash:
```bash
git add -A && git commit -m "Finish guessing game" && git push
```

Optional: link a section to the book with `- [ ] 2.1 [Title](https://doc.rust-lang.org/book/...)`.

## Preview the site locally

Needs [Node.js](https://nodejs.org/).

PowerShell:
```powershell
node scripts/build-site.mjs; cd _site; python -m http.server 8000   # open http://localhost:8000
```

Bash:
```bash
node scripts/build-site.mjs && cd _site && python3 -m http.server 8000   # open http://localhost:8000
```

## Starting a new chapter project

PowerShell:
```powershell
cd chapter-02; cargo new guessing_game; cd guessing_game; cargo run
```

Bash:
```bash
cd chapter-02 && cargo new guessing_game && cd guessing_game && cargo run
```
