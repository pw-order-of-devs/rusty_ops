import { login } from '$lib/api/auth';

export const ssr = false;

export const load = ({ url }) => {
	return {
		redirect: url.searchParams.get('redirectTo') ?? '/home'
	};
};

export const actions = {
	login: async ({ request }) => {
		const credentials = await request.formData();
		return await login(
			credentials.get('login')?.toString() ?? '',
			credentials.get('password')?.toString() ?? ''
		);
	}
};
