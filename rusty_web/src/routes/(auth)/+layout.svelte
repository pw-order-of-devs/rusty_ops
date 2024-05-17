<script lang="ts">
	import NavLeft from 'src/components/auth/NavLeft.svelte';
</script>

<div class="page">
	<NavLeft />

	<div class="content">
		<slot />
	</div>
</div>

<style lang="scss">
	.page {
		display: flex;
		flex-direction: row;

		.content {
			width: calc(100vw - 3rem);
			height: calc(100vh - 4rem);
		}
	}
</style>
