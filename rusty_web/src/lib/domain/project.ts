import type { Job } from '$lib/domain/job';
import type { LastPipeline } from '$lib/domain/pipeline';

export type Source = 'Internal' | 'GitHub';

export interface Project {
	id: string;
	source: Source;
	name: string;
	url: string;
	jobs: Job[];
	lastPipeline: LastPipeline | undefined;
}
