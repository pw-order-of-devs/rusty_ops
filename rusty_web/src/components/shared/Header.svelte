<script lang="ts">
	import { goto } from '$app/navigation';
	import { toastInfo } from '$lib/toasts';
	import { deleteTokenCookie } from '$lib/token';
	import Button from 'src/components/shared/Button.svelte';
	import { faSignIn, faSignOut } from '@fortawesome/free-solid-svg-icons';

	export let authenticated = false;
	$: authenticated = authenticated;

	export let isLoginPage = false;
	$: isLoginPage = isLoginPage;

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
	nav {
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		align-items: center;
		padding: 0.5rem 2rem;

		.app-name {
			font-size: 3rem;
		}
	}

	.bottom-line {
		box-shadow: 0 0 0 0.02rem #e2e2e2;
	}
</style>
