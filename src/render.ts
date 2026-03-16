import { fontFaces } from './fonts.js';
import { BIO, IDENTITY } from './content.js';

// ── Types ──────────────────────────────────────────────────────

export type Year = {
  from: string;
  to: string;
  days: number[];
};

// ── Color Palette ──────────────────────────────────────────────
// Dark mode — transparent background
// Extracted from PFP + banner images

const C = {
  text: '#c4a478',
  dot0: '#28231e',
  dot1: '#3d3028',
  dot2: '#6b5038',
  dot3: '#9a7850',
  dot4: '#c4a478',
  dotBorder: 'rgba(255,255,255,0.04)',
} as const;

// ── Breakpoints ────────────────────────────────────────────────

const BP_MEDIUM = 550;

// ── SVG Shell ──────────────────────────────────────────────────

interface SvgAttrs { [key: string]: string }

const attr = (obj: SvgAttrs) =>
  Object.entries(obj).reduce((acc, [k, v]) => `${acc} ${k}="${v}"`, '');

const svg = (styles: string, html: string, attributes: SvgAttrs) => {
  if (!attributes.width) attributes.width = '100%';
  return `<svg xmlns="http://www.w3.org/2000/svg" fill="none" ${attr(attributes)}>
    <foreignObject width="100%" height="100%">
      <div xmlns="http://www.w3.org/1999/xhtml">
        <style>${styles}</style>
        ${html}
      </div>
    </foreignObject>
  </svg>`;
};

// ── Shared Styles ──────────────────────────────────────────────

const shared = /* css */ `
  ${fontFaces}

  :root {
    --color-text: ${C.text};
    --color-dot-0: ${C.dot0};
    --color-dot-1: ${C.dot1};
    --color-dot-2: ${C.dot2};
    --color-dot-3: ${C.dot3};
    --color-dot-4: ${C.dot4};
    --color-dot-border: ${C.dotBorder};

    --default-delay: 1s;
    --default-duration: 1.55s;
    --default-stagger: 0.1s;

    --animate-in-links-delay: calc(var(--default-delay) + var(--default-stagger) * 0);
    --animate-in-org-delay: calc(var(--default-delay) + var(--default-stagger) * 2);
    --animate-in-stats-delay: calc(var(--default-delay) + var(--default-stagger) * 4);
    --animate-in-copy-delay: calc(var(--default-delay) + var(--default-stagger) * 5);
    --animate-in-graph-delay: calc(var(--default-delay) + var(--default-stagger) * 17);
  }

  *, *::before, *::after { box-sizing: border-box; }

  .wrapper {
    contain: strict;
    block-size: calc(var(--size-height) * 1px);
    container-type: inline-size;
    position: relative;
    overflow: clip;
    font-family: 'Departure-Mono', monospace;
    color: var(--color-text);
  }

  @-moz-document url-prefix() {
    .wrapper { display: none; }
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(6, 1fr);
  }

  .fade-in {
    will-change: opacity;
    animation-name: fade-in;
    animation-fill-mode: both;
    animation-duration: var(--duration, var(--default-duration));
    animation-timing-function: var(--ease, ease-out);
    animation-delay: var(--delay, var(--default-delay));
  }

  @keyframes fade-in {
    0% { opacity: 0; }
    100% { opacity: 1; }
  }

  .shine {
    background-color: var(--color-text);
    background-image: linear-gradient(-75deg,
      rgba(0,0,0,0) 0%,
      rgba(255,255,255,0.18) 15%,
      rgba(0,0,0,0) 25%
    );
    background-size: 200%;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    animation-name: shine;
    animation-duration: 14s;
    animation-iteration-count: infinite;
  }

  @keyframes shine {
    0% { background-position: 200%; }
    10% { background-position: 0%; }
    100% { background-position: 0%; }
  }

  p { margin: 0; }
`;

// ── Section: Top ───────────────────────────────────────────────
// Links (left) + name/title/org (right)

export const top = (props: { height: number }) => {
  const styles = /* css */ `
    ${shared}
    :root { --size-height: ${props.height}; }

    .wrapper {
      display: flex;
      justify-content: flex-start;
      align-items: flex-start;
      padding: 0 2px;
    }

    .orginfo {
      --delay: var(--animate-in-org-delay);
      text-align: left;
      line-height: 17px;
    }
    .orginfo-row {
      display: flex;
      align-items: baseline;
      justify-content: flex-start;
      gap: 5px;
    }
    .orginfo-label {
      font-family: 'Departure-Mono', monospace;
      font-size: 9px;
      opacity: 0.4;
    }
    .orginfo-val {
      font-family: 'Writer', Georgia, serif;
      font-size: 12px;
    }
    .orginfo-dot {
      width: 3px; height: 3px;
      display: inline-block;
      margin-right: 2px;
      vertical-align: middle;
      background: var(--color-text);
    }
    .arabic { font-family: 'Arabic', serif; }
  `;

  const html = /* html */ `
    <div class="wrapper">
      <div class="orginfo fade-in">
        <div class="orginfo-row">
          <span class="orginfo-label"><span class="orginfo-dot"></span>name</span>
          <span class="orginfo-val">${IDENTITY.name} <span class="arabic">⌊${IDENTITY.nameArabic}⌋</span></span>
        </div>
        <div class="orginfo-row">
          <span class="orginfo-label"><span class="orginfo-dot"></span>title</span>
          <span class="orginfo-val">${IDENTITY.title}</span>
        </div>
        <div class="orginfo-row">
          <span class="orginfo-label"><span class="orginfo-dot"></span>org</span>
          <span class="orginfo-val">${IDENTITY.org}</span>
        </div>
      </div>
    </div>
  `;

  return svg(styles, html, { height: `${props.height}` });
};

// ── Section: Link (individual clickable) ───────────────────────

export const link = (props: { height: number; width: number; index: number }) => (label: string) => {
  const styles = /* css */ `
    ${shared}
    :root {
      --size-height: ${props.height};
      --size-width: ${props.width};
      --i: ${props.index};
    }

    .wrapper {
      --delay: calc(var(--animate-in-links-delay) + var(--i) * 1.2s);
    }
    @-moz-document url-prefix() {
      .wrapper { display: block; }
    }

    .link {
      font-family: 'Writer', Georgia, serif;
      font-size: 12px;
      display: flex;
      justify-content: start;
      align-items: center;
      gap: 2px;
    }
    .link__arrow {
      font-size: 9px;
      position: relative;
      inset-block-start: 0.5px;
      animation-name: rotate;
      animation-duration: 5s;
      animation-timing-function: ease-in-out;
      animation-iteration-count: infinite;
      animation-delay: ${(Math.random() * 5).toFixed(1)}s;
    }
    @keyframes rotate {
      0% { transform: rotate(0deg); }
      10%, 100% { transform: rotate(360deg); }
    }
  `;

  const html = /* html */ `
    <main class="wrapper">
      <a class="link fade-in">
        <div class="link__label shine" style="animation-delay: ${(Math.random() * 10).toFixed(1)}s">${label}</div>
        <div class="link__arrow">↗</div>
      </a>
    </main>
  `;

  return svg(styles, html, {
    width: `${props.width}`,
    height: `${props.height}`,
  });
};

// ── Section: Main ──────────────────────────────────────────────

export type MainProps = {
  height: number;
  years: Year[];
  sizes: number[][];
  length: number;
  stats: { week: number; month: number; year: number; total: number };
  dots: { rows: number; size: number; gap: number };
  year: { gap: number };
};

export const main = (props: MainProps) => {
  const styles = /* css */ `
    ${shared}

    :root {
      --rows: ${props.dots.rows};
      --size-width: 100cqw;
      --size-height: ${props.height};
      --size-dot-gap: ${props.dots.gap};
      --size-dot: ${props.dots.size};
      --size-year-gap: ${props.year.gap};
      --size-label-height: 20;
    }

    .wrapper {
      display: grid;
      grid-template-columns: repeat(6, 1fr);
      grid-template-rows: auto auto;
      padding: 0 2px 4px;
    }

    /* ── Contribution stats ── */
    .stats {
      --delay: var(--animate-in-stats-delay);
      grid-column: 1 / 3;
      grid-row: 1;
      align-self: start;
      padding-bottom: 6px;
      font-family: 'Departure-Mono', monospace;
      font-size: 11px;
      line-height: 17px;
    }
    .stats-title {
      font-size: 9px;
      opacity: 0.4;
      margin-bottom: 3px;
      display: flex;
      align-items: center;
      gap: 3px;
    }
    .stats-dot {
      width: 3px; height: 3px;
      background: var(--color-text);
      display: inline-block;
    }
    .stat-row {
      display: flex;
      align-items: baseline;
      justify-content: space-between;
    }
    .stat-num { font-weight: 500; flex-shrink: 0; }
    .stat-line {
      flex: 1;
      margin: 0 6px;
      border-bottom: 0.5px solid var(--color-text);
      opacity: 0.12;
      align-self: baseline;
      margin-bottom: 3px;
    }
    .stat-label { opacity: 0.4; font-size: 9px; flex-shrink: 0; }

    /* ── Bio ── */
    .intro {
      grid-column: 4 / 7;
      grid-row: 1;
      font-family: 'Writer', Georgia, serif;
      font-size: 13px;
      font-weight: 300;
      line-height: 1.55;
      align-self: start;
      padding-bottom: 6px;
    }
    .intro span {
      contain: content;
      --duration: 980ms;
      --delay: calc(var(--animate-in-copy-delay) + var(--i) * 5ms);
    }

    @container (width > ${BP_MEDIUM}px) {
      .intro { font-size: 14px; }
    }

    /* ── Graph ── */
    .graph {
      --delay: var(--animate-in-graph-delay);
      grid-column: 1 / 7;
      grid-row: 2;
    }
    .years {
      --_w: var(--w);
      --_h: calc(var(--h) + var(--size-label-height));
      display: flex;
      gap: calc(var(--size-year-gap) * 1px);
      contain: strict;
      inline-size: calc(var(--_w) * 1px);
      block-size: calc(var(--_h) * 1px);
      will-change: transform;
      backface-visibility: hidden;
      transform: translateZ(0);
      animation-name: scroll, fade-in;
      animation-timing-function: linear, ease-out;
      animation-duration: calc(30s + (var(--_w) * 0.06s)), 2.5s;
      animation-fill-mode: both, both;
      animation-delay: 2s, var(--animate-in-graph-delay);
    }
    @keyframes scroll {
      0% { transform: translateX(0); }
      100% { transform: translateX(calc(-100% + 100cqw)); }
    }
    .year {
      contain: strict;
      content-visibility: auto;
      inline-size: calc(var(--w) * 1px);
      block-size: calc(var(--_h) * 1px);
    }
    .year__label {
      contain: strict;
      block-size: calc(var(--size-label-height) * 1px);
      content-visibility: auto;
      display: flex;
      align-items: end;
      font-size: 9px;
      opacity: 0.35;
    }
    .year__days {
      display: grid;
      grid-auto-flow: column;
      grid-template-rows: repeat(var(--rows), calc(var(--size-dot) * 1px));
      grid-auto-columns: calc(var(--size-dot) * 1px);
      gap: calc(var(--size-dot-gap) * 1px);
      contain: strict;
      content-visibility: auto;
      inline-size: calc(var(--w) * 1px);
      block-size: calc(var(--h) * 1px);
    }
    .year__days .dot {
      contain: strict;
      content-visibility: auto;
      aspect-ratio: 1;
      inline-size: calc(var(--size-dot) * 1px);
      block-size: calc(var(--size-dot) * 1px);
      border: calc(var(--size-dot) * 0.04 * 1px) solid var(--color-dot-border);
      border-radius: calc(var(--size-dot) * 0.12 * 1px);
    }
    .dot--0 { background-color: var(--color-dot-0); }
    .dot--1 { background-color: var(--color-dot-1); }
    .dot--2 { background-color: var(--color-dot-2); }
    .dot--3 { background-color: var(--color-dot-3); }
    .dot--4 {
      background-color: var(--color-dot-4);
      animation: pulse 4s ease-in-out infinite;
      animation-delay: var(--pd, 0s);
    }
    @keyframes pulse {
      0%, 100% { opacity: 1; }
      50% { opacity: 0.65; }
    }
  `;

  const format = (date: Date) =>
    date.toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' });

  const dateLabel = (i: number) =>
    i === 0 ? format(new Date()) : new Date(props.years[i].from).getFullYear();

  const days = (d: Year['days']) =>
    d.map((level) => {
      const pd = level === 4 ? ` style="--pd: ${(Math.random() * 4).toFixed(2)}s"` : '';
      return `<div class="dot dot--${level}"${pd}></div>`;
    }).join('');

  const fmt = (n: number) => n.toLocaleString('en-US');

  // Character-by-character fade-in
  let charIdx = 0;
  const bioChars = BIO.split('\n').map((line, li, arr) => {
    if (line === '') return '<br/>';
    const spans = line.split('').map((c) => {
      const ch = c === ' ' ? '&#160;' : c;
      const s = `<span class="fade-in" style="--i: ${charIdx};">${ch}</span>`;
      charIdx++;
      return s;
    }).join('');
    return spans + (li < arr.length - 1 ? '<br/>' : '');
  }).join('');

  const html = /* html */ `
    <main class="wrapper">
      <article class="stats fade-in">
        <div class="stats-title"><span class="stats-dot"></span>contributions</div>
        <div class="stat-row">
          <span class="stat-num shine" style="animation-delay: 7s">${fmt(props.stats.week)}</span>
          <span class="stat-line"></span>
          <span class="stat-label">this week</span>
        </div>
        <div class="stat-row">
          <span class="stat-num shine" style="animation-delay: 8.5s">${fmt(props.stats.month)}</span>
          <span class="stat-line"></span>
          <span class="stat-label">this month</span>
        </div>
        <div class="stat-row">
          <span class="stat-num shine" style="animation-delay: 10s">${fmt(props.stats.year)}</span>
          <span class="stat-line"></span>
          <span class="stat-label">this year</span>
        </div>
        <div class="stat-row">
          <span class="stat-num shine" style="animation-delay: 11.5s">${fmt(props.stats.total)}</span>
          <span class="stat-line"></span>
          <span class="stat-label">all time</span>
        </div>
      </article>

      <article class="intro">
        <p>${bioChars}</p>
      </article>

      <article class="graph">
        <div class="years" style="--w: ${props.length}; --h: ${props.sizes[0][1]};">
          ${props.years.map((year, i) => /* html */ `
            <div class="year year--${i}" style="--w: ${props.sizes[i][0]}; --h: ${props.sizes[i][1]};">
              <div class="year__days">${days(year.days)}</div>
              <div class="year__label"><span>${dateLabel(i)}</span></div>
            </div>
          `).join('')}
        </div>
      </article>
    </main>
  `;

  return svg(styles, html, { height: `${props.height}` });
};

// ── Section: Fallback (Firefox) ────────────────────────────────

export const fallback = (props: { height: number; width: number }) => {
  const styles = /* css */ `
    ${shared}
    :root { --size-height: ${props.height}; --size-width: ${props.width}; }

    .wrapper { display: none; }
    @-moz-document url-prefix() {
      .wrapper { display: flex; align-items: end; }
    }
    .intro {
      font-family: 'Writer', Georgia, serif;
      font-size: 18px;
      font-weight: 300;
      line-height: 1.55;
    }
    .intro span {
      contain: content;
      --duration: 980ms;
      --delay: calc(var(--animate-in-copy-delay) + var(--i) * 10ms);
    }
    .hint {
      --duration: 1.2s;
      --delay: calc(var(--animate-in-copy-delay) + 2.5s);
      margin-block-start: 10px;
      font-size: 10px;
      font-style: italic;
    }
  `;

  let charIdx = 0;
  const bioChars = BIO.split('').map((c) => {
    if (c === '\n') return '<br/>';
    const ch = c === ' ' ? '&#160;' : c;
    return `<span class="fade-in" style="--i: ${charIdx++};">${ch}</span>`;
  }).join('');

  const html = /* html */ `
    <main class="wrapper">
      <div class="intro">
        <p>${bioChars}</p>
        <p class="hint fade-in">— try Chrome/Safari for the full experience!</p>
      </div>
    </main>
  `;

  return svg(styles, html, {
    width: `${props.width}`,
    height: `${props.height}`,
    viewBox: `0 0 ${props.width} ${props.height}`,
  });
};
