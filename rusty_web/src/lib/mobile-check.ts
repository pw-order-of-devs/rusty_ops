import { goto } from '$app/navigation';

export const mobileCheck = () => {
	if (window.innerWidth < 768) {
		goto('/error?kind=Not+supported&message=Mobile+view+is+not+supported');
	}
};
