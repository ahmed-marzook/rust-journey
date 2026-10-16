// Builds the progress site into _site/.
//
// Progress comes from the checklists in chapter-XX/README.md:
//   # Chapter 2: Programming a Guessing Game
//   📖 https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
//   - [x] 2.1 Some section          ← done
//   - [ ] 2.2 [Another](https://…)  ← not done (optional link)
//
// The journal and start date come from commits that touch chapter-* folders.
//
// Run locally:  node scripts/build-site.mjs

import { readdirSync, readFileSync, existsSync, mkdirSync, cpSync, writeFileSync, rmSync } from "node:fs";
import { execSync } from "node:child_process";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("..", import.meta.url));
const out = join(root, "_site");

const chapters = readdirSync(root)
  .filter(d => /^chapter-\d+$/.test(d) && existsSync(join(root, d, "README.md")))
  .sort()
  .map(dir => {
    const md = readFileSync(join(root, dir, "README.md"), "utf8");
    const num = Number(dir.split("-")[1]);
    const title = md.match(/^#\s+Chapter\s+\d+:\s*(.+)$/m)?.[1].trim() ?? `Chapter ${num}`;
    const url = md.match(/https:\/\/doc\.rust-lang\.org\/book\/\S+/)?.[0] ?? null;
    const sections = [...md.matchAll(/^\s*[-*]\s+\[([ xX])\]\s+(\d+(?:\.\d+)*)\s+(.+?)\s*$/gm)].map(m => {
      const link = m[3].match(/^\[(.+)\]\((.+)\)$/);
      return { num: m[2], title: link ? link[1] : m[3], url: link ? link[2] : null, done: m[1] !== " " };
    });
    return { num, title, url, folder: dir, sections };
  });

// The first unticked section is where you are now.
const current = chapters.flatMap(c => c.sections).find(s => !s.done);
if (current) current.doing = true;

const git = cmd => { try { return execSync(`git ${cmd}`, { cwd: root, encoding: "utf8" }).trim(); } catch { return ""; } };
const journal = git(`log --no-merges --date=short --pretty=format:%ad%x09%s -n 100 -- "chapter-*"`)
  .split("\n").filter(Boolean)
  .map(line => { const [date, ...text] = line.split("\t"); return { date, text: text.join("\t") }; });
const started = git(`log --reverse --date=short --pretty=format:%ad -- "chapter-*"`).split("\n")[0] || null;
const repo = (process.env.GITHUB_REPOSITORY && `https://github.com/${process.env.GITHUB_REPOSITORY}`)
  || git("remote get-url origin").replace(/\.git$/, "").replace(/^git@github\.com:/, "https://github.com/").replace(/^https?:\/\/[^/]*\/git\//, "https://github.com/")
  || "https://github.com/ahmed-marzook/rust-journey";

rmSync(out, { recursive: true, force: true });
mkdirSync(out);
cpSync(join(root, "site"), out, { recursive: true });
writeFileSync(join(out, "data.js"),
  `const DATA = ${JSON.stringify({ repo, started, chapters, journal }, null, 2)};\n`);

const all = chapters.flatMap(c => c.sections);
console.log(`Built _site/: ${all.filter(s => s.done).length}/${all.length} sections done, ${journal.length} journal entries.`);
