export interface Job {
	id: string;
	name: string;
	description: string | undefined;
	template: string | undefined;
	project_id: string;
}
