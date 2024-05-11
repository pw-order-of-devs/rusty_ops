import { toast } from '@zerodevx/svelte-toast';

export const success = (message: string) =>
	toast.push(message, {
		theme: {
			'--toastBackground': '#5cb85c',
			'--toastColor': '#fff',
			'--toastBarBackground': '#fff'
		}
	});

export const info = (message: string) =>
	toast.push(message, {
		theme: {
			'--toastBackground': '#5bc0de',
			'--toastColor': '#fff',
			'--toastBarBackground': '#fff'
		}
	});

export const warning = (message: string) =>
	toast.push(message, {
		theme: {
			'--toastBackground': '#f0ad4e',
			'--toastColor': '#fff',
			'--toastBarBackground': '#fff'
		}
	});

export const toastError = (message: string) =>
	toast.push(message, {
		theme: {
			'--toastBackground': '#d9534f',
			'--toastColor': '#fff',
			'--toastBarBackground': '#fff'
		}
	});
