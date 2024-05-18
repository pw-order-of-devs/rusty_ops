<script lang="ts">
	import Button from 'src/components/shared/Button.svelte';
	import {
		faClipboard,
		faHome,
		faServer,
		faUser,
		faUsers
	} from '@fortawesome/free-solid-svg-icons';
</script>

<div class="nav-left">
	<Button
		href="/home"
		icon={faHome}
		tooltipOpts={{ content: 'Home page', placement: 'right' }}
		flat
	/>
	<Button
		href="/projects"
		icon={faClipboard}
		tooltipOpts={{ content: 'Projects management', placement: 'right' }}
		flat
	/>
	<Button
		href="/account"
		icon={faUser}
		tooltipOpts={{ content: 'User account', placement: 'right' }}
		flat
	/>
	<Button
		href="/settings/users"
		icon={faUsers}
		tooltipOpts={{ content: 'User management', placement: 'right' }}
		flat
	/>
	<Button
		href="/settings/agents"
		icon={faServer}
		tooltipOpts={{ content: 'Agents management', placement: 'right' }}
		flat
	/>
</div>

<style lang="scss">
	@import 'src/styles/global';

	.nav-left {
		display: flex;
		flex-direction: column;
		align-items: center;
		width: 3rem;
		padding: 0.2rem;
		gap: 0.2rem;
		box-shadow: -0.02rem 0 0 $color-white-2 inset;
	}
</style>
