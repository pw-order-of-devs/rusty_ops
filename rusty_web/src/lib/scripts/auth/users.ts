export const getCurrentUser = async () => {
	return await fetch('?/getCurrentUser', {
		method: 'POST',
		body: ''
	});
};

export const changePassword = async (
	username: string,
	oldPassword: string,
	newPassword: string
) => {
	return await fetch('?/changePassword', {
		method: 'POST',
		body: JSON.stringify({ username, oldPassword, newPassword })
	});
};

export const deleteAccount = async (username: string) => {
	return await fetch('?/deleteAccount', {
		method: 'POST',
		body: JSON.stringify({ username })
	});
};

export const updatePreferences = async (username: string, preferences: string) => {
	return await fetch('?/updatePreferences', {
		method: 'POST',
		body: JSON.stringify({ username, preferences })
	});
};

export const getCredentials = async (username: string) => {
	return await fetch('?/getCredentials', {
		method: 'POST',
		body: JSON.stringify({ username })
	});
};

export const addCredential = async (
	username: string,
	name: string,
	source: string,
	token: string
) => {
	return await fetch('?/addCredential', {
		method: 'POST',
		body: JSON.stringify({ username, name, source, token })
	});
};

export const revokeCredential = async (username: string, id: string) => {
	return await fetch('?/revokeCredential', {
		method: 'POST',
		body: JSON.stringify({ username, id })
	});
};
