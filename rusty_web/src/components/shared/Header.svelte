<script lang="ts">
	import moment from 'moment';
	import { goto } from '$app/navigation';
	import { toastInfo } from '$lib/ui/toasts';
	import { deleteTokenCookie } from '$lib/utils/token';
	import Button from 'src/components/shared/Button.svelte';
	import { faSignIn, faSignOut } from '@fortawesome/free-solid-svg-icons';
	import { onDestroy, onMount } from 'svelte';

	export let token = '';
	$: token = token;

	export let authenticated = false;
	$: authenticated = authenticated;

	export let isLoginPage = false;
	$: isLoginPage = isLoginPage;

	let interval: number;

	onMount(() => {
		interval = setInterval(() => {
			let payload = token.split('.');
			if (payload.length >= 2) payload = JSON.parse(atob(payload[1]));
			const exp = moment(payload['exp'] * 1000).utc();
			const now = moment().utc();
			if (exp.isBefore(now)) logout();
		}, 3000);
	});

	onDestroy(() => {
		clearInterval(interval);
	});

	const logout = () => {
		deleteTokenCookie();
		toastInfo('Session has ended');
		goto('/', { replaceState: true, invalidateAll: true });
		return true;
	};
</script>

<nav class:bottom-line={!isLoginPage}>
	<div class="app-name">RustyOps</div>
	{#if !isLoginPage}
		{#if authenticated}
			<Button action={logout} icon={faSignOut} label="Log out" flat />
		{:else}
			<Button href="/login" icon={faSignIn} label="Log in" flat />
		{/if}
	{/if}
</nav>

<style lang="scss">
	@import 'src/styles/global';

	nav {
		height: 4rem;
		padding: 0 2rem;
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		align-items: center;

		.app-name {
			font-size: 2.2rem;
		}
	}

	.bottom-line {
		box-shadow: 0 -0.02rem 0 $color-white-2 inset;
	}
</style>
