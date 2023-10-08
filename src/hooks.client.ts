import * as config from '$lib/config';
import { getWebAutoInstrumentations } from '@opentelemetry/auto-instrumentations-web';
import { ZoneContextManager } from '@opentelemetry/context-zone';
import { OTLPTraceExporter } from '@opentelemetry/exporter-trace-otlp-http';
import { registerInstrumentations } from '@opentelemetry/instrumentation';
import { detectResourcesSync, envDetector, Resource } from '@opentelemetry/resources';
import { BatchSpanProcessor, WebTracerProvider } from '@opentelemetry/sdk-trace-web';

await config.init();

const otelDisabled = (config.env.PUBLIC_OTEL_SDK_DISABLED || 'false').toLowerCase() === 'true';

if (otelDisabled) {
	console.info('Skipping OpenTelemetry initialization');
} else {
	const tracer_provider = new WebTracerProvider({
		resource: Resource.default().merge(detectResourcesSync({ detectors: [envDetector] }))
	});

	tracer_provider.addSpanProcessor(
		new BatchSpanProcessor(
			new OTLPTraceExporter({
				headers: {} // Force exporter to use XHR instead of Beacon API; https://github.com/open-telemetry/opentelemetry-js/issues/1593
			})
		)
	);
	tracer_provider.register({
		// https://github.com/open-telemetry/opentelemetry-js/issues/3171
		contextManager: new ZoneContextManager()
	});

	registerInstrumentations({
		instrumentations: getWebAutoInstrumentations({
			'@opentelemetry/instrumentation-xml-http-request': {
				clearTimingResources: true
			},
			'@opentelemetry/instrumentation-fetch': {
				clearTimingResources: true,
				// Propagation headers are not shown in browser debugger Without this
				// https://github.com/open-telemetry/opentelemetry-js/discussions/2209
				propagateTraceHeaderCorsUrls: [/http:\/\/localhost:.*/]
			}
		})
	});

	console.info('OpenTelemetry initialized');
}
