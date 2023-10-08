import { By, WebDriver } from 'selenium-webdriver';
import { describe, expect, it } from 'vitest';

describe('test', () => {
	it('should show something', async () => {
		const driver: WebDriver = globalThis.driver;
		await driver.manage().setTimeouts({ implicit: 3000 });
		const text = await driver.findElement(By.id('greeting-message')).getText();
		expect(text).to.match(/^[wW]elcome to SvelteKit/);
	});
});
