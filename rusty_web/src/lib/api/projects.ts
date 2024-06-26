import { fetchPost } from '$lib/utils/api';
import type { Project } from '$lib/domain/project';

const getProjectsQuery = (page: number, filter: string) => `query {
	projects {
		get(filter: ${filter}, options: { pageNumber: ${page}, pageSize: 30, sortMode: ASCENDING, sortField: "name" }){
			total
			page
			pageSize
			entries {
				id
				name
				url
			}
		}
	}
}`;

export const fetchProjects = async (auth: string, page: number, group: string) => {
	try {
		let groupMatch = group.match(/[a-z-]/gi);
		const filter = `{ group_id: ${groupMatch === null ? null : `${group}`} }`;
		const response = await fetchPost(
			auth,
			JSON.stringify({ query: getProjectsQuery(page, filter) })
		);

		if (!response.ok) {
			return {
				errors: ['Fetch projects failed']
			};
		} else {
			const { data, errors } = await response.json();
			if (errors && errors.length > 0) {
				return {
					errors: errors.map((error: { message: string }) => error.message)
				};
			} else if (data) {
				const paged = data?.projects?.get;
				const projects: Project[] = paged?.entries ?? [];
				return {
					total: paged?.total ?? 0,
					page: paged?.page ?? 1,
					pageSize: paged?.pageSize ?? 20,
					entries: projects
				};
			}
		}
	} catch (error) {
		return {
			errors: ['Fetch projects failed']
		};
	}
};
