const apiUrl = () => import.meta.env.VITE_API_URL ?? 'http://localhost:8000/graphql';

export const basicAuthHeader = (login: string, password: string) => {
	const credentials = login + ':' + password;
	return `Basic ${btoa(credentials)}`;
};

export const bearerAuthHeader = (token: string) => {
	return `Bearer ${token}`;
};

export const fetchPost = async (auth: string | null, body: string) => {
	let headers = new Headers();
	headers.append('Content-Type', 'application/json');
	if (auth !== null) {
		headers.append('Authorization', auth);
	}

	return await fetch(apiUrl(), {
		method: 'POST',
		headers: headers,
		body: body
	});
};
