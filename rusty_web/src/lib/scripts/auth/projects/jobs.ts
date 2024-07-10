import type { Writable } from 'svelte/store';
import { toastError } from '$lib/ui/toasts';
import { parseResponse } from '$lib/scripts/utils/parse';

export const getProjectJobs = async (id: string, name: string, pageNumber: number) => {
	return await fetch('?/getProjectJobs', {
		method: 'POST',
		body: JSON.stringify({ id, pageNumber, name })
	});
};

export const jobsFilterKeyPressed = async (
	loading: Writable<boolean>,
	projectId: string,
	filter: string,
	data: any
) => {
	loading.update((_) => true);
	const response = await getProjectJobs(projectId, filter, 1);

	if (!response.ok) {
		toastError('Error while fetching project jobs');
	} else {
		data.jobs = await parseResponse(response);
		loading.update((_) => false);
	}
	return data;
};

export const jobsListScrolled = async (
	scrollable: HTMLElement,
	loading: Writable<boolean>,
	projectId: string,
	filter: string,
	data: any
) => {
	if (scrollable.scrollTop + scrollable.clientHeight >= scrollable.scrollHeight) {
		if (data.jobs!.page * data.jobs!.pageSize >= data.jobs!.total) {
			return data;
		}

		loading.update((_) => true);
		const response = await getProjectJobs(projectId, filter, data.jobs!.page + 1);

		if (!response.ok) {
			toastError('Error while fetching project jobs');
		} else {
			const parsed = await parseResponse(response);
			parsed.entries = [...data.jobs!.entries!, ...parsed.entries];
			data.jobs! = parsed;
			loading.update((_) => false);
		}
	}
	return data;
};
