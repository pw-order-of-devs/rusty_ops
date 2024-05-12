<script>
	import Button from 'src/components/shared/Button.svelte';
</script>

<nav>
	<div class="app-name">RustyOps</div>
	<Button href="/login" label="Log in" />
</nav>

<style lang="scss">
	nav {
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		align-items: center;
		padding: 0.5rem 2rem;
		box-shadow: 0 0 0 0.02rem #e2e2e2;

		.app-name {
			font-size: 3rem;
		}
	}
</style>
