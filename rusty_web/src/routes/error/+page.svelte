<script lang="ts">
	import { page } from '$app/stores';
	import Button from 'src/components/shared/Button.svelte';
	import { faTriangleExclamation } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';

	export let data;
	export let backButtonVisible: boolean = false;

	$: {
		backButtonVisible = data.backButtonVisible;
	}
</script>

<div class="container">
	<FontAwesomeIcon icon={faTriangleExclamation} size="6x" />
	<span class="kind">{$page.url.searchParams.get('kind')}</span>
	<span class="message">{$page.url.searchParams.get('message')}</span>
	{#if backButtonVisible}
		<Button href="/" label="Back home" />
	{/if}
</div>

<style lang="scss">
	.container {
		width: 100vw;
		height: 100vh;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;

		.kind {
			margin-top: 1rem;
			font-size: 2rem;
			font-weight: bold;
		}

		.message {
			font-size: 1rem;
			margin-top: 0.5rem;
			margin-bottom: 1rem;
		}
	}
</style>
