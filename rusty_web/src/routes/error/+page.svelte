<script lang="ts">
	import { page } from '$app/stores';
	import Button from 'src/components/shared/Button.svelte';
	import { faTriangleExclamation } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';

	export let data;
	export let backButtonVisible: boolean = false;

	$: backButtonVisible = data.backButtonVisible;
</script>

<div class="container">
	<FontAwesomeIcon icon={faTriangleExclamation} size="6x" />
	<span class="kind">{$page.url.searchParams.get('kind')}</span>
	<span class="message">{$page.url.searchParams.get('message')}</span>
	{#if backButtonVisible}
		<Button href="/" label="Back home" />
	{/if}
</div>

<style lang="scss">
	@import 'src/styles/error/style';
</style>
