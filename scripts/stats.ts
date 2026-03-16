import { writeFile } from 'node:fs/promises';
import { GITHUB_USERNAME } from '../src/content.js';

type Year = {
  from: string;
  to: string;
  days: number[];
};

type DayData = {
  date: string;
  level: number;
  count: number;
};

// ── Scrape contribution data from GitHub profile ───────────────
// Parses both data-level (for dot colors) and tool-tip text (for exact counts).
// No PAT required — all public data.

async function fetchHTML(url: string): Promise<string> {
  const response = await fetch(url, {
    headers: {
      'User-Agent': `${GITHUB_USERNAME}/readme`,
      'Accept': 'text/html',
    },
  });
  if (!response.ok) {
    throw new Error(`Failed to fetch ${url}: HTTP ${response.status}`);
  }
  return response.text();
}

function parseContributions(html: string): DayData[] {
  // Step 1: Build map of component-id → date + level from <td> elements
  // Pattern: id="contribution-day-component-X-Y" data-date="2026-01-25" ... data-level="2"
  const cellPattern = /id="(contribution-day-component-\d+-\d+)"[^>]*data-date="(\d{4}-\d{2}-\d{2})"[^>]*data-level="(\d)"/g;
  // Also match reversed attribute order (data-date before id)
  const cellPattern2 = /data-date="(\d{4}-\d{2}-\d{2})"[^>]*id="(contribution-day-component-\d+-\d+)"[^>]*data-level="(\d)"/g;

  const cellMap = new Map<string, { date: string; level: number }>();
  let match;

  while ((match = cellPattern.exec(html)) !== null) {
    cellMap.set(match[1], { date: match[2], level: parseInt(match[3], 10) });
  }
  while ((match = cellPattern2.exec(html)) !== null) {
    cellMap.set(match[2], { date: match[1], level: parseInt(match[3], 10) });
  }

  // Step 2: Build map of component-id → count from <tool-tip> elements
  // Pattern: for="contribution-day-component-X-Y" ... >N contributions on ...
  // or: >No contributions on ...
  const tipPattern = /for="(contribution-day-component-\d+-\d+)"[^>]*>(\d+) contributions? on|for="(contribution-day-component-\d+-\d+)"[^>]*>No contributions on/g;
  const countMap = new Map<string, number>();

  while ((match = tipPattern.exec(html)) !== null) {
    if (match[1]) {
      // "N contributions on"
      countMap.set(match[1], parseInt(match[2], 10));
    } else if (match[3]) {
      // "No contributions on"
      countMap.set(match[3], 0);
    }
  }

  // Step 3: Join into day data
  const days: DayData[] = [];
  for (const [id, cell] of cellMap) {
    days.push({
      date: cell.date,
      level: cell.level,
      count: countMap.get(id) ?? 0,
    });
  }

  // Sort by date
  days.sort((a, b) => a.date.localeCompare(b.date));
  return days;
}

function parseHeadingTotal(html: string): number {
  const pattern = /([\d,]+)\s+contributions/;
  const match = html.match(pattern);
  return match ? parseInt(match[1].replace(/,/g, ''), 10) : 0;
}

async function scrapeYear(username: string, year: number) {
  const html = await fetchHTML(
    `https://github.com/users/${username}/contributions?from=${year}-01-01&to=${year}-12-31`
  );

  const allDays = parseContributions(html);
  const headingTotal = parseHeadingTotal(html);

  const today = new Date().toISOString().slice(0, 10);
  const days = allDays.filter(d => d.date.startsWith(`${year}-`) && d.date <= today);

  const summedTotal = days.reduce((sum, d) => sum + d.count, 0);
  const levels = [0, 1, 2, 3, 4].map(l => days.filter(d => d.level === l).length);

  console.log(`    ${year}: ${days.length} days, ${summedTotal} contributions (heading: ${headingTotal}), levels: [${levels.join(', ')}]`);

  return { days, total: headingTotal };
}

// ── Main ───────────────────────────────────────────────────────

const currentYear = new Date().getFullYear();
const START_YEAR = 2024;
const now = new Date();
const today = now.toISOString().slice(0, 10);
const weekStart = new Date(now);
weekStart.setDate(weekStart.getDate() - weekStart.getDay()); // most recent Sunday
const oneWeekAgo = weekStart.toISOString().slice(0, 10);
const oneMonthAgo = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-01`;

console.log(`Scraping contributions for ${GITHUB_USERNAME}...`);

const years: Year[] = [];
let weekCount = 0;
let monthCount = 0;
let yearCount = 0;

for (let y = currentYear; y >= START_YEAR; y--) {
  const { days } = await scrapeYear(GITHUB_USERNAME, y);

  // Level array for the dot grid (reversed = most recent first)
  years.push({
    from: `${y}-01-01T00:00:00.000Z`,
    to: y === currentYear ? now.toISOString() : `${y + 1}-01-01T00:00:00.000Z`,
    days: days.map(d => d.level).reverse(),
  });

  // Exact contribution counts for stats
  for (const { date, count } of days) {
    if (count === 0) continue;
    if (date >= oneWeekAgo && date <= today) weekCount += count;
    if (date >= oneMonthAgo && date <= today) monthCount += count;
    if (date >= `${currentYear}-01-01` && date <= today) yearCount += count;
  }
}

// "Last 12 months" total
const last12Html = await fetchHTML(`https://github.com/users/${GITHUB_USERNAME}/contributions`);
const last12Total = parseHeadingTotal(last12Html);
console.log(`    Last 12 months: ${last12Total}`);

const stats = {
  week: weekCount,
  month: monthCount,
  year: yearCount,
  total: last12Total,
};

console.log(`\n  Summary:`);
console.log(`  Total (last 12mo): ${stats.total.toLocaleString()}`);
console.log(`  This year: ${stats.year}`);
console.log(`  This month: ${stats.month}`);
console.log(`  This week: ${stats.week}`);

await writeFile('src/stats.json', JSON.stringify({ years, stats }));
console.log('  ✓ src/stats.json');