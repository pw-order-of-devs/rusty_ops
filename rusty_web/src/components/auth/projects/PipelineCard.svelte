<script lang="ts">
	import moment from 'moment';
	import { tooltip } from '$lib/ui/tooltip';
	import type { Pipeline } from '$lib/domain/pipeline';
	import Card from 'src/components/auth/Card.svelte';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import { faArrowRight } from '@fortawesome/free-solid-svg-icons';

	export let entry: Pipeline;
	export let currentPath: string;

	let executionTime: string = '';

	$: {
		let duration = moment.duration(
			moment(entry.endDate).valueOf() - moment(entry.startDate).valueOf()
		);
		if (duration.isValid()) {
			executionTime = '';
			if (duration.days() > 0) executionTime += duration.days().toString() + 'd ';
			if (duration.hours() > 0) executionTime += duration.hours().toString() + 'h ';
			if (duration.minutes() > 0) executionTime += duration.minutes().toString() + 'd ';
			if (duration.seconds() > 0) executionTime += duration.seconds().toString() + 's ';
			executionTime += duration.milliseconds().toString() + 'ms';
		} else {
			executionTime = '-';
		}
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
			<div>Execution time: {executionTime}</div>
			<div>
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
