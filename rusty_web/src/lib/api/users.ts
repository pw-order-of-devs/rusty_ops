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
