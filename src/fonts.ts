import { readFileSync, readdirSync } from 'node:fs';

const fontBase64FromDir = (relativeDir: string, extension: string) => {
  const dirUrl = new URL(relativeDir, import.meta.url);
  const file = readdirSync(dirUrl).find((name) =>
    name.toLowerCase().endsWith(extension.toLowerCase())
  );

  if (!file) {
    throw new Error(
      `No ${extension} font found in ${relativeDir}. Check your fonts directory setup.`
    );
  }

  return readFileSync(new URL(`${relativeDir}/${file}`, import.meta.url)).toString('base64');
};

// Use subsetted woff2 files — ~26KB total instead of ~260KB
const WRITER_WOFF2 = fontBase64FromDir('../fonts/writer', '.woff2');
const DEPARTURE_WOFF2 = fontBase64FromDir('../fonts/departure', '.woff2');
const ARABIC_WOFF2 = fontBase64FromDir('../fonts/arabic', '.woff2');

export const fontFaces = /* css */ `
  @font-face {
    font-family: 'Writer';
    src: url(data:font/woff2;base64,${WRITER_WOFF2}) format('woff2');
    font-display: swap;
  }
  @font-face {
    font-family: 'Departure-Mono';
    src: url(data:font/woff2;base64,${DEPARTURE_WOFF2}) format('woff2');
    font-display: swap;
  }
  @font-face {
    font-family: 'Arabic';
    src: url(data:font/woff2;base64,${ARABIC_WOFF2}) format('woff2');
    font-display: swap;
  }
`;
