export type MSGroup = {
	id: string;
	createdDateTime: Date;
	description: string;
	displayName: string;
	mail: string;
	visibility: string;
}

export type MSUser = {
	id: string;
	mail: string;
	displayName: string;
	givenName: string;
	surname: string;
	userPrincipalName: string;
	memberOf: MSGroup[];
}
