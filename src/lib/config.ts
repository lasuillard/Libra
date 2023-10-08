import ipc from './ipc';

export let env: { [_: string]: string } = {};

/** Initialize config. */
export async function init() {
	const _env = await ipc.get_envs();
	env = { ..._env };

	// Inject all `PUBLIC_OTEL_*` environments variables with `PUBLIC_` prefix removed
	// NOTE: This is for telemetry providers to detect configurations; we should access via `env` if needed
	// https://github.com/open-telemetry/opentelemetry-js/blob/main/experimental/packages/exporter-trace-otlp-http/src/platform/browser/OTLPTraceExporter.ts
	// https://github.com/open-telemetry/opentelemetry-js/blob/main/packages/opentelemetry-core/src/platform/browser/environment.ts
	// https://github.com/open-telemetry/opentelemetry-js/blob/main/packages/opentelemetry-core/src/platform/browser/globalThis.ts
	for (const [key, value] of Object.entries(env)) {
		if (!key.startsWith('PUBLIC_OTEL_')) {
			continue;
		}
		Object.defineProperty(globalThis, key.replace(/^PUBLIC_/, ''), { value });
	}
}

export const settings = {
	APPLICATION_ID: 'com.libra.web'
};
