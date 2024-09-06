import type { Writable } from 'svelte/store';
import type { Group } from '$lib/domain/group';
import { toastError } from '$lib/ui/toasts';
import { parseResponse } from '$lib/scripts/utils/parse';

export const groupClicked = async (
	entry: Group,
	loading: Writable<boolean>,
	source: string,
	data: any
) => {
	if (entry.id === data.groups?.active?.id) {
		return;
	}
	loading.update(() => true);
	data.groups!.active = entry;
	const response = await fetchProjects(entry.id, source, '', 1);

	if (!response.ok) {
		toastError('Error while fetching projects');
	} else {
		const parsed = await parseResponse(response);
		data.projects!.entries = parsed.entries;
	}
	loading.update(() => false);
	return data;
};

export const projectsFilterChanged = async (
	loading: Writable<boolean>,
	groupId: string,
	filter: string,
	source: string,
	data: any
) => {
	loading.update(() => true);
	const response = await fetchProjects(groupId, source, filter, 1);

	if (!response.ok) {
		toastError('Error while fetching projects');
	} else {
		data.projects = await parseResponse(response);
		loading.update(() => false);
	}
	return data;
};

export const projectsListScrolled = async (
	scrollable: HTMLElement,
	loading: Writable<boolean>,
	groupId: string,
	filter: string,
	source: string,
	data: any
) => {
	if (scrollable.scrollTop + scrollable.clientHeight >= scrollable.scrollHeight) {
		if (data.projects!.page * data.projects!.pageSize >= data.projects!.total) {
			return data;
		}

		loading.update(() => true);
		const response = await fetchProjects(groupId, source, filter, data.projects!.page + 1);

		if (!response.ok) {
			toastError('Error while fetching projects');
		} else {
			const parsed = await parseResponse(response);
			parsed.entries = [...data.projects!.entries!, ...parsed.entries];
			data.projects! = parsed;
			loading.update(() => false);
		}
	}
	return data;
};

export const fetchProjects = async (id: string, source: string, name: string, pageNumber: number) => {
	return await fetch('?/fetchProjects', {
		method: 'POST',
		body: JSON.stringify({ groupId: `"${id}"`, pageNumber, source, name })
	});
};

export const getProjectById = async (id: string) => {
	return await fetch('?/getProjectById', {
		method: 'POST',
		body: JSON.stringify({ id })
	});
};
