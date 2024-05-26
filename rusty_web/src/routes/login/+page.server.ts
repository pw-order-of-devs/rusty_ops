import { basicAuthHeader, fetchPost } from '$lib/utils/api';

export const ssr = false;

export const load = ({ url }) => {
	return {
		redirect: url.searchParams.get('redirectTo') ?? '/home'
	};
};

export const actions = {
	login: async ({ request }) => {
		const credentials = await request.formData();

		try {
			let response = await fetchPost(
				basicAuthHeader(credentials),
				JSON.stringify({
					query: `query { auth { login } }`
				})
			);

			if (!response.ok) {
				return {
					errors: ['Authentication Failed']
				};
			} else {
				const { data, errors } = await response.json();
				if (errors && errors.length > 0) {
					return {
						errors: errors.map((error: { message: string }) => error.message)
					};
				} else if (data) {
					let token = data?.auth?.login;
					if (token) {
						return {
							token
						};
					} else {
						return {
							errors: ['Authentication Failed']
						};
					}
				}
			}
		} catch (error) {
			return {
				errors: ['Authentication Failed']
			};
		}
	}
};
