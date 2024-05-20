<script lang="ts">
	import Card from 'src/components/auth/Card.svelte';
</script>

<div class="home-page">
	<Card>
		<div>status</div>
	</Card>
	<div class="widgets">
		<Card>
			<div>item 1</div>
		</Card>
		<Card>
			<div>item 2</div>
		</Card>
	</div>
</div>

<style lang="scss">
	.home-page {
		height: calc(100vh - 6rem);
		display: grid;
		grid-template-columns: 1fr 1fr;
		overflow: hidden;
		gap: 1rem;
		padding: 1rem;

		.widgets {
			display: flex;
			flex-direction: column;
			gap: 0.5rem;
		}
	}
</style>
