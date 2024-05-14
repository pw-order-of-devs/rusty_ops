<script lang="ts">
	import type { MouseEventHandler } from 'svelte/elements';

	export let action: MouseEventHandler<HTMLSpanElement> | null = null;
	export let href: string | null = null;
	export let target: string | null = "_self";
	export let label: string;
</script>

{#if href}
	<div class="button">
		<a {href} target="{target}">{label}</a>
	</div>
{/if}
{#if action}
	<button type="button" on:click={action}>{label}</button>
{/if}

<style>
	.button,
	button {
		padding: 0.5rem 1rem;
		background-color: #e2e2e2;
		border-radius: 0.2rem;
		cursor: pointer;
	}

	a,
	button {
		color: #1b1b1b;
		text-decoration: none;
		font-size: 1.2rem;
	}

	button {
		border: none;
	}

	.button:hover,
	button:hover {
		background-color: #c4c4c4;
	}

	.button:active,
	button:active {
		background-color: #a6a6a6;
	}
</style>
