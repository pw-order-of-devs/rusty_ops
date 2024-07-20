import type { Writable } from 'svelte/store';
import { toastError } from '$lib/ui/toasts';
import { parseResponse } from '$lib/scripts/utils/parse';

export const getJobPipelines = async (id: string, pageNumber: number) => {
	return await fetch('?/getJobPipelines', {
		method: 'POST',
		body: JSON.stringify({ id, pageNumber })
	});
};

export const pipelinesListScrolled = async (
	scrollable: HTMLElement,
	loading: Writable<boolean>,
	jobId: string,
	data: any
) => {
	if (scrollable.scrollTop + scrollable.clientHeight >= scrollable.scrollHeight) {
		if (data.pipelines!.page * data.pipelines!.pageSize >= data.pipelines!.total) {
			return data;
		}

		loading.update(() => true);
		const response = await getJobPipelines(jobId, data.pipelines!.page + 1);

		if (!response.ok) {
			toastError('Error while fetching job pipelines');
		} else {
			const parsed = await parseResponse(response);
			parsed.entries = [...data.pipelines!.entries!, ...parsed.entries];
			data.pipelines! = parsed;
			loading.update(() => false);
		}
	}
	return data;
};
