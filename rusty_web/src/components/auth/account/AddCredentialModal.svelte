<script lang="ts">
	import Button from 'src/components/shared/Button.svelte';
	import { type User, type UserCredential } from '$lib/domain/user';
	import { parseResponse } from '$lib/scripts/utils/parse';
	import { addCredential } from '$lib/scripts/auth/users';
	import type { Writable } from 'svelte/store';
	import { toastError, toastSuccess } from '$lib/ui/toasts';

	export let loading: Writable<boolean>;
	export let pageData: { user: User } | undefined = undefined;
	export let visible = false;
	export let hideDialog = () => {};
	export let credentialAdded = (credential: UserCredential) => {};

	let showModal = false;
	$: showModal = visible;

	let credName = '';
	let credToken = '';
	let credSource = import.meta.env.VITE_DEFAULT_PROJECT_SOURCE;

	const handleConfirm = async () => {
		loading.update(() => true);
		let result = await parseResponse(
			await addCredential(pageData?.user.username ?? '', credName, credSource, credToken)
		);
		if (result.errors && result.errors.length > 0) {
			result.errors.forEach((err: string) => toastError(err));
		} else {
			credentialAdded({
				id: result,
				name: credName,
				source: credSource,
				token: credToken,
				userId: pageData?.user.id ?? ''
			});
			toastSuccess('New credential registered successfully');
		}
		loading.update(() => false);
		hideDialog();
		showModal = false;
	};

	const handleCancel = async () => {
		hideDialog();
		showModal = false;
	};
</script>

<div class="modal-container" class:show={showModal}>
	<div class="modal">
		<p>Register new credential</p>
		<input id="new-credential-name" type="text" placeholder="Name" bind:value={credName} />
		<select id="new-credential-source" bind:value={credSource}>
			<option value="GIT_HUB">GitHub</option>
		</select>
		<input id="new-credential-token" type="password" placeholder="Token" bind:value={credToken} />
		<div class="button-container">
			<Button action={handleCancel} label="Cancel" flat />
			<Button action={handleConfirm} label="Confirm" flat />
		</div>
	</div>
</div>

<style lang="scss">
	@import 'src/styles/global';

	.modal-container {
		position: fixed;
		top: 0;
		left: 0;
		width: 100vw;
		height: 100vh;
		background: rgba(0, 0, 0, 0.3);
		display: none;
		justify-content: center;
		align-items: center;

		&.show {
			display: flex;
		}
	}

	.modal {
		display: flex;
		flex-direction: column;
		background: var(--color-primary-2);
		font-size: 1.5rem;
		font-weight: bold;
		padding: 1rem 3rem;
		border-radius: 0.2rem;
		min-width: 24rem;

		p {
			display: flex;
			justify-content: center;
		}

		input[type='text'],
		input[type='password'],
		select {
			box-sizing: border-box;
			margin: 0.5rem;
			width: 25rem;
			padding: 0.4rem;
			background-color: var(--color-primary-1);
			color: var(--color-secondary-2);
			font-size: 1rem;
			border: 0.05rem var(--color-secondary-2) solid;
			border-radius: 0.5rem;
		}
	}

	.button-container {
		margin-top: 1rem;
		display: flex;
		justify-content: space-between;
	}
</style>
