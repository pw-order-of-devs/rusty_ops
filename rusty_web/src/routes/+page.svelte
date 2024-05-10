<script lang="ts">
	import Content from 'src/components/landing/Content.svelte';
	import Features from 'src/components/landing/Features.svelte';
	import Footer from 'src/components/landing/Footer.svelte';
	import Nav from 'src/components/landing/Nav.svelte';

	import { goto } from '$app/navigation';

	if (window.innerWidth < 768) {
		goto('/error?kind=Not+supported&message=Mobile+view+is+not+supported');
	}
</script>

<Nav />
<Content />
<Features />
<Footer />
