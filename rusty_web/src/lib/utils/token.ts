const dateZero = 'Thu, 01 Jan 1970 00:00:00 UTC';

export const deleteTokenCookie = () => {
	document.cookie = `rustyToken=; expires=${dateZero}; path=/;`;
};

export const setTokenCookie = (token: string) => {
	document.cookie = `rustyToken=${token}; expiry=${parseTokenExpiry(token)}; secure=true; path=/;`;
};

const parseTokenExpiry = (token: string) => {
	const parts = token.split('.');
	if (parts.length !== 3) {
		return dateZero;
	}
	const claimsStr = atob(token.split('.')[1]);
	const expClaim = JSON.parse(claimsStr).exp;
	return new Date(parseInt(expClaim)).toUTCString();
};
