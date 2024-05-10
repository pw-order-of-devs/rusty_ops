<nav>
	<div class="app-name">RustyOps</div>
	<div class="button">
		<a href="/login">Log in</a>
	</div>
</nav>

<style lang="scss">
	@import 'src/styles/button';

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
