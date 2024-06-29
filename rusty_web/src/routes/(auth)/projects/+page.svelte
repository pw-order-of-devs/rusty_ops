<script lang="ts">
	import Loader from 'src/components/shared/Loader.svelte';
	import ProjectCard from 'src/components/auth/projects/ProjectCard.svelte';
	import { groupsFilterKeyPressed, groupsListScrolled } from 'src/scripts/auth/projects/groups';
	import { groupClicked } from 'src/scripts/auth/projects/projects';
	import Card from 'src/components/auth/Card.svelte';
	import type { Group } from '$lib/domain/group';
	import { writable } from 'svelte/store';

	let loading = writable(false);
	let loadingGroups = writable(false);
	let scrollableGroups: HTMLElement;
	let groupsFilter = '';
	export let data;

	const groupClicked_ = (entry: Group) => async () => {
		data = await groupClicked(entry, loading, data);
	};

	const groupsFilterKeyPressed_ = async (_: KeyboardEvent) => {
		data = await groupsFilterKeyPressed(loadingGroups, groupsFilter, data);
	};

	const groupsListScrolled_ = async () => {
		data = await groupsListScrolled(scrollableGroups, loadingGroups, groupsFilter, data);
	};
</script>

{#if $loading}
	<Loader />
{/if}

<div class="projects-page">
	<div class="projects-groups-wrapper">
		<input
			class="projects-group-filter"
			type="text"
			placeholder="Group name"
			bind:value={groupsFilter}
			on:keyup={groupsFilterKeyPressed_}
		/>
		<div class="projects-group-default">
			<Card classes={data.groups?.active?.id === '' ? 'active' : ''}>
				<div on:click={groupClicked_({ id: '', name: 'Default' })} role="none">{'Default'}</div>
			</Card>
		</div>
		<div class="projects-group-wrapper">
			{#if $loadingGroups}
				<Loader />
			{/if}

			<div class="projects-groups" bind:this={scrollableGroups} on:scroll={groupsListScrolled_}>
				{#each data.groups?.entries ?? [] as entry (entry.id)}
					<Card classes={data.groups?.active?.id === entry.id ? 'active' : ''}>
						<div on:click={groupClicked_(entry)} role="none">{entry.name}</div>
					</Card>
				{/each}
			</div>
		</div>
	</div>

	{#if (data.projects?.entries ?? []).length === 0}
		<div class="no-entries">No entries</div>
	{:else}
		<div class="entries">
			{#each data.projects?.entries ?? [] as entry (entry.id)}
				<ProjectCard {entry} />
			{/each}
		</div>
	{/if}
</div>

<style lang="scss">
	@import 'src/styles/auth/projects/style';
</style>
