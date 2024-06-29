<script>
	export let classes = '';
</script>

<div class={`card ${classes}`}>
	<slot />
</div>

<style lang="scss">
	@import 'src/styles/auth/card';
</style>
