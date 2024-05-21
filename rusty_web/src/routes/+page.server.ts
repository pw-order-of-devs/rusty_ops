import { redirect } from '@sveltejs/kit';

export const ssr = false;

export const load = ({ cookies, url }) => {
	if (cookies.get('rustyToken')) {
		throw redirect(303, `/home`);
	}
};
