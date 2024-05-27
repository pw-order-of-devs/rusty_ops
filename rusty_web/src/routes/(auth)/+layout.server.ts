import { redirect } from '@sveltejs/kit';

export const load = ({ cookies, url }) => {
	if (!cookies.get('rustyToken')) {
		throw redirect(303, `/login?redirectTo=${url.pathname}`);
	}

	return {
		currentPage: url.pathname
	};
};
