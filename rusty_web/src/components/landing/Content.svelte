<script>
	import { faGears } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';

	import { repository, wiki } from 'src/lib/links';
</script>

<div class="content">
	<div class="about">
		<div>
			<div class="welcome-text-1">Welcome to RustyOps</div>
			<div class="welcome-text-2">CI/CD management for your projects</div>
		</div>
		<div style="width: 4rem"></div>
		<FontAwesomeIcon icon={faGears} size="6x" />
	</div>
	<div class="more">
		<span>Explore more about the project:</span>
	</div>
	<div class="buttons">
		<div class="button">
			<a href={repository} target="_blank">Repository</a>
		</div>
		<div class="button">
			<a href={wiki} target="_blank">Documentation</a>
		</div>
	</div>
</div>

<style lang="scss">
	@import 'src/styles/button';

	.content {
		padding: 1rem;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;

		.about {
			margin-top: 8rem;
			display: flex;
			flex-direction: row;
			justify-content: center;
			align-items: center;

			.welcome-text-1 {
				font-size: 3rem;
				display: flex;
				justify-content: end;
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
