import { context, propagation, type Context } from '@opentelemetry/api';

export class IPCMessage<T> {
	// NOTE: Map<string, string> does not work, seems it's because the default getter & setter implementation
	//       aims to object, not map
	// See https://github.com/open-telemetry/opentelemetry-js/blob/main/api/src/propagation/TextMapPropagator.ts#L113-L137
	headers: { [_: string]: string };
	body: T;

	constructor(body: T, headers?: { [_: string]: string }, cx?: Context | null) {
		this.body = body;
		this.headers = headers ?? {};
		if (cx !== null) {
			this.inject_cx(cx ?? context.active());
		}
	}

	extract_cx(parent_cx?: Context): Context {
		return propagation.extract(parent_cx ?? context.active(), this.headers);
	}

	inject_cx(cx: Context) {
		propagation.inject(cx, this.headers);
	}
}
