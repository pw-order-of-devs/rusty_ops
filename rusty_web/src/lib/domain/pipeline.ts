export interface LastPipeline {
	id: string;
	number: number;
	status: string;
	registerDate: string;
	jobName: string;
}

export interface Pipeline {
	id: string;
	number: number;
	status: string;
	branch: string;
	registerDate: string;
	startDate: string | undefined;
	endDate: string | undefined;
	jobId: string;
}
