<script lang="ts">
	import Feature from 'src/components/landing/Feature.svelte';

	import {
		faCodeBranch,
		faCodeFork,
		faDownload,
		faShieldAlt,
		faSliders,
		faTools
	} from '@fortawesome/free-solid-svg-icons';
</script>

<div class="features">
	<Feature
		icon={faCodeBranch}
		title="Continuous Integration, Continuous Deployment"
		description="RustyOps is a versatile solution for integration and delivery of any project"
	/>
	<Feature
		icon={faDownload}
		title="Quick start"
		description="Delivered as Docker images, RustyOps is easy to boot using Docker(Compose) or Kubernetes configuration"
	/>
	<Feature
		icon={faSliders}
		title="Easily configurable"
		description="RustyOps is a versatile solution for integration and delivery of any project"
	/>
	<Feature
		icon={faCodeFork}
		title="Distributed, Scalable"
		description="RustyOps distributes work around registered agents, which can be scaled and configured according to team's needs"
	/>
	<Feature
		icon={faTools}
		title="Extensible Plugin Ecosystem"
		description="Expand RustyOps functionality with a rich selection of plugins tailored to diverse project needs"
	/>
	<Feature
		icon={faShieldAlt}
		title="Robust Security Measures"
		description="With built-in security features and continuous updates, RustyOps ensures the safety of your CI/CD pipeline and project assets"
	/>
</div>

<style lang="scss">
	.features {
		margin-top: 6rem;
		padding: 3rem;
		background-color: #444;
		display: grid;
		grid-template-columns: 1fr 1fr 1fr;
		grid-template-rows: 1fr 1fr;
		gap: 4rem;
		place-items: center;
	}
</style>
