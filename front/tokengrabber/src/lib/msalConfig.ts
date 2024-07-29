import { PublicClientApplication } from "@azure/msal-browser";
import { PUBLIC_CLIENT_ID, PUBLIC_AUTHORITY } from "$env/static/public";

const msalConfig = {
	auth: {
		clientId: PUBLIC_CLIENT_ID,
		authority: PUBLIC_AUTHORITY,
		knownAuthorities: [],
		redirectUri: "http://localhost:5173/auth",
		postLogoutRedirectUri: "http://localhost:5173/",
		navigateToLoginRequestUrl: true,
	},
	cache: {
		cacheLocation: "sessionStorage",
		storeAuthStateInCookie: false,
	},
	system: {
		windowHashTimeout: 60000,
		iframeHashTimeout: 6000,
		loadFrameTimeout: 0,
	},
};

const loginRequest = {
	scopes: [
		"openid",
		"api://" + PUBLIC_CLIENT_ID + "/Infopanel.Login",
		"User.Read",
	]
}

const msalInstance = new PublicClientApplication(msalConfig);

export { msalInstance, loginRequest };
