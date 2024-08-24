<script lang="ts">
	import {
		contributing,
		discussions,
		docker,
		issues,
		license,
		milestones,
		plugins,
		repository,
		security,
		structure,
		wiki
	} from '$lib/links';
	import { faFile } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
</script>

<div class="footer">
	<div class="misc">
		<div class="support">
			<span> Want to support the project? </span>
			<span>
				<a href={contributing} target="_blank">Contribute</a>
				or report an <a href={issues} target="_blank">issue</a>.
			</span>
			<br />
			<span>
				<a href={license} target="_blank">
					<FontAwesomeIcon icon={faFile} size="1x" />
				</a>
				The project is licenced under MIT license
			</span>
		</div>
		<div class="resources">
			<div class="label">Resources</div>
			<a href={docker} target="_blank">Docker images</a>
			<a href={wiki} target="_blank">Documentation</a>
			<a href={plugins} target="_blank">Plugins</a>
			<a href={security} target="_blank">Security</a>
		</div>
		<div class="project">
			<div class="label">Project</div>
			<a href={structure} target="_blank">Structure</a>
			<a href={repository} target="_blank">Repository</a>
			<a href={issues} target="_blank">Issues</a>
			<a href={milestones} target="_blank">Milestones</a>
		</div>
		<div class="community">
			<div class="label">Community</div>
			<a href={discussions} target="_blank">Discussions</a>
			<a href={contributing} target="_blank">Contributing</a>
		</div>
	</div>
	<br />
	<br />
	<div>Paweł Walus - Order of Devs | 2024</div>
	<br />
</div>

<style lang="scss">
	@import 'src/styles/global';

	.footer {
		margin-top: 6rem;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;

		.misc {
			width: 100%;
			display: flex;
			flex-direction: row;
			justify-content: center;
			gap: 4rem;

			.support,
			.resources,
			.project,
			.community {
				display: flex;
				flex-direction: column;

				.label {
					font-weight: bold;
					margin-bottom: 0.2rem;
				}
			}
		}
	}

	a {
		color: var(--color-secondary-2);
	}
</style>
