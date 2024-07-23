import { bearerAuthHeader } from '$lib/utils/api';
import { getJobById } from '$lib/api/jobs';
import { getJobPipelines, registerPipeline } from '$lib/api/pipelines';

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
	getJobPipelines: async ({ request, cookies }) => {
		const body = await request.json();
		const jwtToken = bearerAuthHeader(cookies.get('rustyToken') ?? '');
		return JSON.stringify(await getJobPipelines(jwtToken, body.pageNumber, body.id));
	},
	registerPipeline: async ({ request, cookies }) => {
		const body = await request.json();
		const jwtToken = bearerAuthHeader(cookies.get('rustyToken') ?? '');
		return JSON.stringify(await registerPipeline(jwtToken, body.jobId, body.branch));
	}
};
