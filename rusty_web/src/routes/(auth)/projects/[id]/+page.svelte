<script lang="ts">
	import Card from 'src/components/auth/Card.svelte';
	import JobCard from 'src/components/auth/projects/JobCard.svelte';
	import Button from 'src/components/shared/Button.svelte';
	import Loader from 'src/components/shared/Loader.svelte';
	import { getProjectById } from '$lib/scripts/auth/projects/projects';
	import {
		getProjectJobs,
		jobsFilterKeyPressed,
		jobsListScrolled
	} from '$lib/scripts/auth/projects/jobs';
	import type { ProjectData } from '$lib/scripts/auth/projects/data';
	import { parseResponse } from '$lib/scripts/utils/parse';
	import { writable } from 'svelte/store';
	import { onMount } from 'svelte';
	import { faArrowUpRightFromSquare } from '@fortawesome/free-solid-svg-icons';

	let loading = writable(false);
	let loadingJobs = writable(false);

	let scrollableJobs: HTMLElement;

	let jobsFilter = '';

	let pageData: ProjectData | undefined = undefined;
	export let data;

	onMount(async () => {
		loading.update(() => true);
		let project = await parseResponse(await getProjectById(data['id']));
		let jobs = await parseResponse(await getProjectJobs(data['id'], '', 1));
		pageData = { jobs, project };
		loading.update(() => false);
	});

	const jobsFilterKeyPressed_ = async () => {
		pageData = await jobsFilterKeyPressed(loadingJobs, data['id'], jobsFilter, pageData);
	};

	const jobsListScrolled_ = async () => {
		pageData = await jobsListScrolled(
			scrollableJobs,
			loadingJobs,
			data['id'],
			jobsFilter,
			pageData
		);
	};
</script>

{#if $loading}
	<Loader />
{/if}

<div class="project-page">
	<div class="project-title">
		<Card classes="project-card">
			<div class="project-name wrap-text">
				{pageData?.project?.name}
			</div>
			<div class="project-url">
				<Button
					label={pageData?.project?.url}
					href={pageData?.project?.url}
					classes="project-url-button"
					icon={faArrowUpRightFromSquare}
					tooltipOpts={{ content: 'Project repository', placement: 'bottom' }}
					target="_blank"
					flat
				/>
			</div>
		</Card>
	</div>
	<div class="project-jobs">
		<input
			class="jobs-filter"
			type="text"
			placeholder="Job name"
			bind:value={jobsFilter}
			on:keyup={jobsFilterKeyPressed_}
		/>

		{#if (pageData?.jobs?.entries ?? []).length === 0}
			<div class="no-entries">No entries</div>
		{:else}
			<div class="project-jobs-wrapper">
				{#if $loadingJobs}
					<Loader />
				{/if}

				<div class="entries" bind:this={scrollableJobs} on:scroll={jobsListScrolled_}>
					{#each pageData?.jobs?.entries ?? [] as entry (entry.id)}
						<JobCard {entry} />
					{/each}
				</div>
			</div>
		{/if}
	</div>
</div>

<style lang="scss">
	@import 'src/styles/auth/projects/id/style';
</style>
