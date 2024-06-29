<script lang="ts">
	import { tooltip } from '$lib/ui/tooltip';
	import type { Project } from '$lib/domain/project';
	import Card from 'src/components/auth/Card.svelte';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import { faArrowRight, faCode, faPencil } from '@fortawesome/free-solid-svg-icons';

	export let entry: Project;
</script>

<Card>
	<div class="project-card">
		<div class="project-name">{entry.name}</div>
		<div class="project-buttons">
			<a
				href={entry.url}
				target="_blank"
				use:tooltip={{
					content: 'Repository URL',
					placement: 'bottom'
				}}
			>
				<FontAwesomeIcon icon={faCode} />
			</a>
			<a
				href="/projects/{entry.id}"
				use:tooltip={{
					content: 'Edit project',
					placement: 'bottom'
				}}
			>
				<FontAwesomeIcon icon={faPencil} />
			</a>
			<a
				href="/projects/{entry.id}"
				use:tooltip={{
					content: 'Preview project',
					placement: 'bottom'
				}}
			>
				<FontAwesomeIcon icon={faArrowRight} />
			</a>
		</div>
	</div>
</Card>

<style lang="scss">
	@import 'src/styles/auth/projects/projectCard';
</style>
