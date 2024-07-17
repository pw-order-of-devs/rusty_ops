import type { Job } from '$lib/domain/job';
import type { LastPipeline } from '$lib/domain/pipeline';

export interface Project {
	id: string;
	name: string;
	url: string;
	jobs: Job[];
	lastPipeline: LastPipeline | undefined;
}
