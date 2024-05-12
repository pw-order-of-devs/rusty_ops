export const load = ({ cookies }) => {
	return {
		visited: cookies.get('rustyVisited') === 'true'
	};
};
