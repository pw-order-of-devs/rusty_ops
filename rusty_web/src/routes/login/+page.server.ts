export const ssr = false;

export const load = ({ url }) => {
	return {
		redirect: url.searchParams.get('redirectTo') ?? '/',
	}
}

export const actions = {
	login: async ({ request, url }) => {
		const credentials = await request.formData();
		const authHeader = credentials.get('login') + ':' + credentials.get('password');

		try {
			let response = await fetch('http://localhost:8000/graphql', {
				method: 'POST',
				headers: {
					ContentType: 'application/json',
					Authorization: `Basic ${btoa(authHeader)}`
				},
				body: JSON.stringify({
					query: `query { auth { login } }`
				})
			});

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
							errors: ['Failed to authenticate']
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
