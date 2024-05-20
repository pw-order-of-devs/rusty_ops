<script lang="ts">
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { mobileCheck } from '$lib/utils/mobile-check';
	import { toastError, toastSuccess } from '$lib/ui/toasts';
	import { setTokenCookie } from '$lib/utils/token';
	import Loader from 'src/components/shared/Loader.svelte';
	import { faGears } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';

	let loading = false;
	export let form;
	export let data;

	$: {
		if (form?.token) {
			setTokenCookie(form.token);
			toastSuccess('Welcome!');
			goto(data.redirect, { replaceState: true, invalidateAll: true });
		} else if (form?.errors) {
			form?.errors.forEach(function (err: string) {
				toastError(err);
			});
			form = null;
		}
	}

	onMount(() => {
		mobileCheck();
	});
</script>

{#if loading}
	<Loader />
{/if}

<div class="container">
	<div class="logo">
		<FontAwesomeIcon icon={faGears} size="10x" />
	</div>
	<form
		class="form"
		method="POST"
		action="?/login"
		use:enhance={() => {
			loading = true;

			return async ({ update }) => {
				await update();
				loading = false;
			};
		}}
	>
		<div class="title">Login</div>
		<input
			type="text"
			placeholder="login"
			name="login"
			autocomplete="off"
			required
			disabled={loading}
		/>
		<input
			type="password"
			placeholder="password"
			name="password"
			autocomplete="off"
			required
			disabled={loading}
		/>
		<button type="submit">Submit</button>
	</form>
</div>

<style lang="scss">
	@import 'src/styles/global';

	.container {
		width: 100vw;
		height: calc(100vh - 4rem);
		display: grid;
		grid-template-columns: 3fr 4fr;

		.logo,
		.form {
			display: flex;
			justify-content: center;
			align-items: center;
		}

		.logo {
			background-color: $color-black-2;
		}

		.form {
			display: flex;
			flex-direction: column;
			justify-content: center;
			align-items: center;

			.title {
				font-size: 2rem;
				font-weight: bold;
			}

			input,
			button {
				box-sizing: border-box;
				margin-top: 1.5rem;
				width: 20rem;
				padding: 0.8rem;
				background-color: $color-black-1;
				color: $color-white-2;
				font-size: 1rem;
				border: 0.05rem $color-white-2 solid;
				border-radius: 2rem;
			}

			button {
				background-color: $color-black-4;
				cursor: pointer;
			}

			button:hover {
				background-color: $color-black-5;
			}

			button:active {
				background-color: $color-black-6;
			}
		}
	}
</style>
