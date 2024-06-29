import type { Group } from '$lib/domain/group';
import type { Project } from '$lib/domain/project';

interface Groups {
	active: Group;
	entries: Group[];
}

interface Projects {
	entries: Project[];
}

export interface Data {
	groups: Groups | undefined;
	projects: Projects | undefined;
}
