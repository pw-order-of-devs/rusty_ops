export interface User {
	id: string;
	email: string;
	username: string;
}

export type CredSource = 'GIT_HUB' | 'GITLAB' | 'BITBUCKET';

export interface UserCredential {
	id: string;
	name: string;
	source: CredSource;
	sourceDisplay: string;
	token: string;
	userId: string;
}
