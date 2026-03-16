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

const WRITER_OTF = fontBase64FromDir('../fonts/writer', '.otf');
const DEPARTURE_WOFF = fontBase64FromDir('../fonts/departure', '.woff');
const ARABIC_TTF = fontBase64FromDir('../fonts/arabic', '.ttf');

export const fontFaces = /* css */ `
  @font-face {
    font-family: 'Writer';
    src: url(data:font/otf;base64,${WRITER_OTF}) format('opentype');
    font-display: swap;
  }
  @font-face {
    font-family: 'Departure-Mono';
    src: url(data:font/woff;base64,${DEPARTURE_WOFF}) format('woff');
    font-display: swap;
  }
  @font-face {
    font-family: 'Arabic';
    src: url(data:font/ttf;base64,${ARABIC_TTF}) format('truetype');
    font-display: swap;
  }
`;
