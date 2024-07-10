export const parseResponse = async (response: Response) => {
	const resp = (await response.json()).data;
	let parsed = JSON.parse(resp.substring(1, resp.length - 1));
	if (typeof parsed === 'string') {
		parsed = JSON.parse(parsed);
	}

	return parsed;
};
