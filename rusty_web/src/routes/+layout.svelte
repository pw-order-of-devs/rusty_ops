<script lang="ts">
	import { fade } from 'svelte/transition';
	import Button from 'src/components/shared/Button.svelte';
	import Header from 'src/components/shared/Header.svelte';
	import { faCookieBite } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import { SvelteToast } from '@zerodevx/svelte-toast';
	import { onMount } from 'svelte';
	import { afterNavigate } from '$app/navigation';

	export let data;
	let token = '';
	let authenticated = false;
	let isLoginPage = false;
	let isRegisterPage = false;
	let visited = false;

	$: token = data.token ?? '';
	$: authenticated = data.authenticated;
	$: isLoginPage = data.isLoginPage;
	$: isRegisterPage = data.isRegisterPage;
	$: visited = data.visited;

	const acceptCookies = () => {
		document.cookie = 'rustyVisited=true; path=/;';
		visited = true;
		return true;
	};

	afterNavigate(() => {
		const theme =
			JSON.parse(localStorage.getItem('preferences') ?? '{}')?.theme ??
			import.meta.env.VITE_THEME ??
			'dark';
		localStorage.setItem('theme', theme);
		document.documentElement.setAttribute('data-theme', theme);
	});
</script>

<Header {token} {authenticated} {isLoginPage} {isRegisterPage} />

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
	@import 'src/styles/cookies';
</style>
