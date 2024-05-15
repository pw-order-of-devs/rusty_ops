export const load = ({ cookies, url }) => {
	return {
		authenticated: (cookies.get('rustyToken') ?? '').length > 0,
		isLoginPage: url.pathname === '/login',
		visited: cookies.get('rustyVisited') === 'true'
	};
};
