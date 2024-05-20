import { toast } from '@zerodevx/svelte-toast';

export const toastSuccess = (message: string) =>
	toast.push(message, {
		theme: {
			'--toastBackground': '#5cb85c',
			'--toastColor': '#fff',
			'--toastBarBackground': '#fff'
		}
	});

export const toastInfo = (message: string) =>
	toast.push(message, {
		theme: {
			'--toastBackground': '#5bc0de',
			'--toastColor': '#fff',
			'--toastBarBackground': '#fff'
		}
	});

export const toastWarning = (message: string) =>
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
