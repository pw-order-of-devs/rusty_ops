export interface User {
	id: string;
	email: string;
	username: string;
}

export interface UserCredential {
	id: string;
	name: string;
	source: 'GitHub' | 'Gitlab' | 'Bitbucket';
	token: string;
	userId: string;
}
