<script lang="ts">
	import NavLeft from 'src/components/auth/NavLeft.svelte';

	export let data;
</script>

<div class="page">
	<NavLeft current={data.currentPage} />

	<div class="content">
		<slot />
	</div>
</div>

<style lang="scss">
	@import 'src/styles/auth/style';
</style>
