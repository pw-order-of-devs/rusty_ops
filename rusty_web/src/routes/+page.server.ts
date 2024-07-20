import { redirect } from '@sveltejs/kit';

export const ssr = false;

export const load = ({ cookies }) => {
	if (cookies.get('rustyToken')) {
		throw redirect(303, `/home`);
	}
};
