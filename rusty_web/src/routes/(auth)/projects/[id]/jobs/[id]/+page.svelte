<script lang="ts">
	import Card from 'src/components/auth/Card.svelte';
	import Loader from 'src/components/shared/Loader.svelte';
	import type { Pipeline, PipelineSubscription } from '$lib/domain/pipeline';
	import type { JobData } from '$lib/scripts/auth/projects/data';
	import { getJobById } from '$lib/scripts/auth/projects/jobs';
	import {
		getJobPipelines,
		pipelinesListScrolled,
		registerPipeline
	} from '$lib/scripts/auth/projects/pipelines';
	import { parseResponse } from '$lib/scripts/utils/parse';
	import { WebsocketClient } from '$lib/ws/pipelines';
	import { writable } from 'svelte/store';
	import { onMount } from 'svelte';
	import HighlightSvelte, { LineNumbers } from 'svelte-highlight';
	import yaml from 'svelte-highlight/languages/yaml';
	import 'svelte-highlight/styles/atom-one-dark.css';
	import PipelineCard from 'src/components/auth/projects/PipelineCard.svelte';

	let loading = writable(false);
	let loadingPipelines = writable(false);

	let scrollablePipelines: HTMLElement;

	let pageData: JobData | undefined = undefined;

	export let data;
	let currentPath = '';

	onMount(async () => {
		new WebsocketClient(
			data.jwtToken,
			data['id'],
			"pipelineInserted { id number status branch registerDate startDate endDate jobId }",
			(message: PipelineSubscription) => {
				if (pageData !== undefined && message.payload.data.pipelineInserted !== undefined) {
					pageData!.pipelines.entries.unshift(message.payload.data.pipelineInserted);
					pageData = pageData;
				}
			}
		).connect();
		new WebsocketClient(
			data.jwtToken,
			data['id'],
			"pipelineUpdated { id number status branch registerDate startDate endDate jobId }",
			(message: PipelineSubscription) => {
				if (pageData !== undefined && message.payload.data.pipelineUpdated !== undefined) {
					let pipeline = message.payload.data.pipelineUpdated!;
					let index = pageData!.pipelines.entries.findIndex((item) => item.id == pipeline.id);
					pageData!.pipelines.entries[index] = pipeline;
				}
			}
		).connect();

		currentPath = new URL(window.location.href).pathname;
		loading.update(() => true);
		let job = await parseResponse(await getJobById(data['id']));
		let pipelines = await parseResponse(await getJobPipelines(data['id'], 1));
		pageData = { job, template: atob(job.template), pipelines };

		let element = document.getElementsByClassName('job-template')[0].children[0];
		element.setAttribute('style', 'overflow: auto; height: calc(100vh - 13rem)');
		loading.update(() => false);
	});

	const pipelinesListScrolled_ = async () => {
		pageData = await pipelinesListScrolled(
			scrollablePipelines,
			loadingPipelines,
			data['id'],
			pageData
		);
	};

	const rerunPipeline = async (entry: Pipeline) => {
		await registerPipeline(entry.jobId, entry.branch);
	};
</script>

{#if $loading}
	<Loader />
{/if}

<div class="job-page">
	<div class="job-title">
		<Card classes="job-card">
			<div class="job-name wrap-text">
				{pageData?.job?.name}
			</div>
			<div class="job-description wrap-text">
				{pageData?.job?.description ?? "No description"}
			</div>
		</Card>
	</div>
	<div class="job-data">
		<div class="job-template">
			<HighlightSvelte language={yaml} code={pageData?.template ?? ''} let:highlighted>
				<LineNumbers {highlighted} />
			</HighlightSvelte>
		</div>
		<div class="job-pipelines">
			{#if $loadingPipelines}
				<Loader />
			{/if}

			<div class="entries" bind:this={scrollablePipelines} on:scroll={pipelinesListScrolled_}>
				{#each pageData?.pipelines?.entries ?? [] as entry (entry.id)}
					<PipelineCard {entry} {currentPath} {rerunPipeline} />
				{/each}
			</div>
		</div>
	</div>
</div>

<style lang="scss">
	@import 'src/styles/auth/projects/id/jobs/id/style';
</style>
