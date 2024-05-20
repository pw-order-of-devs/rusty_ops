import { goto } from '$app/navigation';

export const mobileCheck = () => {
	if (/Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent)) {
		goto('/error?kind=Not+supported&message=Mobile+view+is+not+supported').then((r) => {});
	}
};
