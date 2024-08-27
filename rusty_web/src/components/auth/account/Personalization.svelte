<script lang="ts">
	import Button from 'src/components/shared/Button.svelte';
	import type { Writable } from 'svelte/store';
	import { parseResponse } from '$lib/scripts/utils/parse';
	import { updatePreferences } from '$lib/scripts/auth/users';
	import type { User } from '$lib/domain/user';
	import { toastError, toastSuccess } from '$lib/ui/toasts';

	export let loading: Writable<boolean>;
	export let pageData: { user: User } | undefined = undefined;

	let themes = ['dark', 'dark-purple', 'dark-green', 'light', 'light-purple', 'light-green'];
	let currentTheme = localStorage.getItem('theme') ?? import.meta.env.theme;

	const setCurrentTheme = (theme: string) => {
		currentTheme = theme;
		document.documentElement.setAttribute('data-theme', theme);
	};

	const savePreferences = async () => {
		loading.update(() => true);
		let preferences = JSON.parse(localStorage.getItem('preferences') ?? '{}');
		preferences['theme'] = currentTheme;
		let result = await parseResponse(
			await updatePreferences(pageData?.user.username ?? '', JSON.stringify(preferences))
		);
		if (result.errors && result.errors.length > 0) {
			result.errors.forEach((err: string) => toastError(err));
		} else {
			localStorage.setItem('preferences', JSON.stringify(preferences));
			localStorage.setItem('theme', currentTheme);
			toastSuccess('Preferences updated');
		}
		loading.update(() => false);
	};
</script>

<div>
	<div class="mt-1">
		<div class="top-bar">
			<label for="themes">Theme:</label>
			<Button action={savePreferences} label="Save" />
		</div>
		<div class="themes-wrapper">
			{#each themes as theme}
				<div class="theme-wrapper" on:click={() => setCurrentTheme(theme)} role="none">
					<div class={theme === currentTheme ? 'active' : ''} data-theme={theme}>
						<div />
						<div />
					</div>
					<span>{theme}</span>
				</div>
			{/each}
		</div>
	</div>
</div>

<style lang="scss">
	@import 'src/styles/auth/account/style';
</style>
