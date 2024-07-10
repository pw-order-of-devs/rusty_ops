<script lang="ts">
	import Card from 'src/components/auth/Card.svelte';
	import ProjectCard from 'src/components/auth/projects/ProjectCard.svelte';
	import Loader from 'src/components/shared/Loader.svelte';
	import type { Group } from '$lib/domain/group';
	import { type ProjectsData } from '$lib/scripts/auth/projects/data';
	import {
		fetchGroups,
		groupsFilterKeyPressed,
		groupsListScrolled
	} from '$lib/scripts/auth/projects/groups';
	import {
		fetchProjects,
		projectsFilterKeyPressed,
		projectsListScrolled
	} from '$lib/scripts/auth/projects/projects';
	import { groupClicked } from '$lib/scripts/auth/projects/projects';
	import { parseResponse } from '$lib/scripts/utils/parse';
	import { writable } from 'svelte/store';
	import { onMount } from 'svelte';

	let loading = writable(false);
	let loadingGroups = writable(false);
	let loadingProjects = writable(false);

	let scrollableGroups: HTMLElement;
	let scrollableProjects: HTMLElement;

	let groupsFilter = '';
	let projectsFilter = '';

	let pageData: ProjectsData | undefined = undefined;

	onMount(async () => {
		loading.update(() => true);
		let groups = await parseResponse(await fetchGroups('', 1));
		let projects = await parseResponse(await fetchProjects('', '', 1));
		pageData = { groups, projects };
		loading.update(() => false);
	});

	const groupClicked_ = (entry: Group) => async () => {
		pageData = await groupClicked(entry, loading, pageData);
		projectsFilter = '';
	};

	const groupsFilterKeyPressed_ = async (_: KeyboardEvent) => {
		pageData = await groupsFilterKeyPressed(loadingGroups, groupsFilter, pageData);
	};

	const groupsListScrolled_ = async () => {
		pageData = await groupsListScrolled(scrollableGroups, loadingGroups, groupsFilter, pageData);
	};

	const projectsListScrolled_ = async () => {
		let groupId = pageData?.groups?.active?.id ?? '';
		pageData = await projectsListScrolled(
			scrollableProjects,
			loadingProjects,
			groupId,
			projectsFilter,
			pageData
		);
	};

	const projectsFilterKeyPressed_ = async (_: KeyboardEvent) => {
		let groupId = pageData?.groups?.active?.id ?? '';
		pageData = await projectsFilterKeyPressed(loadingProjects, groupId, projectsFilter, pageData);
	};
</script>

{#if $loading}
	<Loader />
{/if}

<div class="projects-page">
	<div>
		<input
			class="projects-group-filter"
			type="text"
			placeholder="Group name"
			bind:value={groupsFilter}
			on:keyup={groupsFilterKeyPressed_}
		/>
		<div class="projects-group-default">
			<Card classes={pageData?.groups?.active?.id === '' ? 'active' : ''}>
				<div on:click={groupClicked_({ id: '', name: 'Default' })} role="none">{'Default'}</div>
			</Card>
		</div>
		<div class="projects-groups-wrapper">
			{#if $loadingGroups}
				<Loader />
			{/if}

			<div class="projects-groups" bind:this={scrollableGroups} on:scroll={groupsListScrolled_}>
				{#each pageData?.groups?.entries ?? [] as entry (entry.id)}
					<Card classes={pageData?.groups?.active?.id === entry.id ? 'active' : ''}>
						<div on:click={groupClicked_(entry)} role="none">{entry.name}</div>
					</Card>
				{/each}
			</div>
		</div>
	</div>

	<div>
		<input
			class="projects-filter"
			type="text"
			placeholder="Project name"
			bind:value={projectsFilter}
			on:keyup={projectsFilterKeyPressed_}
		/>

		{#if (pageData?.projects?.entries ?? []).length === 0}
			<div class="no-entries">No entries</div>
		{:else}
			<div class="projects-wrapper">
				{#if $loadingProjects}
					<Loader />
				{/if}

				<div class="entries" bind:this={scrollableProjects} on:scroll={projectsListScrolled_}>
					{#each pageData?.projects?.entries ?? [] as entry (entry.id)}
						<ProjectCard {entry} />
					{/each}
				</div>
			</div>
		{/if}
	</div>
</div>

<style lang="scss">
	@import 'src/styles/auth/projects/style';
</style>
