<script lang="ts">
	import AddCredentialModal from 'src/components/auth/account/AddCredentialModal.svelte';
	import Button from 'src/components/shared/Button.svelte';
	import { credSourceMap, type User, type UserCredential } from '$lib/domain/user';
	import { parseResponse } from '$lib/scripts/utils/parse.js';
	import { getCredentials, revokeCredential } from '$lib/scripts/auth/users';
	import { toastError, toastSuccess } from '$lib/ui/toasts';
	import type { Writable } from 'svelte/store';
	import { onMount } from 'svelte';
	import { faBan } from '@fortawesome/free-solid-svg-icons';

	export let loading: Writable<boolean>;
	export let pageData: { user: User } | undefined = undefined;
	let entries: UserCredential[] = [];
	let showModal = false;

	onMount(async () => {
		loading.update(() => true);
		let credentials = await parseResponse(await getCredentials(pageData?.user?.username ?? ''));
		if (credentials.errors) {
			credentials.errors.forEach((e: string) => {
				toastError('Failed to fetch credential: ' + e);
			});
		}
		if (credentials.entries) {
			entries = credentials.entries;
		}
		loading.update(() => false);
	});

	const _revokeCredential = async (credential: UserCredential) => {
		loading.update(() => true);
		let result = await parseResponse(
			await revokeCredential(pageData?.user.username ?? '', credential.id)
		);
		if (result.errors && result.errors.length > 0) {
			result.errors.forEach((err: string) => toastError(err));
		} else {
			entries = entries.filter((item) => item !== credential);
			toastSuccess('Credential revoked successfully');
		}
		loading.update(() => false);
	};

	const credentialAdded = async (credential: UserCredential) => {
		entries.push(credential);
		entries = entries;
	};

	const hideDialog = async () => {
		showModal = false;
	};
</script>

<AddCredentialModal {hideDialog} {credentialAdded} {pageData} {loading} visible={showModal} />

<div>
	<div class="mt-1">
		<div class="top-bar">
			<label for="credentials">Credentials:</label>
			<Button action={() => (showModal = true)} label="Add" />
		</div>
		<div class="credentials-wrapper">
			{#each entries as cred}
				<div class="credential-card">
					<div>
						<div>Name: {cred.name}</div>
						<div>source: {credSourceMap[cred.source]}</div>
					</div>
					<div>
						<Button
							action={() => _revokeCredential(cred)}
							icon={faBan}
							tooltipOpts={{ content: 'Revoke credential', placement: 'bottom' }}
							flat
						/>
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>

<style lang="scss">
	@import 'src/styles/auth/account/style.scss';
</style>
