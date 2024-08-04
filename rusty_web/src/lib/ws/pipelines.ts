import type { PipelineSubscription } from '$lib/domain/pipeline';
import { generateUUID } from '$lib/utils/uuid';

export const subscribe = (
	jwtToken: string,
	jobId: string,
	query: string,
	action: (msg: PipelineSubscription) => void
) => {
	const ws = new WebSocket(import.meta.env.VITE_WS_URL ?? 'ws://localhost:8000/ws', 'graphql-ws');

	ws.onopen = () => {
		console.debug('WebSocket is connected');
		ws.send(
			JSON.stringify({
				type: 'connection_init',
				payload: { auth: `${jwtToken}`, extra: { jobId: `${jobId}` } }
			})
		);
	};

	ws.onmessage = (event: MessageEvent) => {
		const receivedData = JSON.parse(event.data);
		if (receivedData.type === 'connection_ack') {
			ws.send(
				JSON.stringify({
					id: generateUUID(),
					type: 'start',
					payload: {
						query: `subscription {
							${query}
						}`
					}
				})
			);
		} else if (receivedData.type === 'data') {
			action(receivedData);
		}
	};

	ws.onclose = () => {
		console.debug('WebSocket is closed');
	};

	ws.onerror = (error: Event) => {
		console.debug('WebSocket error: ', error);
	};
};
