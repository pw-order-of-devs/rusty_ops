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
		let project = await getProjectById(jwtToken, body.id);
		return JSON.stringify(project);
	},
	getProjectJobs: async ({ request, cookies }) => {
		const body = await request.json();
		const jwtToken = bearerAuthHeader(cookies.get('rustyToken') ?? '');
		let jobs = await getProjectJobs(jwtToken, body.pageNumber, body.id, body.name);
		return JSON.stringify(jobs);
	}
};
