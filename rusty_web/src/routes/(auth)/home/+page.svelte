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
	@import 'src/styles/auth/home/style';
</style>
