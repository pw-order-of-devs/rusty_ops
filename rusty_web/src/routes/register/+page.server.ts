import { login, register } from '$lib/api/auth';

export const ssr = false;

export const actions = {
	register: async ({ request }) => {
		const data = await request.formData();
		const result = await register(
			data.get('email')?.toString() ?? '',
			data.get('username')?.toString() ?? '',
			data.get('password')?.toString() ?? ''
		);

		if (result?.id) {
			return await login(
				data.get('username')?.toString() ?? '',
				data.get('password')?.toString() ?? ''
			);
		}

		return result;
	}
};
