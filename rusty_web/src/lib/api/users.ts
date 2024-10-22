import { fetchPost } from '$lib/utils/api';

const getCurrentUserQuery = () => {
	return `query {
		users {
			getCurrent {
				id
				email
				username
			}
		}
	}`;
};

export const getCurrentUser = async (auth: string) => {
	try {
		const response = await fetchPost(auth, JSON.stringify({ query: getCurrentUserQuery() }));

		if (!response.ok) {
			return {
				errors: ['Get user data failed']
			};
		} else {
			const { data, errors } = await response.json();
			if (errors && errors.length > 0) {
				return {
					errors: errors.map((error: { message: string }) => error.message)
				};
			} else if (data) {
				return data?.users?.getCurrent;
			}
		}
	} catch (error) {
		return {
			errors: ['Get user data failed']
		};
	}
};

const getCredentialsQuery = (username: string) => {
	return `query {
		users {
			getUserCredentials(username: "${username}") {
				total
				page
				pageSize
				entries {
					id
					name
					source
					userId
				}
			}
		}
	}`;
};

export const getCredentials = async (auth: string, username: string) => {
	try {
		const response = await fetchPost(
			auth,
			JSON.stringify({ query: getCredentialsQuery(username) })
		);

		if (!response.ok) {
			return {
				errors: ['Get user credentials failed']
			};
		} else {
			const { data, errors } = await response.json();
			if (errors && errors.length > 0) {
				return {
					errors: errors.map((error: { message: string }) => error.message)
				};
			} else if (data) {
				const paged = data?.users?.getUserCredentials;
				return {
					total: paged?.total ?? 0,
					page: paged?.page ?? 1,
					pageSize: paged?.pageSize ?? 20,
					entries: paged?.entries ?? []
				};
			}
		}
	} catch (error) {
		return {
			errors: ['Get user credentials failed']
		};
	}
};

const addCredentialQuery = (username: string, name: string, source: string, token: string) => {
	return `mutation {
		users {
			addCredential(
				username: "${username}",
				credential: {
					name: "${name}"
					source: ${source}
					token: "${token}"
				}
			)
		}
	}`;
};

export const addCredential = async (
	auth: string,
	username: string,
	name: string,
	source: string,
	token: string
) => {
	try {
		const response = await fetchPost(
			auth,
			JSON.stringify({ query: addCredentialQuery(username, name, source, token) })
		);

		if (!response.ok) {
			return {
				errors: ['Add credential failed']
			};
		} else {
			const { data, errors } = await response.json();
			if (errors && errors.length > 0) {
				return {
					errors: errors.map((error: { message: string }) => error.message)
				};
			} else if (data) {
				return data?.users?.addCredential;
			}
		}
	} catch (error) {
		return {
			errors: ['Add credential failed']
		};
	}
};

const revokeCredentialQuery = (username: string, id: string) => {
	return `mutation {
		users {
			revokeCredential(
				username: "${username}",
				id: "${id}"
			)
		}
	}`;
};

export const revokeCredential = async (auth: string, username: string, id: string) => {
	try {
		const response = await fetchPost(
			auth,
			JSON.stringify({ query: revokeCredentialQuery(username, id) })
		);

		if (!response.ok) {
			return {
				errors: ['Revoke credential failed']
			};
		} else {
			const { data, errors } = await response.json();
			if (errors && errors.length > 0) {
				return {
					errors: errors.map((error: { message: string }) => error.message)
				};
			} else if (data) {
				return 'ok';
			}
		}
	} catch (error) {
		return {
			errors: ['Revoke credential failed']
		};
	}
};

const changePasswordQuery = (username: string, oldPassword: string, newPassword: string) => {
	return `mutation {
		users {
			changePassword(
				username: "${username}",
				oldPassword: "${oldPassword}"
				newPassword: "${newPassword}"
			)
		}
	}`;
};

export const changePassword = async (
	auth: string,
	username: string,
	oldPassword: string,
	newPassword: string
) => {
	try {
		const response = await fetchPost(
			auth,
			JSON.stringify({ query: changePasswordQuery(username, oldPassword, newPassword) })
		);

		if (!response.ok) {
			return {
				errors: ['Change password failed']
			};
		} else {
			const { data, errors } = await response.json();
			if (errors && errors.length > 0) {
				return {
					errors: errors.map((error: { message: string }) => error.message)
				};
			} else if (data) {
				return 'ok';
			}
		}
	} catch (error) {
		return {
			errors: ['Change password failed']
		};
	}
};

const deleteAccountQuery = (username: string) => {
	return `mutation {
		users {
			deleteByUsername(
				username: "${username}"
			)
		}
	}`;
};

export const deleteAccount = async (auth: string, username: string) => {
	try {
		const response = await fetchPost(auth, JSON.stringify({ query: deleteAccountQuery(username) }));

		if (!response.ok) {
			return {
				errors: ['Delete account failed']
			};
		} else {
			const { data, errors } = await response.json();
			if (errors && errors.length > 0) {
				return {
					errors: errors.map((error: { message: string }) => error.message)
				};
			} else if (data) {
				return data?.users?.deleteByUsername;
			}
		}
	} catch (error) {
		return {
			errors: ['Delete account failed']
		};
	}
};

const updatePreferencesQuery = (username: string, preferences: string) => {
	return `mutation {
		users {
			updatePreferences(
				username: "${username}",
				preferences: ${JSON.stringify(preferences)},
			)
		}
	}`;
};

export const updatePreferences = async (auth: string, username: string, preferences: string) => {
	try {
		const response = await fetchPost(
			auth,
			JSON.stringify({ query: updatePreferencesQuery(username, preferences) })
		);

		if (!response.ok) {
			return {
				errors: ['Update user preferences failed']
			};
		} else {
			const { data, errors } = await response.json();
			if (errors && errors.length > 0) {
				return {
					errors: errors.map((error: { message: string }) => error.message)
				};
			} else if (data) {
				return data?.users?.updatePreferences;
			}
		}
	} catch (error) {
		return {
			errors: ['Update user preferences failed']
		};
	}
};
