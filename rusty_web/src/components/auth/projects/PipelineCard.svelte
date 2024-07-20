<script lang="ts">
	import moment from 'moment';
	import { tooltip } from '$lib/ui/tooltip';
	import type { Pipeline } from '$lib/domain/pipeline';
	import Card from 'src/components/auth/Card.svelte';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import { faArrowRight, faArrowRightRotate } from '@fortawesome/free-solid-svg-icons';

	export let entry: Pipeline;
	export let currentPath: string;
	export let rerunPipeline: any;

	let executionTime: string = '';

	$: {
		let duration = moment.duration(
			moment(entry.endDate).valueOf() - moment(entry.startDate).valueOf()
		);
		if (duration.isValid()) {
			executionTime = 'Executed in ';
			buildRunTime(duration);
		} else {
			duration = moment.duration(moment.now().valueOf() - moment(entry.startDate).valueOf());
			if (duration.isValid()) {
				executionTime = 'Running for ';
				buildRunTime(duration);
			} else {
				executionTime = 'Created ' + moment(entry.registerDate).fromNow();
			}
		}
	}

	const buildRunTime = (duration: moment.Duration) => {
		if (duration.days() > 0) executionTime += duration.days().toString() + 'd ';
		if (duration.hours() > 0) executionTime += duration.hours().toString() + 'h ';
		if (duration.minutes() > 0) executionTime += duration.minutes().toString() + 'd ';
		if (duration.seconds() > 0) executionTime += duration.seconds().toString() + 's ';
		executionTime += duration.milliseconds().toString() + 'ms';
	}
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
