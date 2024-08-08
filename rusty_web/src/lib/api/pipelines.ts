import { fetchPost } from '$lib/utils/api';
import type { Pipeline } from '$lib/domain/pipeline';

const getJobPipelinesQuery = (page: number, id: string) => {
	const filter = `filter: { job_id: { equals: "${id}" } }, `;
	const options = `options: { pageNumber: ${page}, pageSize: 20, sortMode: DESCENDING, sortField: "number" }`;

	return `query {
		pipelines {
			get(${filter}${options}){
				total
				page
				pageSize
				entries {
					id
					number
					status
					stageStatus
					branch
					registerDate
					startDate
					endDate
					jobId
				}
			}
		}
	}`;
};

export const getJobPipelines = async (auth: string, page: number, id: string) => {
	try {
		const response = await fetchPost(
			auth,
			JSON.stringify({ query: getJobPipelinesQuery(page, id) })
		);

		if (!response.ok) {
			return {
				errors: ['Fetch job pipelines failed']
			};
		} else {
			const { data, errors } = await response.json();
			if (errors && errors.length > 0) {
				return {
					errors: errors.map((error: { message: string }) => error.message)
				};
			} else if (data) {
				const paged = data?.pipelines?.get;
				const pipelines: Pipeline[] = paged?.entries ?? [];
				return {
					total: paged?.total ?? 0,
					page: paged?.page ?? 1,
					pageSize: paged?.pageSize ?? 20,
					entries: pipelines
				};
			}
		}
	} catch (error) {
		return {
			errors: ['Fetch job pipelines failed']
		};
	}
};

const registerPipelineMutation = (jobId: string, branch: string) => {
	return `mutation {
		pipelines {
			register(pipeline: {
				jobId: "${jobId}",
				branch: "${branch}"
			})
		}
	}`;
};

export const registerPipeline = async (auth: string, jobId: string, branch: string) => {
	try {
		const response = await fetchPost(
			auth,
			JSON.stringify({ query: registerPipelineMutation(jobId, branch) })
		);

		if (!response.ok) {
			return {
				errors: ['Register pipeline failed']
			};
		} else {
			const { data, errors } = await response.json();
			if (errors && errors.length > 0) {
				return {
					errors: errors.map((error: { message: string }) => error.message)
				};
			} else if (data) {
				return data;
			}
		}
	} catch (error) {
		return {
			errors: ['Register pipeline failed']
		};
	}
};

const getPipelineByIdQuery = (id: string) => {
	return `query {
		pipelines {
			getById(id: "${id}"){
                id
                number
                branch
                status
				stageStatus
                registerDate
                startDate
                endDate
                jobId
                agentId
			}
		}
	}`;
};

export const getPipelineById = async (auth: string, id: string) => {
	try {
		const response = await fetchPost(auth, JSON.stringify({ query: getPipelineByIdQuery(id) }));

		if (!response.ok) {
			return {
				errors: ['Get pipeline by id failed']
			};
		} else {
			const { data, errors } = await response.json();
			if (errors && errors.length > 0) {
				return {
					errors: errors.map((error: { message: string }) => error.message)
				};
			} else if (data) {
				return data?.pipelines?.getById;
			}
		}
	} catch (error) {
		return {
			errors: ['Get pipeline by id failed']
		};
	}
};

const getPipelineByLogsQuery = (id: string) => {
	return `query { pipelines { getLogs(id: "${id}") } }`;
};

export const getPipelineLogs = async (auth: string, id: string) => {
	try {
		const response = await fetchPost(auth, JSON.stringify({ query: getPipelineByLogsQuery(id) }));

		if (!response.ok) {
			return {
				errors: ['Get pipeline logs failed']
			};
		} else {
			const { data, errors } = await response.json();
			if (errors && errors.length > 0) {
				return {
					errors: errors.map((error: { message: string }) => error.message)
				};
			} else if (data) {
				return data?.pipelines?.getLogs
					.map((value: string) => JSON.parse(value))
					.reduce((rv: any, x: any) => {
						let key = x['stage'];
						if (key == 'rusty-before') {
							key = 'before';
						}
						if (key == 'rusty-after') {
							key = 'after';
						}
						(rv[key] = rv[key] || []).push(x);
						return rv;
					}, {});
			}
		}
	} catch (error) {
		return {
			errors: ['Get pipeline logs failed']
		};
	}
};
