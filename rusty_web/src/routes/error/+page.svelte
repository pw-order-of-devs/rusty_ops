<script lang="ts">
	import { page } from '$app/stores';

	import { faTriangleExclamation } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';

	export let backButtonVisible: boolean;
</script>

<div class="container">
	<FontAwesomeIcon icon={faTriangleExclamation} size="6x" />
	<span class="kind">{$page.url.searchParams.get('kind')}</span>
	<span class="message">{$page.url.searchParams.get('message')}</span>
	{#if backButtonVisible}
		<div class="button">
			<a href="/">Back home</a>
		</div>
	{/if}
</div>

<style lang="scss">
	@import 'src/styles/button';

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
		}

		.button {
			margin-top: 1rem;
		}
	}
</style>
