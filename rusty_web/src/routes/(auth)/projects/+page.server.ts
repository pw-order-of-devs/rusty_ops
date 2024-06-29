import { bearerAuthHeader } from '$lib/utils/api';
import { fetchProjects } from '$lib/api/projects';
import { fetchGroups } from '$lib/api/groups';

export const actions = {
	fetchGroups: async ({ request, cookies }) => {
		const body = await request.json();
		const jwtToken = bearerAuthHeader(cookies.get('rustyToken') ?? '');
		let groups = await fetchGroups(jwtToken, body.pageNumber, body.groupName);
		return JSON.stringify(groups);
	},
	fetchProjects: async ({ request, cookies }) => {
		const body = await request.json();
		const jwtToken = bearerAuthHeader(cookies.get('rustyToken') ?? '');
		let projects = await fetchProjects(jwtToken, body.pageNumber, body.groupId);
		return JSON.stringify(projects);
	}
};
