<script lang="ts">
	import * as yaml from 'js-yaml';
	import { tooltip } from '$lib/ui/tooltip';
	import type { PipelineData } from '$lib/scripts/auth/projects/data';
	import { parseResponse } from '$lib/scripts/utils/parse';
	import { getJobById } from '$lib/scripts/auth/projects/jobs';
	import { getPipelineById, getPipelineLogs } from '$lib/scripts/auth/projects/pipelines';
	import { updateRunTime } from '$lib/utils/pipeline-run-time';
	import { WebsocketClient } from '$lib/ws/pipelines';
	import Loader from 'src/components/shared/Loader.svelte';
	import Card from 'src/components/auth/Card.svelte';
	import { writable } from 'svelte/store';
	import { onDestroy, onMount } from 'svelte';
	import type { PipelineSubscription } from '$lib/domain/pipeline';

	let loading = writable(false);

	let pageData: PipelineData | undefined = undefined;

	let executionTime: string = '';
	let interval: number;

	let logsStage = 'all';
	let previewLogs: string[] = [];
	let stages: string[] = [];

	export let data;

	onMount(async () => {
		loading.update(() => true);
		let pipeline = await parseResponse(await getPipelineById(data['id']));
		let templateStr = atob((await parseResponse(await getJobById(pipeline.jobId))).template);
		let logs = await parseResponse(await getPipelineLogs(data['id']));
		pageData = { pipeline, template: yaml.load(templateStr), logs };

		buildStagesList();
		updatePreviewLogs();
		subscribe();
		loading.update(() => false);

		executionTime = updateRunTime(pageData?.pipeline);
		interval = setInterval(() => {
			executionTime = updateRunTime(pageData?.pipeline);
		}, 3000);
	});

	onDestroy(() => {
		clearInterval(interval);
	});

	const updatePreviewLogs = () => {
		if (logsStage === 'all') {
			previewLogs = Object.values(pageData?.logs ?? {}).reduce((acc: string[], group: any) => {
				const value = group.map((item: any) => item['line']);
				return acc.concat(value);
			}, []);
		} else {
			previewLogs = pageData?.logs[logsStage].map((item: any) => item['line']);
		}
	};

	const buildStagesList = () => {
		let keys = Object.keys(pageData!.template);
		stages = Object.keys(pageData!.template['stages']);
		if (keys.includes('before')) stages.unshift('before');
		stages.unshift('all');
		if (keys.includes('after')) stages.unshift('after');
	};

	const updateLogsStage = (stage: string) => {
		logsStage = stage;
		updatePreviewLogs();
	};

	const subscribe = () => {
		new WebsocketClient(
			data.jwtToken,
			pageData!.pipeline.jobId,
			'pipelineUpdated { id number status branch registerDate startDate endDate jobId agentId }',
			(message: PipelineSubscription) => {
				if (pageData !== undefined && message.payload.data.pipelineUpdated !== undefined) {
					if (message.payload.data.pipelineUpdated.id === data['id']) {
						pageData!.pipeline = message.payload.data.pipelineUpdated;
					}
				}
			}
		).connect();
		new WebsocketClient(
			data.jwtToken,
			data['id'],
			'pipelineLogs',
			(message: PipelineSubscription) => {
				if (pageData !== undefined && message.payload.data.pipelineLogs !== undefined) {
					const logEntry = JSON.parse(message.payload.data.pipelineLogs);
					let stage = logEntry['stage'];
					if (stage == 'rusty-before') {
						stage = 'before';
					}
					if (stage == 'rusty-after') {
						stage = 'after';
					}
					if (!Object.keys(pageData!.logs).includes(stage)) {
						pageData!.logs[stage] = [];
					}
					pageData!.logs[stage].push(logEntry);
					updatePreviewLogs();
				}
			}
		).connect();
	};
</script>

{#if $loading}
	<Loader />
{/if}

<div class="pipeline-page">
	<div class="pipeline-title">
		<Card classes="pipeline-card">
			<div class="pipeline-card-status">
				<div>#{pageData?.pipeline.number}</div>
				<div
					class="circle circle-{pageData?.pipeline.status.toLowerCase()}"
					use:tooltip={{
						content: pageData?.pipeline.status.toLowerCase() ?? '',
						placement: 'bottom'
					}}
				/>
				<div>{executionTime} @ {pageData?.pipeline.branch}</div>
			</div>
		</Card>
	</div>
	<div class="pipeline-stages">
		<Card classes="pipeline-card">
			<div class="pipeline-card-stages">
				<div class="tabs">
					{#each stages as entry, index (entry)}
						<div
							class="wrap-text"
							class:selected={logsStage === entry}
							on:click={() => updateLogsStage(entry)}
							on:keydown={() => updateLogsStage(entry)}
							role="tab"
							tabindex={index}
						>
							{entry}
						</div>
					{/each}
				</div>

				<div class="logs">
					{#each previewLogs ?? [] as entry}
						<div>{entry}</div>
					{/each}
				</div>
			</div>
		</Card>
	</div>
</div>

<style lang="scss">
	@import 'src/styles/auth/projects/id/jobs/id/pipelines/id/style';
</style>
