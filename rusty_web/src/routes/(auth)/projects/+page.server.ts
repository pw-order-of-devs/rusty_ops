import { bearerAuthHeader } from '$lib/utils/api';
import { fetchProjects } from '$lib/api/projects';
import { fetchGroups } from '$lib/api/groups';

export const actions = {
	fetchGroups: async ({ request, cookies }) => {
		const body = await request.json();
		const jwtToken = bearerAuthHeader(cookies.get('rustyToken') ?? '');
		return JSON.stringify(await fetchGroups(jwtToken, body.pageNumber, body.groupName));
	},
	fetchProjects: async ({ request, cookies }) => {
		const body = await request.json();
		const jwtToken = bearerAuthHeader(cookies.get('rustyToken') ?? '');
		return JSON.stringify(
			await fetchProjects(jwtToken, body.pageNumber, body.groupId, body.source, body.name)
		);
	}
};
