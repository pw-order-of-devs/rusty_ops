<script lang="ts">
	import Card from 'src/components/auth/Card.svelte';
	import Credentials from 'src/components/auth/account/Credentials.svelte';
	import Profile from 'src/components/auth/account/Profile.svelte';
	import Personalization from 'src/components/auth/account/Personalization.svelte';
	import Loader from 'src/components/shared/Loader.svelte';
	import type { User } from '$lib/domain/user';
	import { parseResponse } from '$lib/scripts/utils/parse';
	import { getCurrentUser } from '$lib/scripts/auth/users';
	import { writable } from 'svelte/store';
	import { onMount } from 'svelte';

	let loading = writable(false);
	let pageData: { user: User } | undefined = undefined;

	export let current = 'profile';

	onMount(async () => {
		loading.update(() => true);
		let user = await parseResponse(await getCurrentUser());
		pageData = { user };
		loading.update(() => false);
	});

	const setCurrent = (curr: string) => {
		current = curr;
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
		<div on:click={() => setCurrent('personalization')} role="none">
			<Card classes={current === 'personalization' ? 'active' : ''}>Personalization</Card>
		</div>
		<div on:click={() => setCurrent('credentials')} role="none">
			<Card classes={current === 'credentials' ? 'active' : ''}>Credentials</Card>
		</div>
	</div>
	<div class="account-content">
		{#if current === 'profile'}
			<Profile {pageData} {loading} />
		{:else if current === 'personalization'}
			<Personalization {pageData} {loading} />
		{:else if current === 'credentials'}
			<Credentials {pageData} {loading} />
		{/if}
	</div>
</div>

<style lang="scss">
	@import 'src/styles/auth/account/style.scss';
</style>
