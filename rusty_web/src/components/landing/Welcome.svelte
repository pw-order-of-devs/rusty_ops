<script lang="ts">
	import { repository, wiki } from '$lib/links';
	import Button from 'src/components/shared/Button.svelte';
</script>

<div class="content">
	<div class="about">
		<div>
			<div class="welcome-text-1">Welcome to RustyOps</div>
			<div class="welcome-text-2">CI/CD management for your projects</div>
		</div>
	</div>
	<div class="more">
		<span>Explore more about the project:</span>
	</div>
	<div class="buttons">
		<Button href={repository} target="_blank" label="Repository" />
		<Button href={wiki} target="_blank" label="Documentation" />
	</div>
</div>

<style lang="scss">
	.content {
		padding: 1rem;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;

		.about {
			margin-top: 8rem;
			display: flex;
			flex-direction: column;
			justify-content: center;
			align-items: center;
			gap: 2rem;

			.welcome-text-1 {
				font-size: 3rem;
				display: flex;
				justify-content: center;
			}

			.welcome-text-2 {
				margin-top: 0.3rem;
				font-size: 2rem;
			}
		}

		.more {
			margin-top: 3rem;
			font-size: 1.5rem;
		}

		.buttons {
			margin-top: 1rem;
			display: flex;
			flex-direction: row;
			justify-content: center;
			align-items: center;
			gap: 2rem;
		}
	}
</style>
