<script lang="ts">
	import Loader from 'src/components/shared/Loader.svelte';
	import ProjectCard from 'src/components/auth/projects/ProjectCard.svelte';
	import Card from 'src/components/auth/Card.svelte';
	import type { Group } from '$lib/domain/group';
	import { toastError } from '$lib/ui/toasts';

	let loading = false;
	let loadingGroups = false;
	let scrollableGroups: HTMLElement;
	let groupsFilter = '';
	export let data;

	const groupsFilterKeyPressed = async (_: KeyboardEvent) => {
		loadingGroups = true;
		const response = await fetch('?/fetchGroups', {
			method: 'POST',
			body: JSON.stringify({ groupName: groupsFilter, pageNumber: 1 })
		});

		if (!response.ok) {
			toastError('Error while fetching groups');
		} else {
			const resp = (await response.json()).data;
			let parsed = JSON.parse(resp.substring(1, resp.length - 1));
			if (typeof parsed === 'string') {
				parsed = JSON.parse(parsed);
			}

			data.groups = parsed;
			loadingGroups = false;
		}
	};

	const groupClicked = (entry: Group) => async () => {
		if (entry.id === data.groups?.active?.id) {
			return;
		}
		loading = true;
		data.groups!.active = entry;
		const response = await fetch('?/fetchProjects', {
			method: 'POST',
			body: JSON.stringify({ groupId: `"${entry.id}"`, pageNumber: 1 })
		});

		if (!response.ok) {
			toastError('Error while fetching projects');
		} else {
			const resp = (await response.json()).data;
			let parsed = JSON.parse(resp.substring(1, resp.length - 1));
			if (typeof parsed === 'string') {
				parsed = JSON.parse(parsed);
			}

			data.projects!.entries = parsed.entries;
		}
		loading = false;
	};

	const groupsListScrolled = async () => {
		if (
			scrollableGroups.scrollTop + scrollableGroups.clientHeight >=
			scrollableGroups.scrollHeight
		) {
			if (data.groups!.page * data.groups!.pageSize >= data.groups!.total) {
				return;
			}

			loadingGroups = true;
			const response = await fetch('?/fetchGroups', {
				method: 'POST',
				body: JSON.stringify({ groupName: groupsFilter, pageNumber: data.groups!.page + 1 })
			});

			if (!response.ok) {
				toastError('Error while fetching groups');
			} else {
				const resp = (await response.json()).data;
				let parsed = JSON.parse(resp.substring(1, resp.length - 1));
				if (typeof parsed === 'string') {
					parsed = JSON.parse(parsed);
				}

				parsed.entries = [...data.groups!.entries!, ...parsed.entries];
				data.groups! = parsed;
				loadingGroups = false;
			}
		}
	};
</script>

{#if loading}
	<Loader />
{/if}

<div class="projects-page">
	<div class="projects-groups-wrapper">
		<input
			class="projects-group-filter"
			type="text"
			placeholder="Group name"
			bind:value={groupsFilter}
			on:keyup={groupsFilterKeyPressed}
		/>
		<div class="projects-group-default">
			<Card classes={data.groups?.active?.id === '' ? 'active' : ''}>
				<div on:click={groupClicked({ id: '', name: 'Default' })} role="none">{'Default'}</div>
			</Card>
		</div>
		<div class="projects-groups" bind:this={scrollableGroups} on:scroll={groupsListScrolled}>
			{#if loadingGroups}
				<Loader />
			{/if}

			{#each data.groups?.entries ?? [] as entry (entry.id)}
				<Card classes={data.groups?.active?.id === entry.id ? 'active' : ''}>
					<div on:click={groupClicked(entry)} role="none">{entry.name}</div>
				</Card>
			{/each}
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
	@import 'src/styles/global';

	.projects-page {
		display: flex;
		flex-direction: row;

		.projects-group-filter {
			box-sizing: border-box;
			margin: 0.5rem;
			width: 25rem;
			padding: 0.4rem;
			background-color: $color-black-1;
			color: $color-white-2;
			font-size: 1rem;
			border: 0.05rem $color-white-2 solid;
			border-radius: 0.5rem;
		}

		.projects-group-default {
			height: 2rem;
			width: 25rem;
			padding: 0.5rem;

			:global(.active) {
				background-color: $color-black-2;
			}
		}

		.projects-groups {
			position: relative;
			display: flex;
			flex-direction: column;
			height: calc(100vh - 11rem);
			width: 25rem;
			padding: 0.5rem;
			gap: 0.5rem;
			overflow-y: auto;

			:global(.card) {
				padding: 0;
			}

			div {
				max-width: 25rem;
				padding: 0.5rem;
				cursor: pointer;
				white-space: nowrap;
				overflow: hidden;
				text-overflow: ellipsis;
			}

			:global(.active) {
				background-color: $color-black-2;
			}
		}

		.no-entries {
			padding: 1rem;
			font-size: 2rem;
		}

		.entries {
			height: calc(100vh - 6rem);
			width: calc(100vw - 25rem - 6rem);
			display: grid;
			grid-template-columns: 1fr 1fr 1fr;
			grid-template-rows: 1fr 1fr 1fr 1fr 1fr;
			gap: 0.5rem;
			padding: 0.5rem;
			overflow-y: auto;
		}
	}
</style>
