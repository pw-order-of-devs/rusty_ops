import type { Writable } from 'svelte/store';
import { toastError } from '$lib/ui/toasts';

export const groupsFilterKeyPressed = async (
	loading: Writable<boolean>,
	filter: string,
	data: any
) => {
	loading.update((_) => true);
	const response = await fetchGroups(filter, 1);

	if (!response.ok) {
		toastError('Error while fetching groups');
	} else {
		data.groups = await parseGroups(response);
		loading.update((_) => false);
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

		loading.update((_) => true);
		const response = await fetchGroups(filter, data.groups!.page + 1);

		if (!response.ok) {
			toastError('Error while fetching groups');
		} else {
			const parsed = await parseGroups(response);
			parsed.entries = [...data.groups!.entries!, ...parsed.entries];
			data.groups! = parsed;
			loading.update((_) => false);
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

export const parseGroups = async (response: Response) => {
	const resp = (await response.json()).data;
	let parsed = JSON.parse(resp.substring(1, resp.length - 1));
	if (typeof parsed === 'string') {
		parsed = JSON.parse(parsed);
	}

	return parsed;
};
