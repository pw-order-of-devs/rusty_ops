import { bearerAuthHeader } from '$lib/utils/api';
import { getProjectJobs } from '$lib/api/jobs';
import { getProjectById } from '$lib/api/projects';

export function load({ params }) {
	return { id: params.id };
}

export const actions = {
	getProjectById: async ({ request, cookies }) => {
		const body = await request.json();
		const jwtToken = bearerAuthHeader(cookies.get('rustyToken') ?? '');
		return JSON.stringify(await getProjectById(jwtToken, body.id));
	},
	getProjectJobs: async ({ request, cookies }) => {
		const body = await request.json();
		const jwtToken = bearerAuthHeader(cookies.get('rustyToken') ?? '');
		return JSON.stringify(await getProjectJobs(jwtToken, body.pageNumber, body.id, body.name));
	}
};
