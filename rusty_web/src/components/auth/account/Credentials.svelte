<script lang="ts">
	import Button from 'src/components/shared/Button.svelte';
	import type { User, UserCredential } from '$lib/domain/user';
	import { parseResponse } from '$lib/scripts/utils/parse.js';
	import { getCredentials } from '$lib/scripts/auth/users';
	import { toastError } from '$lib/ui/toasts';
	import type { Writable } from 'svelte/store';
	import { onMount } from 'svelte';
	import { faBan } from '@fortawesome/free-solid-svg-icons';

	export let loading: Writable<boolean>;
	export let pageData: { user: User } | undefined = undefined;
	let entries: UserCredential[] = [];

	onMount(async () => {
		loading.update(() => true);
		let credentials = await parseResponse(await getCredentials(pageData?.user?.username ?? ''));
		if (credentials.errors) {
			credentials.errors.forEach((e: string) => {
				toastError("Failed to fetch credential: " + e);
			})
		}
		if (credentials.entries) {
			entries = credentials.entries;
		}
		loading.update(() => false);
	});

	const addCredential = () => {}
</script>

<div>
	<div class="mt-1">
		<div class="top-bar">
			<label for="credentials">Credentials:</label>
			<Button action={addCredential} label="Add" />
		</div>
		<div class="credentials-wrapper">
			{#each entries as cred}
				<div class="credential-card">
					<div>
						<div>name {cred.name}</div>
						<div>source: {cred.sourceDisplay}</div>
					</div>
					<div>
						<Button
							action={() => {}}
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
