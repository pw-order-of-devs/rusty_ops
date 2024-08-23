<script lang="ts">
	import Card from 'src/components/auth/Card.svelte';
	import Loader from 'src/components/shared/Loader.svelte';
	import ConfirmModal from 'src/components/shared/ConfirmModal.svelte';
	import type { User } from '$lib/domain/user';
	import { parseResponse } from '$lib/scripts/utils/parse';
	import { changePassword, deleteAccount, getCurrentUser } from '$lib/scripts/auth/users';
	import { toastError, toastInfo, toastSuccess } from '$lib/ui/toasts';
	import { deleteTokenCookie, setTokenCookie } from '$lib/utils/token';
	import { writable } from 'svelte/store';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	let loading = writable(false);
	let showModal = false;
	let modalText = 'Are you sure?';
	let action: 'change_password' | 'delete_account' | undefined = undefined;

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

	const changePasswordModal = () => {
		action = 'change_password';
		modalText = 'Changing password.\nAre you sure?';
		showModal = true;
	};

	const deleteAccountModal = () => {
		action = 'delete_account';
		modalText = 'Deleting account.\nThis action cannot be undone.';
		showModal = true;
	};

	const changePassword_ = async () => {
		loading.update(() => true);
		let result = await parseResponse(
			await changePassword(pageData?.user.username ?? '', oldPassword, newPassword)
		);
		oldPassword = newPassword = '';
		if (result.token) {
			setTokenCookie(result.token);
			toastSuccess('Password was updated');
		}
		if (result.errors && result.errors.length > 0) {
			result.errors.forEach((err: string) => toastError(err));
		}
		loading.update(() => false);
	};

	const deleteAccount_ = async () => {
		loading.update(() => true);
		let result = await parseResponse(await deleteAccount(pageData?.user.username ?? ''));
		if (result.errors) {
			result.errors.forEach((err: string) => toastError(err));
		} else if (result === 1) {
			deleteTokenCookie();
			toastInfo('Account deleted');
			goto('/', { replaceState: true, invalidateAll: true });
		}
		loading.update(() => false);
	};

	const onConfirm = async () => {
		const action_ = action;
		action = undefined;
		showModal = false;

		switch (action_) {
			case 'change_password':
				await changePassword_();
				break;
			case 'delete_account':
				await deleteAccount_();
				break;
		}
	};

	const onCancel = async () => {
		action = undefined;
		showModal = false;
	};
</script>

{#if $loading}
	<Loader />
{/if}

<ConfirmModal visible={showModal} title={modalText} {onConfirm} {onCancel} />

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
				<input type="button" value="Submit" on:click={changePasswordModal} />
			</form>
			<form>
				<div>Delete account (irreversible)</div>
				<input type="button" value="Delete account" on:click={deleteAccountModal} />
			</form>
		{:else if current === 'credentials'}
			credentials list
		{/if}
	</div>
</div>

<style lang="scss">
	@import 'src/styles/auth/account/style.scss';
</style>
