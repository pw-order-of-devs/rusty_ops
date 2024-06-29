import type { Writable } from 'svelte/store';
import { toastError } from '$lib/ui/toasts';

export const groupsFilterKeyPressed = async (
	loadingGroups: Writable<boolean>,
	groupsFilter: string,
	data: any
) => {
	loadingGroups.update((_) => true);
	const response = await fetchGroups(groupsFilter, 1);

	if (!response.ok) {
		toastError('Error while fetching groups');
	} else {
		data.groups = await parseGroups(response);
		loadingGroups.update((_) => false);
	}
	return data;
};

export const groupsListScrolled = async (
	scrollableGroups: HTMLElement,
	loadingGroups: Writable<boolean>,
	groupsFilter: string,
	data: any
) => {
	if (scrollableGroups.scrollTop + scrollableGroups.clientHeight >= scrollableGroups.scrollHeight) {
		if (data.groups!.page * data.groups!.pageSize >= data.groups!.total) {
			return data;
		}

		loadingGroups.update((_) => true);
		const response = await fetchGroups(groupsFilter, data.groups!.page + 1);

		if (!response.ok) {
			toastError('Error while fetching groups');
		} else {
			const parsed = await parseGroups(response);
			parsed.entries = [...data.groups!.entries!, ...parsed.entries];
			data.groups! = parsed;
			loadingGroups.update((_) => false);
		}
	}
	return data;
};

export const fetchGroups = async (groupsFilter: string, pageNumber: number) => {
	return await fetch('?/fetchGroups', {
		method: 'POST',
		body: JSON.stringify({ groupName: groupsFilter, pageNumber })
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
