<script lang="ts">
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { mobileCheck } from '$lib/utils/mobile-check';
	import { toastError, toastSuccess } from '$lib/ui/toasts';
	import { setTokenCookie } from '$lib/utils/token';
	import Loader from 'src/components/shared/Loader.svelte';
	import { faGears } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';

	let loading = false;
	export let form;
	export let data;

	$: {
		if (form?.token) {
			setTokenCookie(form.token);
			localStorage.setItem('preferences', form.preferences);
			toastSuccess('Welcome!');
			goto(data.redirect, { replaceState: true, invalidateAll: true });
		} else if (form?.errors) {
			form?.errors.forEach((err: string) => toastError(err));
			form = null;
		}
	}

	onMount(() => {
		mobileCheck();
	});
</script>

{#if loading}
	<Loader />
{/if}

<div class="container">
	<div class="logo">
		<FontAwesomeIcon icon={faGears} size="10x" />
	</div>
	<form
		class="form"
		method="POST"
		action="?/login"
		use:enhance={() => {
			loading = true;

			return async ({ update }) => {
				await update();
				loading = false;
			};
		}}
	>
		<div class="title">Log In</div>
		<input
			type="text"
			placeholder="username"
			name="login"
			autocomplete="username"
			required
			disabled={loading}
		/>
		<input
			type="password"
			placeholder="password"
			name="password"
			autocomplete="current-password"
			required
			disabled={loading}
		/>
		<button type="submit">Submit</button>
		<div class="login-signup-switch">
			<span>Not a registered user?</span>
			<a href="/register">Sign up</a>
		</div>
	</form>
</div>

<style lang="scss">
	@import 'src/styles/signin/login';
</style>
