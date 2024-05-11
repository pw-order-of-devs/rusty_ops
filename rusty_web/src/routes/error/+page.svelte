<script lang="ts">
	import { page } from '$app/stores';

	import { faTriangleExclamation } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
</script>

<div class="container">
	<FontAwesomeIcon icon={faTriangleExclamation} size="6x" />
	<span class="kind">{$page.url.searchParams.get('kind')}</span>
	<span class="message">{$page.url.searchParams.get('message')}</span>
	<div class="button">
		<a href="/">Back home</a>
	</div>
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
			font-size: 4rem;
			font-weight: bold;
		}

		.message {
			margin-top: 0.5rem;
			font-size: 2rem;
		}

		.button {
			margin-top: 1rem;
		}
	}
</style>
