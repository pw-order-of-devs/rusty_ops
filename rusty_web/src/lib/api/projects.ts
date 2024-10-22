import { fetchPost } from '$lib/utils/api';
import { type Project, projectSourceMap } from '$lib/domain/project';

const getProjectsQuery = (page: number, group: string, source: string, name: string) => {
	const groupMatch = group.match(/[a-z-]/gi);
	const groupIdFilter = `group_id: ${groupMatch === null ? null : `{ equals: ${group} }`}`;
	const nameFilter = `name: { contains: "${name === undefined ? '' : name}" }`;
	const filter = `filter: { ${groupIdFilter}, ${nameFilter}, source: { equals: "${projectSourceMap[source]}" } }, `;
	const options = `options: { pageNumber: ${page}, pageSize: 30, sortMode: ASCENDING, sortField: "name" }`;

	return `query {
		projects {
			get(${filter}${options}){
				total
				page
				pageSize
				entries {
					id
					source
					name
					url
					jobs {
						name
						pipelines {
							id
							number
							status
							stageStatus
							registerDate
						}
					}
				}
			}
		}
	}`;
};

export const fetchProjects = async (
	auth: string,
	page: number,
	group: string,
	source: string,
	name: string
) => {
	try {
		const response = await fetchPost(
			auth,
			JSON.stringify({ query: getProjectsQuery(page, group, source, name) })
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
				projects.forEach((p) => {
					const flattened = p.jobs.reduce((accumulator: any[], current: any) => {
						current.pipelines.forEach((pp: any) => (pp.jobName = current.name));
						return accumulator.concat(current.pipelines);
					}, []);
					flattened.sort((a, b) => {
						if (a.registerDate < b.registerDate) {
							return 1;
						}
						if (a.registerDate > b.registerDate) {
							return -1;
						}
						return 0;
					});
					if (flattened.length > 0) {
						p.lastPipeline = flattened[0];
					}
				});

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
				source
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
