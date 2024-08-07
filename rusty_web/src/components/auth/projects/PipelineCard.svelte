<script lang="ts">
	import { tooltip } from '$lib/ui/tooltip';
	import { updateRunTime } from '$lib/utils/pipeline-run-time';
	import type { Pipeline } from '$lib/domain/pipeline';
	import Card from 'src/components/auth/Card.svelte';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import { faArrowRight, faArrowRightRotate } from '@fortawesome/free-solid-svg-icons';
	import { onMount, onDestroy } from 'svelte';

	export let entry: Pipeline;
	export let currentPath: string;
	export let rerunPipeline: any;

	let executionTime: string = '';
	let interval: number;

	onMount(() => {
		executionTime = updateRunTime(entry);

		interval = setInterval(() => {
			executionTime = updateRunTime(entry);
		}, 3000);
	});

	onDestroy(() => {
		clearInterval(interval);
	});
</script>

<Card classes="pipeline-card-wrapper">
	<div class="pipeline-card">
		<div class="pipeline-card-status">
			<div>#{entry.number}</div>
			<div
				class="circle circle-{entry.status.toLowerCase()}"
				use:tooltip={{
					content: entry.status.toLowerCase(),
					placement: 'bottom'
				}}
			/>
		</div>
		<div class="pipeline-card-meta">
			<div>{executionTime} @ {entry.branch}</div>
			<div>
				<a
					href={undefined}
					on:click={rerunPipeline(entry)}
					use:tooltip={{
						content: 'Rerun pipeline',
						placement: 'bottom'
					}}
				>
					<FontAwesomeIcon icon={faArrowRightRotate} />
				</a>
				<a
					href="{currentPath}/pipelines/{entry.id}"
					use:tooltip={{
						content: 'Preview pipeline',
						placement: 'bottom'
					}}
				>
					<FontAwesomeIcon icon={faArrowRight} />
				</a>
			</div>
		</div>
	</div>
</Card>

<style lang="scss">
	@import 'src/styles/auth/projects/id/jobs/id/pipelineCard';
</style>
