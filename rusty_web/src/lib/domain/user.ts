export interface User {
	id: string;
	email: string;
	username: string;
}

export type CredSource = 'GIT_HUB' | 'GITLAB' | 'BITBUCKET';

export const credSourceMap: Record<string, string> = {
	INTERNAL: 'Internal',
	GIT_HUB: 'GitHub',
	GITLAB: 'Gitlab',
	BITBUCKET: 'BitBucket'
};

export interface UserCredential {
	id: string;
	name: string;
	source: CredSource;
	token: string;
	userId: string;
}
