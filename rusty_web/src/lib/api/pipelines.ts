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
					registerDate
					startDate
					endDate
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
