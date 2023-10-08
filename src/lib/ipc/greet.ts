import { settings } from '$lib/config';
import { trace } from '@opentelemetry/api';
import { invoke } from '@tauri-apps/api';
import { IPCMessage } from './message';

/**
 * Call greet IPC.
 * @param name Name of user to greet.
 * @returns Greeting message.
 */
export async function greet(name: string): Promise<string> {
	const tracer = trace.getTracer(settings.APPLICATION_ID);
	const span = tracer.startSpan('ipc/greet');
	span.setAttribute('ipc.request.name', name);

	const response: IPCMessage<string> = await invoke('greet', { message: new IPCMessage(name) });
	const greeting = response.body;

	span.setAttribute('ipc.response.greeting', greeting);
	span.end();

	return greeting;
}
