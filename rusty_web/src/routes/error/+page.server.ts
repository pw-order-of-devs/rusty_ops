export const load = ({ url }) => {
	return {
		backButtonVisible:
			url.searchParams.get('kind') === 'Not supported' &&
			url.searchParams.get('message') === 'Mobile view is not supported'
	};
};
