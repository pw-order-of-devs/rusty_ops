<script lang="ts">
	import { type TooltipOpts, tooltip } from '$lib/ui/tooltip';
	import type { MouseEventHandler } from 'svelte/elements';
	import type { IconProp } from '@fortawesome/fontawesome-svg-core';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';

	export let action: MouseEventHandler<HTMLSpanElement> | null = null;
	export let href: string | null = null;
	export let target: string | null = '_self';
	export let icon: IconProp | null = null;
	export let label: string | null = null;
	export let flat: boolean = false;
	export let tooltipOpts: TooltipOpts | null = null;
	export let classes = '';
</script>

<a {href} class="button {classes}" on:click={action} {target} class:flat use:tooltip={tooltipOpts}>
	{#if icon}
		<FontAwesomeIcon class="fa" {icon} />
	{/if}
	{#if label}
		{label}
	{/if}
</a>

<style lang="scss">
	@import 'src/styles/global';

	a {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background-color: var(--color-secondary-2);
		border-radius: 0.2rem;
		cursor: pointer;
		color: var(--color-primary-1);
		text-decoration: none;
		font-size: 1.2rem;

		&.flat {
			background-color: transparent;
			color: var(--color-secondary-2);

			&:hover {
				background-color: var(--color-primary-3);
			}

			&:active {
				background-color: var(--color-primary-5);
			}
		}

		&:hover {
			background-color: var(--color-secondary-4);
		}

		&:active {
			background-color: var(--color-secondary-6);
		}
	}
</style>
