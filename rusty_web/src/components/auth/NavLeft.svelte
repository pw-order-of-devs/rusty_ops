<script lang="ts">
	import Button from 'src/components/shared/Button.svelte';
	import {
		faClipboard,
		faHome,
		faServer,
		faUser,
		faUsers
	} from '@fortawesome/free-solid-svg-icons';

	export let current: string;
</script>

<div class="nav-left">
	<Button
		href="/home"
		classes={current === '/home' ? 'active' : ''}
		icon={faHome}
		tooltipOpts={{ content: 'Home page', placement: 'right' }}
		flat
	/>
	<Button
		href="/projects"
		classes={current === '/projects' ? 'active' : ''}
		icon={faClipboard}
		tooltipOpts={{ content: 'Projects management', placement: 'right' }}
		flat
	/>
	<Button
		href="/account"
		classes={current === '/account' ? 'active' : ''}
		icon={faUser}
		tooltipOpts={{ content: 'User account', placement: 'right' }}
		flat
	/>
	<Button
		href="/settings/users"
		classes={current === '/settings/users' ? 'active' : ''}
		icon={faUsers}
		tooltipOpts={{ content: 'User management', placement: 'right' }}
		flat
	/>
	<Button
		href="/settings/agents"
		classes={current === '/settings/agents' ? 'active' : ''}
		icon={faServer}
		tooltipOpts={{ content: 'Agents management', placement: 'right' }}
		flat
	/>
</div>

<style lang="scss">
	@import 'src/styles/auth/navLeft';
</style>
