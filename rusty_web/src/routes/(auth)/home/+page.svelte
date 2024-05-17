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
		height: 100%;
		display: grid;
		grid-template-columns: 1fr 1fr;
		overflow: hidden;

		.widgets {
			display: flex;
			flex-direction: column;
		}
	}
</style>
