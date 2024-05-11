<script lang="ts">
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import type { IconDefinition } from '@fortawesome/free-solid-svg-icons';

	export let icon: IconDefinition;
	export let title: string;
	export let description: string;
</script>

<div class="feature">
	<FontAwesomeIcon {icon} size="3x" />
	<div>
		<div>{title}</div>
		<div>{description}</div>
	</div>
</div>

<style lang="scss">
	.feature {
		width: 100%;
		display: flex;
		justify-content: start;
		align-items: center;

		div {
			margin-left: 1rem;

			&:nth-child(1) {
				font-size: 1.2rem;
				font-weight: bold;
			}

			&:nth-child(2) {
				font-size: 0.8rem;
			}
		}
	}
</style>
