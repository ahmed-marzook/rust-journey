# rust-journey 🦀

My path through [The Rust Programming Language](https://doc.rust-lang.org/book/).

**Progress site:** https://ahmed-marzook.github.io/rust-journey/

## Layout

```
chapter-01/ … chapter-21/   code + notes for each chapter of the Book
site/                       the progress website (deployed to GitHub Pages)
  progress.js               ← edit this to update progress
```

## Updating progress

1. In `site/progress.js`, change a section's status to `"done"`, `"doing"` or `"todo"`.
2. Add a line to `JOURNAL` (newest first).
3. Commit and push to `main`. The site redeploys on its own.

## Preview the site locally

PowerShell:
```powershell
cd site; python -m http.server 8000   # then open http://localhost:8000
# or just: Start-Process .\site\index.html
```

Bash:
```bash
cd site && python3 -m http.server 8000   # then open http://localhost:8000
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
