import { spawn } from 'child_process';
import { ChildProcess } from 'node:child_process';
import path from 'path';
import { Builder, Capabilities, WebDriver } from 'selenium-webdriver';
import { afterAll, beforeAll } from 'vitest';
import waitOn from 'wait-on';

// create the path to the expected application binary
const application = path.resolve(__dirname, '..', 'src-tauri', 'target', 'release', 'app');

// Vite dev server
let devServer: ChildProcess;

// `tauri-driver` process
let tauriDriver: ChildProcess;

// Webdriver instance
let driver: WebDriver;

beforeAll(async function () {
	// NOTE: Do not use `pnpm run ...`, it spawns detached process thus unable to kill it
	devServer = spawn('vite', ['dev', '--strictPort']);
	await waitOn({
		resources: ['http://localhost:5173'],
		timeout: 30000
	});

	// Start `tauri-driver`
	tauriDriver = spawn('tauri-driver', {
		stdio: [null, 1, 2]
	});

	// Start webdriver
	const capabilities = new Capabilities();
	capabilities.set('tauri:options', { application });
	capabilities.setBrowserName('wry');
	driver = await new Builder()
		.withCapabilities(capabilities)
		.usingServer('http://localhost:4444/')
		.build();

	globalThis.driver = driver;
}, 120000);

afterAll(async function () {
	delete globalThis.driver;

	await driver?.quit();
	tauriDriver?.kill();
	devServer?.kill();
});
