import { bearerAuthHeader } from '$lib/utils/api';
import { getJobById } from '$lib/api/jobs';
import { getJobPipelines } from '$lib/api/pipelines';

export function load({ params }) {
	return { id: params.id };
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
	}
};
