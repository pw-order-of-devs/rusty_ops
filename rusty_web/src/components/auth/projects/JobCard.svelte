<script lang="ts">
	import moment from 'moment';
	import { tooltip } from '$lib/ui/tooltip';
	import type { Job } from '$lib/domain/job';
	import Card from 'src/components/auth/Card.svelte';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import { faArrowRight, faPencil } from '@fortawesome/free-solid-svg-icons';

	export let entry: Job;
	export let currentPath: string;
	export let status = 'default';

	$: status = entry.pipelines?.[0]?.status.toLowerCase() ?? 'default';
</script>

<Card classes="job-card-wrapper">
	<div class="job-card">
		<div class="job-card-status">
			<div>#{entry.pipelines?.[0]?.number ?? '0'}</div>
			<div
				class="circle circle-{status}"
				use:tooltip={{
					content: status,
					placement: 'bottom'
				}}
			/>
		</div>
		<div class="job-card-meta">
			<div>
				<div class="wrap-text">{entry.name}</div>
				{#if (entry.pipelines ?? []).length > 0}
					<div>@ {moment(entry.pipelines?.[0].registerDate ?? '').fromNow()}</div>
				{/if}
			</div>
			<div>
				<a
					href="{currentPath}/jobs/{entry.id}"
					use:tooltip={{
						content: 'Edit job',
						placement: 'bottom'
					}}
				>
					<FontAwesomeIcon icon={faPencil} />
				</a>
				<a
					href="{currentPath}/jobs/{entry.id}"
					use:tooltip={{
						content: 'Preview job',
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
	@import 'src/styles/auth/projects/id/jobCard';
</style>
