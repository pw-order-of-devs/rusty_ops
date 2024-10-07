<script lang="ts">
	import { enhance } from '$app/forms';
	import { onMount } from 'svelte';
	import { mobileCheck } from '$lib/utils/mobile-check';
	import { toastError, toastSuccess } from '$lib/ui/toasts';
	import { setTokenCookie } from '$lib/utils/token';
	import Loader from 'src/components/shared/Loader.svelte';
	import { faGears } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import { goto } from '$app/navigation';

	let loading = false;
	export let form;

	$: {
		if (form?.token) {
			setTokenCookie(form.token);
			localStorage.setItem('preferences', form.preferences);
			toastSuccess('Account created successfully!');
			goto('/home', { replaceState: true, invalidateAll: true });
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
		action="?/register"
		use:enhance={() => {
			loading = true;

			return async ({ update }) => {
				await update();
				loading = false;
			};
		}}
	>
		<div class="title">Register</div>
		<input
			type="text"
			placeholder="username"
			name="username"
			autocomplete="username"
			required
			disabled={loading}
		/>
		<input
			type="email"
			placeholder="email"
			name="email"
			autocomplete="email"
			required
			disabled={loading}
		/>
		<input
			type="password"
			placeholder="password"
			name="password"
			autocomplete="new-password"
			required
			disabled={loading}
		/>
		<button type="submit">Submit</button>
		<div class="login-signup-switch">
			<span>Already a registered user?</span>
			<a href="/login">Sign in</a>
		</div>
	</form>
</div>

<style lang="scss">
	@import 'src/styles/signin/register';
</style>
