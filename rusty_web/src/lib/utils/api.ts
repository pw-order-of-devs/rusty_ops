import { env } from '$env/dynamic/private';

const apiUrl = () => {
	return env.API_URL ?? 'http://localhost:8000/graphql';
};

export const basicAuthHeader = (data: FormData) => {
	const credentials = data.get('login') + ':' + data.get('password');
	return `Basic ${btoa(credentials)}`;
};

export const bearerAuthHeader = (token: string) => {
	return `Bearer ${token}`;
};

export const fetchPost = async (auth: string, body: string) => {
	return await fetch(apiUrl(), {
		method: 'POST',
		headers: {
			ContentType: 'application/json',
			Authorization: auth
		},
		body: body
	});
};
