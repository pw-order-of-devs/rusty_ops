import type { Group } from '$lib/domain/group';
import type { Job } from '$lib/domain/job';
import type { Project } from '$lib/domain/project';
import type { Pipeline } from '$lib/domain/pipeline';

export interface ProjectsData {
	groups:
		| {
				active: Group;
				entries: Group[];
		  }
		| undefined;
	projects:
		| {
				entries: Project[];
		  }
		| undefined;
}

export interface ProjectData {
	jobs: {
		entries: Job[];
	};
	project: Project | undefined;
}

export interface JobData {
	job: Job;
	template: string;
	pipelines: {
		entries: Pipeline[];
	};
}
