import { fetchPost } from '$lib/utils/api';
import type { Job } from '$lib/domain/job';

const getProjectJobsQuery = (page: number, id: string, name: string) => {
	const filter = `filter: { project_id: { equals: "${id}" }, name: { contains: "${name}" } }, `;
	const options = `options: { pageNumber: ${page}, pageSize: 50, sortMode: ASCENDING, sortField: "name" }`;

	return `query {
		jobs {
			get(${filter}${options}){
				total
				page
				pageSize
				entries {
					id
					name
					description
					projectId
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
	}`;
};

export const getProjectJobs = async (auth: string, page: number, id: string, name: string) => {
	try {
		const response = await fetchPost(
			auth,
			JSON.stringify({ query: getProjectJobsQuery(page, id, name) })
		);

		if (!response.ok) {
			return {
				errors: ['Fetch project jobs failed']
			};
		} else {
			const { data, errors } = await response.json();
			if (errors && errors.length > 0) {
				return {
					errors: errors.map((error: { message: string }) => error.message)
				};
			} else if (data) {
				const paged = data?.jobs?.get;
				const jobs: Job[] = paged?.entries ?? [];
				return {
					total: paged?.total ?? 0,
					page: paged?.page ?? 1,
					pageSize: paged?.pageSize ?? 20,
					entries: jobs
				};
			}
		}
	} catch (error) {
		return {
			errors: ['Fetch project jobs failed']
		};
	}
};

const getJobByIdQuery = (id: string) => {
	return `query {
		jobs {
			getById(id: "${id}"){
				id
				name
				description
				template
			}
		}
	}`;
};

export const getJobById = async (auth: string, id: string) => {
	try {
		const response = await fetchPost(auth, JSON.stringify({ query: getJobByIdQuery(id) }));

		if (!response.ok) {
			return {
				errors: ['Get job by id failed']
			};
		} else {
			const { data, errors } = await response.json();
			if (errors && errors.length > 0) {
				return {
					errors: errors.map((error: { message: string }) => error.message)
				};
			} else if (data) {
				return data?.jobs?.getById;
			}
		}
	} catch (error) {
		return {
			errors: ['Get job by id failed']
		};
	}
};
