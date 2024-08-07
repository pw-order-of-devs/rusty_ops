import { bearerAuthHeader } from '$lib/utils/api';
import { getJobById } from '$lib/api/jobs';
import { getPipelineById, getPipelineLogs } from '$lib/api/pipelines';

export function load({ params, cookies }) {
	return {
		id: params.id,
		jwtToken: bearerAuthHeader(cookies.get('rustyToken') ?? '')
	};
}

export const actions = {
	getJobById: async ({ request, cookies }) => {
		const body = await request.json();
		const jwtToken = bearerAuthHeader(cookies.get('rustyToken') ?? '');
		return JSON.stringify(await getJobById(jwtToken, body.id));
	},
	getPipelineById: async ({ request, cookies }) => {
		const body = await request.json();
		const jwtToken = bearerAuthHeader(cookies.get('rustyToken') ?? '');
		return JSON.stringify(await getPipelineById(jwtToken, body.id));
	},
	getPipelineLogs: async ({ request, cookies }) => {
		const body = await request.json();
		const jwtToken = bearerAuthHeader(cookies.get('rustyToken') ?? '');
		return JSON.stringify(await getPipelineLogs(jwtToken, body.id));
	}
};
