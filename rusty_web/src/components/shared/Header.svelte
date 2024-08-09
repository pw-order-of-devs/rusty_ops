<script lang="ts">
	import moment from 'moment';
	import { goto } from '$app/navigation';
	import { toastInfo } from '$lib/ui/toasts';
	import { deleteTokenCookie, renewToken, setTokenCookie } from '$lib/utils/token';
	import Button from 'src/components/shared/Button.svelte';
	import { faArrowRotateRight, faSignIn, faSignOut } from '@fortawesome/free-solid-svg-icons';
	import { onDestroy, onMount } from 'svelte';

	export let token = '';
	export let authenticated = false;
	export let isLoginPage = false;

	let authTimeLeft = Number.MAX_SAFE_INTEGER;
	let interval: number;

	onMount(() => {
		interval = setInterval(() => {
			let payload = {};
			let payloadSplit = token.split('.');
			if (payloadSplit.length >= 2) payload = JSON.parse(atob(payloadSplit[1]));
			const exp = moment(payload['exp'] * 1000).utc();
			const now = moment().utc();

			authTimeLeft = Math.floor(exp.diff(now) / 1000);
			if (authenticated && exp.isBefore(now)) logout();
		}, 1000);
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

	const _renewToken = async () => {
		const jwt = await renewToken(token);
		if (jwt !== undefined) {
			token = jwt;
			setTokenCookie(jwt);
		}
	};
</script>

<nav class:bottom-line={!isLoginPage}>
	<div class="app-name">RustyOps</div>
	<div class="session-control">
		{#if !isLoginPage}
			{#if authenticated && authTimeLeft <= 30}
				<div>Session expires in {authTimeLeft} seconds</div>
				<Button action={_renewToken} icon={faArrowRotateRight} label="Renew" flat />
			{/if}
			{#if authenticated}
				<Button action={logout} icon={faSignOut} label="Log out" flat />
			{:else}
				<Button href="/login" icon={faSignIn} label="Log in" flat />
			{/if}
		{/if}
	</div>
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

		.session-control {
			display: flex;
			flex-direction: row;
			align-items: center;
		}
	}

	.bottom-line {
		box-shadow: 0 -0.02rem 0 $color-white-2 inset;
	}
</style>
