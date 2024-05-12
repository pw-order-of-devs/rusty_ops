interface Error {
	kind: string | null;
	message: string | null;
}

const compare = (a: Error, b: Error) => {
	return a.kind === b.kind && a.message === b.message;
};

const noButton: Error[] = [{ kind: 'Not supported', message: 'Mobile view is not supported' }];

export const load = ({ url }) => {
	let error: Error = {
		kind: url.searchParams.get('kind'),
		message: url.searchParams.get('message')
	};

	return {
		backButtonVisible: noButton.every((item) => !compare(error, item))
	};
};
