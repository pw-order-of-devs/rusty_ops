import { fetchPost } from '$lib/utils/api';
import type { Project } from '$lib/domain/project';

const getProjectsQuery = (page: number, group: string, name: string) => {
	let groupMatch = group.match(/[a-z-]/gi);
	let groupIdFilter = `group_id: ${groupMatch === null ? null : `${group}`}`;
	let filter = `filter: { ${groupIdFilter}, name: { contains: "${name}" } }, `;
	let options = `options: { pageNumber: ${page}, pageSize: 30, sortMode: ASCENDING, sortField: "name" }`;

	return `query {
		projects {
			get(${filter}${options}){
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
};

export const fetchProjects = async (auth: string, page: number, group: string, name: string) => {
	try {
		const response = await fetchPost(
			auth,
			JSON.stringify({ query: getProjectsQuery(page, group, name) })
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

const getProjectByIdQuery = (id: string) => {
	return `query {
		projects {
			getById(id: "${id}"){
				id
				name
				url
			}
		}
	}`;
};

export const getProjectById = async (auth: string, id: string) => {
	try {
		const response = await fetchPost(auth, JSON.stringify({ query: getProjectByIdQuery(id) }));

		if (!response.ok) {
			return {
				errors: ['Get project by id failed']
			};
		} else {
			const { data, errors } = await response.json();
			if (errors && errors.length > 0) {
				return {
					errors: errors.map((error: { message: string }) => error.message)
				};
			} else if (data) {
				return data?.projects?.getById;
			}
		}
	} catch (error) {
		return {
			errors: ['Get project by id failed']
		};
	}
};
