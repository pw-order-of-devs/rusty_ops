<script lang="ts">
	import { mobileCheck } from '$lib/utils/mobile-check';
	import Features from 'src/components/landing/Features.svelte';
	import Footer from 'src/components/landing/Footer.svelte';
	import Welcome from 'src/components/landing/Welcome.svelte';

	mobileCheck();
</script>

<div class="landing-page">
	<Welcome />
	<Features />
	<Footer />
</div>

<style lang="scss">
	@import 'src/styles/landing';
</style>
