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
