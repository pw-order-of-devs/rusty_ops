import type { Writable } from 'svelte/store';
import type { Group } from '$lib/domain/group';
import { toastError } from '$lib/ui/toasts';

export const groupClicked = async (entry: Group, loading: Writable<boolean>, data: any) => {
	if (entry.id === data.groups?.active?.id) {
		return;
	}
	loading.update((_) => true);
	data.groups!.active = entry;
	const response = await fetchProjects(entry.id, '', 1);

	if (!response.ok) {
		toastError('Error while fetching projects');
	} else {
		const parsed = await parseProjects(response);
		data.projects!.entries = parsed.entries;
	}
	loading.update((_) => false);
	return data;
};

export const projectsFilterKeyPressed = async (
	loading: Writable<boolean>,
	groupId: string,
	filter: string,
	data: any
) => {
	loading.update((_) => true);
	const response = await fetchProjects(groupId, filter, 1);

	if (!response.ok) {
		toastError('Error while fetching projects');
	} else {
		data.projects = await parseProjects(response);
		loading.update((_) => false);
	}
	return data;
};

export const projectsListScrolled = async (
	scrollable: HTMLElement,
	loading: Writable<boolean>,
	groupId: string,
	filter: string,
	data: any
) => {
	if (scrollable.scrollTop + scrollable.clientHeight >= scrollable.scrollHeight) {
		if (data.projects!.page * data.projects!.pageSize >= data.projects!.total) {
			return data;
		}

		loading.update((_) => true);
		const response = await fetchProjects(groupId, filter, data.projects!.page + 1);

		if (!response.ok) {
			toastError('Error while fetching projects');
		} else {
			const parsed = await parseProjects(response);
			parsed.entries = [...data.projects!.entries!, ...parsed.entries];
			data.projects! = parsed;
			loading.update((_) => false);
		}
	}
	return data;
};

export const fetchProjects = async (id: string, name: string, pageNumber: number) => {
	return await fetch('?/fetchProjects', {
		method: 'POST',
		body: JSON.stringify({ groupId: `"${id}"`, pageNumber, name })
	});
};

export const parseProjects = async (response: Response) => {
	const resp = (await response.json()).data;
	let parsed = JSON.parse(resp.substring(1, resp.length - 1));
	if (typeof parsed === 'string') {
		parsed = JSON.parse(parsed);
	}

	return parsed;
};
