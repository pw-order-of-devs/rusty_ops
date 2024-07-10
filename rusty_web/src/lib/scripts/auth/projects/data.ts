import type { Group } from '$lib/domain/group';
import type { Job } from '$lib/domain/job';
import type { Project } from '$lib/domain/project';

interface Groups {
	active: Group;
	entries: Group[];
}

interface Projects {
	entries: Project[];
}

export interface ProjectsData {
	groups: Groups | undefined;
	projects: Projects | undefined;
}

export interface ProjectData {
	jobs: {
		entries: Job[];
	};
	project: Project | undefined;
}
