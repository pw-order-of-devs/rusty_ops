import type { Job } from '$lib/domain/job';
import type { LastPipeline } from '$lib/domain/pipeline';

export type Source = 'Internal' | 'GitHub';

export const projectSourceMap: Record<string, string> = {
	INTERNAL: 'Internal',
	GIT_HUB: 'GitHub',
	GITLAB: 'Gitlab',
	BITBUCKET: 'BitBucket'
};

export interface Project {
	id: string;
	source: Source;
	name: string;
	url: string;
	jobs: Job[];
	lastPipeline: LastPipeline | undefined;
}
