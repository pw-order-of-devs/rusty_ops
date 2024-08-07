import type { PipelineSubscription } from '$lib/domain/pipeline';
import { generateUUID } from '$lib/utils/uuid';

export class WebsocketClient {
	private ws: WebSocket | undefined;
	private readonly url: string;
	private readonly protocol: string;
	private readonly jwtToken: string;
	private readonly jobId: string;
	private readonly query: string;
	private readonly action: (msg: PipelineSubscription) => void;

	constructor(
		jwtToken: string,
		jobId: string,
		query: string,
		action: (msg: PipelineSubscription) => void
	) {
		this.url = import.meta.env.VITE_WS_URL ?? 'ws://localhost:8000/ws';
		this.protocol = 'graphql-ws';
		this.jwtToken = jwtToken;
		this.jobId = jobId;
		this.query = query;
		this.action = action;
	}

	connect() {
		this.ws = new WebSocket(this.url, this.protocol);
		this.initEventHandlers();
		return this;
	}

	private initEventHandlers() {
		if (!this.ws) return;

		this.ws.onopen = () => {
			console.debug('WebSocket is connected');
			this.ws?.send(
				JSON.stringify({
					type: 'connection_init',
					payload: { auth: `${this.jwtToken}`, extra: { jobId: `${this.jobId}` } }
				})
			);
		};

		this.ws.onmessage = (event: MessageEvent) => {
			const receivedData = JSON.parse(event.data);
			if (receivedData.type === 'connection_ack') {
				this.ws?.send(
					JSON.stringify({
						id: generateUUID(),
						type: 'start',
						payload: {
							query: `subscription {
							${this.query}
						}`
						}
					})
				);
			} else if (receivedData.type === 'data') {
				this.action(receivedData);
			}
		};

		this.ws.onerror = (error: Event) => {
			console.debug('WebSocket error: ', error);
		};

		this.ws.onclose = (ev: CloseEvent) => {
			console.debug('WebSocket is closed. Reconnect will be attempted in 1 second.', ev.reason);
			setTimeout(() => {
				this.connect();
			}, 1000);
		};
	};
}
