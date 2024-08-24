<script lang="ts">
	import Button from 'src/components/shared/Button.svelte';

	export let visible = false;
	export let title = 'Are you sure?';
	export let onConfirm = () => {};
	export let onCancel = () => {};

	let showModal = false;
	$: showModal = visible;

	let confirmText = [];
	$: confirmText = title.split('\n');

	function handleConfirm() {
		onConfirm();
		showModal = false;
	}

	function handleCancel() {
		onCancel();
		showModal = false;
	}
</script>

<div class="modal-container" class:show={showModal}>
	<div class="modal">
		{#each confirmText as line}
			<p>{line}</p>
		{/each}
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
		background: $color-primary-2;
		font-size: 1.5rem;
		font-weight: bold;
		padding: 1rem 3rem;
		border-radius: 0.2rem;
		min-width: 24rem;

		p {
			display: flex;
			justify-content: center;
		}
	}

	.button-container {
		margin-top: 1rem;
		display: flex;
		justify-content: space-between;
	}
</style>
