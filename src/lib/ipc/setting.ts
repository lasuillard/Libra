import { settings } from '$lib/config';
import { trace } from '@opentelemetry/api';
import { invoke } from '@tauri-apps/api';
import { IPCMessage } from './message';

/**
 * Get environment variables matching given regular expression (Rust syntax).
 * @param expr Regular expression.
 * @returns Map of environment variables.
 */
export async function get_envs(expr?: string): Promise<{ [_: string]: string }> {
	expr ??= '';

	const tracer = trace.getTracer(settings.APPLICATION_ID);
	const span = tracer.startSpan('ipc/get_envs');
	span.setAttribute('ipc.request.expression', expr);

	// Send IPC request
	const response: IPCMessage<{ [_: string]: string }> = await invoke('get_envs', {
		message: new IPCMessage(expr)
	});
	const vars = response.body;

	// Save keys to span attribute
	const keys = Array.from(Object.keys(vars)).join(', ');
	span.setAttribute('ipc.response.keys', keys);
	span.end();

	return vars;
}
