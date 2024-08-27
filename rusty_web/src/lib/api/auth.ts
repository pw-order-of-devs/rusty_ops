import { basicAuthHeader, bearerAuthHeader, fetchPost } from '$lib/utils/api';

const getPreferencesQuery = () => {
	return `query {
		users {
			getCurrent {
				preferences
			}
		}
	}`;
};

export const getPreferences = async (auth: string) => {
	try {
		const response = await fetchPost(auth, JSON.stringify({ query: getPreferencesQuery() }));

		if (!response.ok) {
			return {
				errors: ['Get user preferences failed']
			};
		} else {
			const { data, errors } = await response.json();
			if (errors && errors.length > 0) {
				return {
					errors: errors.map((error: { message: string }) => error.message)
				};
			} else if (data) {
				return data?.users?.getCurrent?.preferences;
			}
		}
	} catch (error) {
		return {
			errors: ['Get user preferences failed']
		};
	}
};

export const login = async (login: string, password: string) => {
	try {
		const response = await fetchPost(
			basicAuthHeader(login, password),
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
				const token = data?.auth?.login;
				if (token) {
					const preferences = await getPreferences(bearerAuthHeader(token));
					return { preferences, token };
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
};

export const register = async (email: string, username: string, password: string) => {
	const query = JSON.stringify({
		query: `mutation {
				users {
				  register(user: {
					  email: "${email}",
					  username: "${username}",
					  password: "${password}",
				  })
			  }
			}`
	});

	try {
		const response = await fetchPost(null, query);
		if (!response.ok) {
			return {
				errors: ['User registration Failed']
			};
		}
		const { data, errors } = await response.json();
		if (errors && errors.length > 0) {
			return {
				errors: errors.map((error: { message: string }) => error.message)
			};
		} else if (data) {
			const id = data?.users?.register;
			if (id) {
				return {
					id
				};
			} else {
				return {
					errors: ['User registration Failed']
				};
			}
		}
	} catch (error) {
		return {
			errors: ['User registration Failed']
		};
	}
};
