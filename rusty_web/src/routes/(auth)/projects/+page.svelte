<script lang="ts">
	import Loader from 'src/components/shared/Loader.svelte';
	import ProjectCard from 'src/components/auth/projects/ProjectCard.svelte';
	import Card from 'src/components/auth/Card.svelte';
	import type { Group } from '$lib/domain/group';
	import { toastError } from '$lib/ui/toasts';

	let loading = false;
	export let data;

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
</script>

{#if loading}
	<Loader />
{/if}

<div class="projects-page">
	<div class="projects-groups">
		{#each data.groups?.entries ?? [] as entry (entry.id)}
			<Card classes={data.groups?.active?.id === entry.id ? 'active' : ''}>
				<div on:click={groupClicked(entry)} role="none">{entry.name}</div>
			</Card>
		{/each}
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

		.projects-groups {
			display: flex;
			flex-direction: column;
      height: calc(100vh - 6rem);
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
