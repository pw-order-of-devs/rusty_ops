<script lang="ts">
	import Card from 'src/components/auth/Card.svelte';
	import Loader from 'src/components/shared/Loader.svelte';
	import type { User } from '$lib/domain/user';
	import { parseResponse } from '$lib/scripts/utils/parse';
	import { changePassword, getCurrentUser } from '$lib/scripts/auth/users';
	import { toastError, toastSuccess } from '$lib/ui/toasts';
	import { writable } from 'svelte/store';
	import { onMount } from 'svelte';
	import { setTokenCookie } from '$lib/utils/token';

	let loading = writable(false);

	export let current = 'profile';

	let pageData: { user: User } | undefined = undefined;
	let oldPassword: string = '';
	let newPassword: string = '';

	onMount(async () => {
		loading.update(() => true);
		let user = await parseResponse(await getCurrentUser());
		pageData = { user };
		loading.update(() => false);
	});

	const setCurrent = (curr: string) => {
		current = curr;
	};

	const changePassword_ = async () => {
		loading.update(() => true);
		let result = await parseResponse(
			await changePassword(pageData?.user.username ?? '', oldPassword, newPassword)
		);
		oldPassword = newPassword = '';
		if (result.token) {
			setTokenCookie(result.token);
			toastSuccess('Changed password');
		}
		if (result.errors && result.errors.length > 0) {
			result.errors.forEach((err: string) => toastError(err));
		}
		loading.update(() => false);
	};
</script>

{#if $loading}
	<Loader />
{/if}

<div class="account-page">
	<div class="account-nav">
		<div on:click={() => setCurrent('profile')} role="none">
			<Card classes={current === 'profile' ? 'active' : ''}>Profile</Card>
		</div>
		<div on:click={() => setCurrent('credentials')} role="none">
			<Card classes={current === 'credentials' ? 'active' : ''}>Credentials</Card>
		</div>
	</div>
	<div class="account-content">
		{#if current === 'profile'}
			<div>
				<label for="username">Username:</label>
				<input
					id="username"
					class="disabled"
					type="text"
					value={pageData?.user?.username ?? ''}
					disabled
				/>
			</div>
			<div>
				<label for="email">Email:</label>
				<input
					id="email"
					class="disabled"
					type="text"
					value={pageData?.user?.email ?? ''}
					disabled
				/>
			</div>
			<form>
				<div>Change password</div>
				<input type="text" name="username" autocomplete="username" style="display: none" />
				<input
					class="enabled"
					type="password"
					placeholder="Old password"
					bind:value={oldPassword}
					autocomplete="current-password"
				/>
				<input
					class="enabled"
					type="password"
					placeholder="New password"
					bind:value={newPassword}
					autocomplete="new-password"
				/>
				<input type="button" value="Submit" on:click={changePassword_} />
			</form>
		{:else if current === 'credentials'}
			credentials list
		{/if}
	</div>
</div>

<style lang="scss">
	@import 'src/styles/auth/account/style.scss';
</style>
