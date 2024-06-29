<script lang="ts">
	import Card from 'src/components/auth/Card.svelte';
	import ProjectCard from 'src/components/auth/projects/ProjectCard.svelte';
	import Loader from 'src/components/shared/Loader.svelte';
	import type { Group } from '$lib/domain/group';
	import { type Data } from '$lib/scripts/auth/projects/data';
	import {
		fetchGroups,
		parseGroups,
		groupsFilterKeyPressed,
		groupsListScrolled
	} from '$lib/scripts/auth/projects/groups';
	import {
		fetchProjects,
		parseProjects,
		projectsFilterKeyPressed,
		projectsListScrolled
	} from '$lib/scripts/auth/projects/projects';
	import { groupClicked } from '$lib/scripts/auth/projects/projects';
	import { writable } from 'svelte/store';
	import { onMount } from 'svelte';

	let loading = writable(false);
	let loadingGroups = writable(false);
	let loadingProjects = writable(false);

	let scrollableGroups: HTMLElement;
	let scrollableProjects: HTMLElement;

	let groupsFilter = '';
	let projectsFilter = '';

	let data: Data | undefined = undefined;

	onMount(async () => {
		loading.update(() => true);
		let groups = await parseGroups(await fetchGroups('', 1));
		let projects = await parseProjects(await fetchProjects('', '', 1));
		data = { groups, projects };
		loading.update(() => false);
	});

	const groupClicked_ = (entry: Group) => async () => {
		data = await groupClicked(entry, loading, data);
		projectsFilter = '';
	};

	const groupsFilterKeyPressed_ = async (_: KeyboardEvent) => {
		data = await groupsFilterKeyPressed(loadingGroups, groupsFilter, data);
	};

	const groupsListScrolled_ = async () => {
		data = await groupsListScrolled(scrollableGroups, loadingGroups, groupsFilter, data);
	};

	const projectsListScrolled_ = async () => {
		let groupId = data?.groups?.active?.id ?? '';
		data = await projectsListScrolled(
			scrollableProjects,
			loadingProjects,
			groupId,
			projectsFilter,
			data
		);
	};

	const projectsFilterKeyPressed_ = async (_: KeyboardEvent) => {
		let groupId = data?.groups?.active?.id ?? '';
		data = await projectsFilterKeyPressed(loadingProjects, groupId, projectsFilter, data);
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
			<Card classes={data?.groups?.active?.id === '' ? 'active' : ''}>
				<div on:click={groupClicked_({ id: '', name: 'Default' })} role="none">{'Default'}</div>
			</Card>
		</div>
		<div class="projects-groups-wrapper">
			{#if $loadingGroups}
				<Loader />
			{/if}

			<div class="projects-groups" bind:this={scrollableGroups} on:scroll={groupsListScrolled_}>
				{#each data?.groups?.entries ?? [] as entry (entry.id)}
					<Card classes={data?.groups?.active?.id === entry.id ? 'active' : ''}>
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

		{#if (data?.projects?.entries ?? []).length === 0}
			<div class="no-entries">No entries</div>
		{:else}
			<div class="projects-wrapper">
				{#if $loadingProjects}
					<Loader />
				{/if}

				<div class="entries" bind:this={scrollableProjects} on:scroll={projectsListScrolled_}>
					{#each data?.projects?.entries ?? [] as entry (entry.id)}
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
