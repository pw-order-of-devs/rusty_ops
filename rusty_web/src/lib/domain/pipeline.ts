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
	stageStatus: Map<string, string> | undefined;
	status: string;
	branch: string;
	registerDate: string;
	startDate: string | undefined;
	endDate: string | undefined;
	jobId: string;
}

export interface PipelineSubscription {
	payload: {
		data: {
			pipelineInserted: Pipeline | undefined;
			pipelineUpdated: Pipeline | undefined;
			pipelineLogs: string | undefined;
		};
	};
}
