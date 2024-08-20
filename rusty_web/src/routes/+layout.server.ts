export const load = ({ cookies, url }) => {
	return {
		token: cookies.get('rustyToken'),
		authenticated: (cookies.get('rustyToken') ?? '').length > 0,
		isLoginPage: url.pathname === '/login',
		isRegisterPage: url.pathname === '/register',
		visited: cookies.get('rustyVisited') === 'true'
	};
};
