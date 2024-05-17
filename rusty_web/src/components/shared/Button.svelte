<script lang="ts">
	import type { MouseEventHandler } from 'svelte/elements';
	import type { IconProp } from '@fortawesome/fontawesome-svg-core';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';

	export let action: MouseEventHandler<HTMLSpanElement> | null = null;
	export let href: string | null = null;
	export let target: string | null = '_self';
	export let icon: IconProp | null = null;
	export let label: string | null = null;
	export let flat: boolean = false;
</script>

<a {href} on:click={action} {target} class:flat>
	{#if icon}
		<FontAwesomeIcon {icon} size="1x" />
	{/if}
	{#if label}
		{label}
	{/if}
</a>

<style>
	a {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background-color: #e2e2e2;
		border-radius: 0.2rem;
		cursor: pointer;
		color: #1b1b1b;
		text-decoration: none;
		font-size: 1.2rem;
	}

	a.flat {
		background-color: transparent;
		color: #e2e2e2;
	}

	a:hover {
		background-color: #c4c4c4;
	}

	a:active {
		background-color: #a6a6a6;
	}

	a.flat:hover {
		background-color: #3d3d3d;
	}

	a.flat:active {
		background-color: #5f5f5f;
	}
</style>
