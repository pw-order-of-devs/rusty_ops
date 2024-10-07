<script lang="ts">
	import type { User, UserCredential } from '$lib/domain/user';
	import { parseResponse } from '$lib/scripts/utils/parse.js';
	import { getCredentials } from '$lib/scripts/auth/users';
	import type { Writable } from 'svelte/store';
	import { onMount } from 'svelte';
	import Button from 'src/components/shared/Button.svelte';

	export let loading: Writable<boolean>;
	export let pageData: { user: User } | undefined = undefined;
	let entries: UserCredential[] = [];

	onMount(async () => {
		loading.update(() => true);
		let credentials = await parseResponse(await getCredentials(pageData?.user?.username ?? ''));
		entries = credentials.entries
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
				{cred}
			{/each}
		</div>
	</div>
</div>

<style lang="scss">
	@import 'src/styles/auth/account/style.scss';
</style>
