import { defineConfig } from 'vitest/config';
export default defineConfig({
	test: {
		include: ['e2e/**/*.{test,spec}.{js,ts}'],
		setupFiles: ['e2e/setup.ts']
	}
});
