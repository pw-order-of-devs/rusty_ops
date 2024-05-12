<script lang="ts">
	import { mobileCheck } from '$lib/mobile-check';
	import Features from 'src/components/landing/Features.svelte';
	import Footer from 'src/components/landing/Footer.svelte';
	import Header from 'src/components/landing/Header.svelte';
	import Welcome from 'src/components/landing/Welcome.svelte';

	mobileCheck();
</script>

<Header />
<Welcome />
<Features />
<Footer />
