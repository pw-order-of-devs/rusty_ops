import { login } from '$lib/api/auth';
import {
	addCredential,
	changePassword,
	deleteAccount,
	getCredentials,
	getCurrentUser,
	revokeCredential,
	updatePreferences
} from '$lib/api/users';
import { bearerAuthHeader } from '$lib/utils/api';

export const actions = {
	getCurrentUser: async ({ cookies }) => {
		const jwtToken = bearerAuthHeader(cookies.get('rustyToken') ?? '');
		return JSON.stringify(await getCurrentUser(jwtToken));
	},
	changePassword: async ({ request, cookies }) => {
		const body = await request.json();
		const jwtToken = bearerAuthHeader(cookies.get('rustyToken') ?? '');
		const result = await changePassword(
			jwtToken,
			body.username,
			body.oldPassword,
			body.newPassword
		);

		if (result === 'ok') {
			const loginResult = await login(body.username, body.newPassword);
			return JSON.stringify(loginResult);
		}

		return JSON.stringify(result);
	},
	deleteAccount: async ({ request, cookies }) => {
		const body = await request.json();
		const jwtToken = bearerAuthHeader(cookies.get('rustyToken') ?? '');
		const result = await deleteAccount(jwtToken, body.username);

		return JSON.stringify(result);
	},
	updatePreferences: async ({ request, cookies }) => {
		const body = await request.json();
		const jwtToken = bearerAuthHeader(cookies.get('rustyToken') ?? '');
		const result = await updatePreferences(jwtToken, body.username, body.preferences);

		return JSON.stringify(result);
	},
	getCredentials: async ({ request, cookies }) => {
		const body = await request.json();
		const jwtToken = bearerAuthHeader(cookies.get('rustyToken') ?? '');
		const result = await getCredentials(jwtToken, body.username);

		return JSON.stringify(result);
	},
	addCredential: async ({ request, cookies }) => {
		const body = await request.json();
		const jwtToken = bearerAuthHeader(cookies.get('rustyToken') ?? '');
		const result = await addCredential(jwtToken, body.username, body.name, body.source, body.token);

		return JSON.stringify(result);
	},
	revokeCredential: async ({ request, cookies }) => {
		const body = await request.json();
		const jwtToken = bearerAuthHeader(cookies.get('rustyToken') ?? '');
		const result = await revokeCredential(jwtToken, body.username, body.id);

		return JSON.stringify(result);
	}
};
