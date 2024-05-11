<script lang="ts">
	import Content from 'src/components/landing/Content.svelte';
	import Features from 'src/components/landing/Features.svelte';
	import Footer from 'src/components/landing/Footer.svelte';
	import Nav from 'src/components/landing/Nav.svelte';

	import { mobileCheck } from 'src/lib/mobile-check';
	mobileCheck();
</script>

<Nav />
<Content />
<Features />
<Footer />
