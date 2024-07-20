import type { Writable } from 'svelte/store';
import { toastError } from '$lib/ui/toasts';
import { parseResponse } from '$lib/scripts/utils/parse';

export const groupsFilterKeyPressed = async (
	loading: Writable<boolean>,
	filter: string,
	data: any
) => {
	loading.update(() => true);
	const response = await fetchGroups(filter, 1);

	if (!response.ok) {
		toastError('Error while fetching groups');
	} else {
		data.groups = await parseResponse(response);
		loading.update(() => false);
	}
	return data;
};

export const groupsListScrolled = async (
	scrollable: HTMLElement,
	loading: Writable<boolean>,
	filter: string,
	data: any
) => {
	if (scrollable.scrollTop + scrollable.clientHeight >= scrollable.scrollHeight) {
		if (data.groups!.page * data.groups!.pageSize >= data.groups!.total) {
			return data;
		}

		loading.update(() => true);
		const response = await fetchGroups(filter, data.groups!.page + 1);

		if (!response.ok) {
			toastError('Error while fetching groups');
		} else {
			const parsed = await parseResponse(response);
			parsed.entries = [...data.groups!.entries!, ...parsed.entries];
			data.groups! = parsed;
			loading.update(() => false);
		}
	}
	return data;
};

export const fetchGroups = async (filter: string, pageNumber: number) => {
	return await fetch('?/fetchGroups', {
		method: 'POST',
		body: JSON.stringify({ groupName: filter, pageNumber })
	});
};
