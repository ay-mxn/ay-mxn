import { writeFile } from 'node:fs/promises';
import { GITHUB_USERNAME } from '../src/content.js';

type Year = {
  from: string;
  to: string;
  days: number[];
};

// ── Scrape contribution data from GitHub profile ───────────────
// This approach reads the same data displayed on your profile page,
// which includes org/EMU contributions that the GraphQL API can't access.

async function scrapeContributions(username: string, year: number): Promise<{ days: number[]; total: number }> {
  // GitHub serves contribution data at this URL as HTML with embedded tooltip data
  const url = `https://github.com/users/${username}/contributions?from=${year}-01-01&to=${year}-12-31`;
  
  const response = await fetch(url, {
    headers: {
      'User-Agent': `${username}/readme`,
      'Accept': 'text/html',
    },
  });

  if (!response.ok) {
    throw new Error(`Failed to fetch contributions for ${year}: HTTP ${response.status}`);
  }

  const html = await response.text();

  // Parse contribution levels from the HTML
  // GitHub renders <td> elements with data-level="0-4" for each day
  const dayPattern = /data-level="(\d)"/g;
  const days: number[] = [];
  let match;
  while ((match = dayPattern.exec(html)) !== null) {
    days.push(parseInt(match[1], 10));
  }

  // Parse total from the heading like "1,829 contributions in the last year"
  // or "X contributions in YYYY"
  const totalPattern = /([\d,]+)\s+contributions/;
  const totalMatch = html.match(totalPattern);
  const total = totalMatch ? parseInt(totalMatch[1].replace(/,/g, ''), 10) : 0;

  console.log(`    ${year}: ${days.length} days, ${total} contributions, levels: [${[0,1,2,3,4].map(l => days.filter(d => d === l).length).join(', ')}]`);

  return { days: days.reverse(), total };
}

async function scrapeCurrentYear(username: string): Promise<{ days: number[]; total: number }> {
  // For the current partial year, fetch without date range to get "last 12 months" view
  const url = `https://github.com/${username}`;
  
  const response = await fetch(url, {
    headers: {
      'User-Agent': `${username}/readme`,
      'Accept': 'text/html',
    },
  });

  if (!response.ok) {
    throw new Error(`Failed to fetch profile: HTTP ${response.status}`);
  }

  const html = await response.text();

  const totalPattern = /([\d,]+)\s+contributions?\s+in\s+the\s+last\s+year/;
  const totalMatch = html.match(totalPattern);
  const total = totalMatch ? parseInt(totalMatch[1].replace(/,/g, ''), 10) : 0;

  return { days: [], total };
}

// ── Main ───────────────────────────────────────────────────────

const currentYear = new Date().getFullYear();
const START_YEAR = 2024;

console.log(`Scraping contributions for ${GITHUB_USERNAME}...`);

const years: Year[] = [];
let grandTotal = 0;

for (let y = currentYear; y >= START_YEAR; y--) {
  const fromDate = `${y}-01-01T00:00:00.000Z`;
  const toDate = y === currentYear
    ? new Date().toISOString()
    : `${y + 1}-01-01T00:00:00.000Z`;

  const { days, total } = await scrapeContributions(GITHUB_USERNAME, y);
  
  years.push({
    from: fromDate,
    to: toDate,
    days,
  });
  grandTotal += total;
}

// Get the "last 12 months" total for the all-time display
const { total: last12Months } = await scrapeCurrentYear(GITHUB_USERNAME);
console.log(`\n  Last 12 months (from profile): ${last12Months}`);

// Compute stats from scraped data
const now = new Date();
const allDays: { date: Date; level: number }[] = [];

for (const y of years) {
  const startDate = new Date(y.from);
  // days are reversed, so index 0 = most recent
  const reversedDays = [...y.days].reverse();
  for (let i = 0; i < reversedDays.length; i++) {
    const date = new Date(startDate);
    date.setDate(date.getDate() + i);
    allDays.push({ date, level: reversedDays[i] });
  }
}

const oneWeekAgo = new Date(now.getTime() - 7 * 24 * 60 * 60 * 1000);
const oneMonthAgo = new Date(now.getFullYear(), now.getMonth() - 1, now.getDate());
const startOfYear = new Date(now.getFullYear(), 0, 1);

let weekCount = 0;
let monthCount = 0;
let yearCount = 0;

for (const { date, level } of allDays) {
  if (level === 0) continue;
  if (date >= oneWeekAgo) weekCount++;
  if (date >= oneMonthAgo) monthCount++;
  if (date >= startOfYear) yearCount++;
}

const stats = {
  week: weekCount,
  month: monthCount,
  year: yearCount,
  total: last12Months || grandTotal,
};

console.log(`\n  Summary:`);
console.log(`  Total: ${stats.total.toLocaleString()}`);
console.log(`  This week: ${stats.week} | month: ${stats.month} | year: ${stats.year}`);

await writeFile('src/stats.json', JSON.stringify({ years, stats }));
console.log('  ✓ src/stats.json');