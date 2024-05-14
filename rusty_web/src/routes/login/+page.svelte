<script lang="ts">
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	import { mobileCheck } from '$lib/mobile-check';
	import { toastError } from '$lib/toasts';
	import Loader from 'src/components/shared/Loader.svelte';

	import { faGears } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';

	let loading = false;
	export let form;
	export let data;

	$: {
		if (form?.token) {
			document.cookie = `rustyToken=${form.token}; path=/;`;
			goto(data.redirect);
		} else if (form?.errors) {
			form?.errors.forEach(function (err: string) {
				toastError(err);
			});
			form = null
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
		<div class="title">Login - RustyOps</div>
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
	.container {
		width: 100vw;
		height: 100vh;
		display: grid;
		grid-template-columns: 3fr 4fr;

		.logo,
		.form {
			display: flex;
			justify-content: center;
			align-items: center;
		}

		.logo {
			background-color: #2c2c2c;
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
				background-color: #1b1b1b;
				color: #e2e2e2;
				font-size: 1rem;
				border: 0.05rem #e2e2e2 solid;
				border-radius: 2rem;
			}

			button {
				background-color: #444;
				cursor: pointer;
			}

			button:hover {
				background-color: #555;
			}

			button:active {
				background-color: #666;
			}
		}
	}
</style>
