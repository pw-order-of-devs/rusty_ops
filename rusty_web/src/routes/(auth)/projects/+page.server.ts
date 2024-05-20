import { bearerAuthHeader, fetchPost } from '$lib/utils/api';
import type { Project } from '$lib/domain/project';

const getProjectsQuery = (page: number) => `query {
	projects {
		get(options: { pageNumber: ${page}, pageSize: 20 }){
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

export const entries = async (auth: string, page: number) => {
	try {
		const response = await fetchPost(auth, JSON.stringify({ query: getProjectsQuery(page) }));

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

export const load = async ({ cookies }) => {
	const jwtToken = bearerAuthHeader(cookies.get('rustyToken') ?? '');
	return await entries(jwtToken, 1);
};
