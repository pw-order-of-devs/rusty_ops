import type { PipelineSubscription } from '$lib/domain/pipeline';
import { generateUUID } from '$lib/utils/uuid';

export const subscribe = (
	jwtToken: string,
	jobId: string,
	onCreated: (msg: PipelineSubscription) => void,
	onUpdated: (msg: PipelineSubscription) => void
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
							pipelineInserted { id number status branch registerDate startDate endDate jobId }
							pipelineUpdated { id number status branch registerDate startDate endDate jobId }
						}`
					}
				})
			);
		} else if (receivedData.type === 'data') {
			if (receivedData.payload.data.pipelineInserted !== undefined) {
				onCreated(receivedData);
			} else if (receivedData.payload.data.pipelineUpdated !== undefined) {
				onUpdated(receivedData);
			}
		}
	};

	ws.onclose = () => {
		console.debug('WebSocket is closed');
	};

	ws.onerror = (error: Event) => {
		console.debug('WebSocket error: ', error);
	};
};
