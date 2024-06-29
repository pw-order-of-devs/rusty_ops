import type { Writable } from 'svelte/store';
import type { Group } from '$lib/domain/group';
import { toastError } from '$lib/ui/toasts';
import { parseGroups } from '$lib/scripts/auth/projects/groups';

export const groupClicked = async (entry: Group, loading: Writable<boolean>, data: any) => {
	if (entry.id === data.groups?.active?.id) {
		return;
	}
	loading.update((_) => true);
	data.groups!.active = entry;
	const response = await fetchProjects(entry.id, 1);

	if (!response.ok) {
		toastError('Error while fetching projects');
	} else {
		const parsed = await parseProjects(response);
		data.projects!.entries = parsed.entries;
	}
	loading.update((_) => false);
	return data;
};

export const fetchProjects = async (id: string, pageNumber: number) => {
	return await fetch('?/fetchProjects', {
		method: 'POST',
		body: JSON.stringify({ groupId: `"${id}"`, pageNumber: 1 })
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
