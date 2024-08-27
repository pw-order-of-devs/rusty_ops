<script lang="ts">
	import type { User } from '$lib/domain/user';
	import { parseResponse } from '$lib/scripts/utils/parse';
	import { changePassword, deleteAccount } from '$lib/scripts/auth/users';
	import { deleteTokenCookie, setTokenCookie } from '$lib/utils/token';
	import { toastError, toastInfo, toastSuccess } from '$lib/ui/toasts';
	import { goto } from '$app/navigation';
	import ConfirmModal from 'src/components/shared/ConfirmModal.svelte';
	import type { Writable } from 'svelte/store';

	let showModal = false;
	let modalText = 'Are you sure?';
	let action: 'change_password' | 'delete_account' | undefined = undefined;

	let oldPassword: string = '';
	let newPassword: string = '';

	export let loading: Writable<boolean>;
	export let pageData: { user: User } | undefined = undefined;

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

<ConfirmModal visible={showModal} title={modalText} {onConfirm} {onCancel} />

<div>
	<div class="mt-1">
		<label for="username">Username:</label>
		<input
			id="username"
			class="disabled"
			type="text"
			value={pageData?.user?.username ?? ''}
			disabled
		/>
	</div>
	<div class="mt-1">
		<label for="email">Email:</label>
		<input id="email" class="disabled" type="text" value={pageData?.user?.email ?? ''} disabled />
	</div>
	<form class="mt-2">
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
	<form class="mt-2">
		<div>Delete account (irreversible)</div>
		<input type="button" value="Delete account" on:click={deleteAccountModal} />
	</form>
</div>

<style lang="scss">
	@import 'src/styles/auth/account/style.scss';
</style>
