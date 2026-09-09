import { chromium } from 'playwright';

const input = await new Promise((resolve, reject) => {
  let raw = '';
  process.stdin.setEncoding('utf8');
  process.stdin.on('data', (chunk) => { raw += chunk; });
  process.stdin.on('end', () => { try { resolve(JSON.parse(raw)); } catch (error) { reject(error); } });
});
const result = { state: 'FAILED', finalUrl: null, title: null, text: null, screenshot: null, download: null, verification: {}, error: null };
let context;
try {
  context = await chromium.launchPersistentContext(input.profileDir, { headless: input.headless, acceptDownloads: false, viewport: { width: 1365, height: 768 } });
  const page = context.pages()[0] ?? await context.newPage();
  page.setDefaultTimeout(Math.min(Math.max(input.timeoutMs, 1000), 300000));
  for (const action of input.actions) {
    if (action.type === 'navigate') {
      const target = new URL(action.url);
      if (!['http:', 'https:'].includes(target.protocol)) throw new Error('Only HTTP and HTTPS are allowed');
      await page.goto(target.href, { waitUntil: 'domcontentloaded' });
    } else if (action.type === 'read') {
      const locator = action.selector ? page.locator(action.selector).first() : page.locator('body');
      result.text = (await locator.innerText()).slice(0, 1000000);
    } else if (action.type === 'click') {
      const locator = page.locator(action.selector).first();
      await locator.waitFor({ state: 'visible' });
      if (action.expected_text) {
        const actual = (await locator.innerText()).trim();
        if (!actual.includes(action.expected_text)) throw new Error('Selector text did not match the reviewed target');
      }
      await locator.click();
    } else if (action.type === 'fill') {
      const locator = page.locator(action.selector).first();
      await locator.waitFor({ state: 'visible' });
      await locator.fill(action.value);
    } else if (action.type === 'press') {
      await page.keyboard.press(action.key);
    } else if (action.type === 'wait_for') {
      await page.locator(action.selector).first().waitFor({ state: 'visible' });
    } else if (action.type === 'screenshot') {
      await page.screenshot({ path: action.path, fullPage: false });
      result.screenshot = action.path;
    } else if (action.type === 'download') {
      const [download] = await Promise.all([page.waitForEvent('download'), page.locator(action.selector).first().click()]);
      const destination = `${action.directory.replace(/[\\/]$/, '')}/${download.suggestedFilename()}`;
      await download.saveAs(destination);
      result.download = destination;
    } else {
      throw new Error(`Unsupported browser action: ${action.type}`);
    }
  }
  result.finalUrl = page.url();
  result.title = await page.title();
  result.verification = { finalUrl: result.finalUrl, title: result.title, actionCount: input.actions.length, verifiedAt: new Date().toISOString() };
  result.state = 'COMPLETED';
} catch (error) {
  result.error = error instanceof Error ? error.message : String(error);
} finally {
  if (context) await context.close();
}
process.stdout.write(JSON.stringify(result));
