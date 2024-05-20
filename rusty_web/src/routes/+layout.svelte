<script lang="ts">
	import { fade } from 'svelte/transition';
	import Button from 'src/components/shared/Button.svelte';
	import Header from 'src/components/shared/Header.svelte';
	import { faCookieBite } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import { SvelteToast } from '@zerodevx/svelte-toast';

	export let data;
	let authenticated = false;
	let isLoginPage = false;
	let visited = false;

	$: authenticated = data.authenticated;
	$: isLoginPage = data.isLoginPage;
	$: visited = data.visited;

	const acceptCookies = () => {
		document.cookie = 'rustyVisited=true; path=/;';
		visited = true;
		return true;
	};
</script>

<Header {authenticated} {isLoginPage} />

<slot />

<SvelteToast />

{#if !visited}
	<div class="cookie-notification" out:fade>
		<FontAwesomeIcon icon={faCookieBite} size="2x" />
		<div>
			By using this website, you agree to use of cookies.
			<br />
			It allows to deliver a better site experience.
		</div>
		<Button action={acceptCookies} label="Accept" />
	</div>
{/if}

<style lang="scss">
	@import 'src/styles/global';

	.cookie-notification {
		position: fixed;
		bottom: 0;
		left: 0;
		right: 0;
		padding: 2rem 8rem;
		background-color: $color-black-1;
		box-shadow: 0 0 0 0.02rem $color-white-2;
		z-index: 9999;
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
		gap: 3rem;
		font-size: 1em;
	}
</style>
