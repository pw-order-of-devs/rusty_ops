import type { Pipeline } from '$lib/domain/pipeline';

export interface Job {
	id: string;
	name: string;
	description: string | undefined;
	template: string | undefined;
	project_id: string;
	pipelines: Pipeline[] | undefined;
}
