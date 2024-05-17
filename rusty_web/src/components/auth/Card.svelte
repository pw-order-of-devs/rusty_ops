<div class="card">
	<slot />
</div>

<style lang="scss">
	@import 'src/styles/global';

	.card {
		border-radius: 0.2rem;
		box-shadow: 0 0 0 0.03rem $color-white-2;
		margin: 0.5rem;
		padding: 0.5rem;
	}
</style>
