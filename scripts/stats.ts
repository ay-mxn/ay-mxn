import { writeFile } from 'node:fs/promises';
import { GITHUB_USERNAME } from '../src/content.js';

type Year = {
  from: string;
  to: string;
  days: number[];
};

type Contribution = {
  contributionCount: number;
  date: string;
  contributionLevel:
    | 'NONE'
    | 'FIRST_QUARTILE'
    | 'SECOND_QUARTILE'
    | 'THIRD_QUARTILE'
    | 'FOURTH_QUARTILE';
};

type GHResponse = {
  data: {
    viewer: {
      contributionsCollection: {
        contributionCalendar: {
          totalContributions: number;
          weeks: {
            contributionDays: Contribution[];
          }[];
        };
      };
    };
  };
  errors?: { message: string }[];
};

async function request(date: { from?: Date; to?: Date }) {
  const body = {
    // Use `viewer` instead of `user(login:)` — this returns the authenticated
    // user's own data, which includes private and org contributions automatically.
    // Requires a CLASSIC PAT with `read:user` scope (fine-grained PATs don't support GraphQL).
    query: `query ($from: DateTime, $to: DateTime) {
      viewer {
        contributionsCollection(from: $from, to: $to) {
          contributionCalendar {
            totalContributions
            weeks {
              contributionDays {
                contributionCount
                date
                contributionLevel
              }
            }
          }
        }
      }
    }`,
    variables: {
      from: date.from?.toISOString(),
      to: date.to?.toISOString(),
    },
  };

  const response = await fetch('https://api.github.com/graphql', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'User-Agent': `${GITHUB_USERNAME}/readme`,
      Authorization: `bearer ${process.env.API_TOKEN_GITHUB}`,
    },
    body: JSON.stringify(body),
  }).then((res) => res.json() as Promise<GHResponse>);

  if (response.errors?.length) {
    throw new Error(`GitHub GraphQL error: ${response.errors.map((e) => e.message).join('; ')}`);
  }

  const calendar = response.data.viewer.contributionsCollection.contributionCalendar;
  return { weeks: calendar.weeks, contributions: calendar.totalContributions };
}

const levelToInt = (level: Contribution['contributionLevel']) => {
  switch (level) {
    case 'NONE': return 0;
    case 'FIRST_QUARTILE': return 1;
    case 'SECOND_QUARTILE': return 2;
    case 'THIRD_QUARTILE': return 3;
    case 'FOURTH_QUARTILE': return 4;
  }
};

async function getAllContributions(start: Date, end = new Date()) {
  const years: Year[] = [];
  let cursor = start;
  let totalContributions = 0;

  while (cursor < end) {
    let next = new Date(cursor.getFullYear() + 1, 0, 1);
    if (next > end) next = end;
    console.info('  Fetching', cursor.toISOString().slice(0, 10), '→', next.toISOString().slice(0, 10));

    const data = await request({ to: next, from: cursor });
    totalContributions += data.contributions;
    years.push({
      from: cursor.toISOString(),
      to: next.toISOString(),
      days: data.weeks
        .flatMap((week) =>
          week.contributionDays.map((day) => levelToInt(day.contributionLevel))
        )
        .reverse(),
    });
    cursor = next;
  }
  return { years: years.reverse(), totalContributions };
}

function computeStats(years: Year[], totalContributions: number) {
  // Flatten all days with dates
  const now = new Date();
  const oneWeekAgo = new Date(now.getTime() - 7 * 24 * 60 * 60 * 1000);
  const oneMonthAgo = new Date(now.getFullYear(), now.getMonth() - 1, now.getDate());
  const startOfYear = new Date(now.getFullYear(), 0, 1);

  let weekCount = 0;
  let monthCount = 0;
  let yearCount = 0;

  for (const y of years) {
    const startDate = new Date(y.from);
    for (let i = 0; i < y.days.length; i++) {
      if (y.days[i] === 0) continue;
      const date = new Date(startDate);
      date.setDate(date.getDate() + i);
      if (date >= oneWeekAgo) weekCount++;
      if (date >= oneMonthAgo) monthCount++;
      if (date >= startOfYear) yearCount++;
    }
  }

  return { week: weekCount, month: monthCount, year: yearCount, total: totalContributions };
}

// ── Main ───────────────────────────────────────────────────────

const START_DATE = new Date('2023-01-01T00:00:00.000Z');

console.log('Fetching contributions...');
const { years, totalContributions } = await getAllContributions(START_DATE);
const stats = computeStats(years, totalContributions);

console.log(`  Total: ${totalContributions.toLocaleString()}`);
console.log(`  This week: ${stats.week} | month: ${stats.month} | year: ${stats.year}`);

await writeFile('src/stats.json', JSON.stringify({ years, stats }));
console.log('  ✓ src/stats.json');