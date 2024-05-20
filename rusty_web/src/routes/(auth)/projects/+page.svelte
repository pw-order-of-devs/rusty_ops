<script lang="ts">
	import ProjectCard from 'src/components/auth/projects/ProjectCard.svelte';

	export let data;
</script>

{#if (data.entries ?? []).length === 0}
	<div>No entries</div>
{:else}
	<div class="entries">
		{#each data.entries ?? [] as entry (entry.id)}
			<ProjectCard {entry} />
		{/each}
	</div>
{/if}

<style lang="scss">
	.entries {
		height: calc(100vh - 6rem);
		display: grid;
		grid-template-columns: 1fr 1fr 1fr 1fr;
		grid-template-rows: 1fr 1fr 1fr 1fr 1fr;
		gap: 0.5rem;
		padding: 1rem;
	}
</style>
