import { writeFile, mkdir, access, readdir } from 'node:fs/promises';
import { LINKS } from '../src/content.js';
import data from '../src/stats.json';

type Year = {
  from: string;
  to: string;
  days: number[];
};

const ASSETS = 'assets';

const dotOptions = { rows: 6, size: 24, gap: 5 };
const yearOptions = { gap: 5 };
const MAX_YEARS = 3;

const FONT_REQUIREMENTS = [
  { dir: 'fonts/writer', ext: '.woff2' },
  { dir: 'fonts/departure', ext: '.woff2' },
  { dir: 'fonts/arabic', ext: '.woff2' },
] as const;

async function ensureFontFiles() {
  const missing: string[] = [];

  for (const req of FONT_REQUIREMENTS) {
    try {
      await access(req.dir);
      const files = await readdir(req.dir);
      const match = files.some((f) => f.toLowerCase().endsWith(req.ext));
      if (!match) {
        missing.push(`${req.dir} (expected one ${req.ext} file)`);
      }
    } catch {
      missing.push(`${req.dir} (directory missing)`);
    }
  }

  if (missing.length) {
    throw new Error(
      `Font preflight failed:\n - ${missing.join('\n - ')}\n` +
        'Fix your fonts folder or update src/fonts.ts if structure changed.'
    );
  }
}

async function build() {
  await ensureFontFiles();
  const { link, main, top } = await import('../src/render.js');

  await mkdir(ASSETS, { recursive: true });

  // Top bar: name/title/org
  const topSvg = top({ height: 20 });
  await writeFile(`${ASSETS}/top.svg`, topSvg);
  console.log('  ✓ top.svg');

  // Individual links (clickable)
  for (let i = 0; i < LINKS.length; i++) {
    const l = LINKS[i];
    const linkSvg = link({ height: 17, width: 100, index: i })(l.label);
    await writeFile(`${ASSETS}/link-${l.label}.svg`, linkSvg);
    console.log(`  ✓ link-${l.label}.svg`);
  }

  // Main: bio + stats + contribution graph
  const years = (data.years as Year[]).slice(0, MAX_YEARS);

  const sizes = years.map((year) => {
    const columns = Math.ceil(year.days.length / dotOptions.rows);
    const width = columns * dotOptions.size + (columns - 1) * dotOptions.gap;
    const height = dotOptions.rows * dotOptions.size + (dotOptions.rows - 1) * dotOptions.gap;
    return [width, height];
  });

  const length =
    sizes.reduce((acc, size) => acc + size[0] + yearOptions.gap, 0) - yearOptions.gap;

  const mainSvg = main({
    height: 440,
    years,
    sizes,
    length,
    stats: data.stats as { week: number; month: number; year: number; total: number },
    dots: dotOptions,
    year: yearOptions,
  });
  await writeFile(`${ASSETS}/main.svg`, mainSvg);
  console.log('  ✓ main.svg');

  console.log('\nDone. SVGs written to assets/');
}

build().catch((e) => {
  console.error(e);
  process.exit(1);
});