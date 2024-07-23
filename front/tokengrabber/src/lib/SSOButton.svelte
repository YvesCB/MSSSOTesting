<script lang="ts">
  import { PublicClientApplication } from "@azure/msal-browser";
  import type { MSUser } from "$lib/types";
  import type {
    Configuration,
    AuthenticationResult,
    AccountInfo,
  } from "@azure/msal-browser";

  import { PUBLIC_CLIENT_ID, PUBLIC_AUTHORITY } from "$env/static/public";

  const msalConfig: Configuration = {
    auth: {
      clientId: PUBLIC_CLIENT_ID,
      authority: PUBLIC_AUTHORITY,
      redirectUri: "http://localhost:5173",
    },
  };

  const loginRequest = {
    scopes: [
      "openid",
      "api://" + PUBLIC_CLIENT_ID + "/Infopanel.Login",
      "User.Read",
    ], // Replace with the scopes you need
  };

  let accessToken: string | null = null;
  let tennantId: string | null = null;

  let msUser: MSUser | null = null;

  let errorMessage: string = "";

  let msalInstance = new PublicClientApplication(msalConfig);
  async function handleLogin() {
    try {
      await msalInstance.initialize();
      const response: AuthenticationResult =
        await msalInstance.loginPopup(loginRequest);
      console.log("Login successful:", response);
      accessToken = response.accessToken;
      tennantId = response.account.tenantId;

      let accounts: AccountInfo[] = msalInstance.getAllAccounts();
      msalInstance.setActiveAccount(accounts[0]);
    } catch (error) {
      if (error instanceof Error) {
        errorMessage = error.message;
      } else {
        errorMessage = String(error);
      }
      console.error("Login failed:", error);
    }
  }

  async function getUserInfo() {
    if (!accessToken) {
      console.log("No Accesstoken");
      return;
    }

    var request = {
      scopes: ["User.Read"],
    };

    try {
      let graphToken = await msalInstance.acquireTokenSilent(request);
      const userRes = await fetch("https://graph.microsoft.com/v1.0/me", {
        headers: {
          Authorization: `Bearer ${graphToken.accessToken}`,
          "Content-Type": "application/json",
        },
      });

      if (!userRes.ok) {
        throw new Error(`Failed to fetch user groups: ${userRes.statusText}`);
      }

      const userData = await userRes.json();
      msUser = {
        id: userData.id,
        mail: userData.mail,
        displayName: userData.displayName,
        givenName: userData.givenName,
        surname: userData.surname,
        userPrincipalName: userData.userPrincipalName,
        memberOf: [],
      };

      const groupRes = await fetch(
        "https://graph.microsoft.com/v1.0/me/memberOf",
        {
          headers: {
            Authorization: `Bearer ${graphToken.accessToken}`,
            "Content-Type": "application/json",
          },
        },
      );

      if (!groupRes.ok) {
        throw new Error(`Failed to fetch user groups: ${groupRes.statusText}`);
      }

      const groupData = await groupRes.json();

      // Create a new array to ensure reactivity
      msUser = {
        ...msUser,
        memberOf: groupData.value.map((g: any) => ({
          id: g.id,
          createdDateTime: new Date(g.createdDateTime),
          description: g.description,
          displayName: g.displayName,
          mail: g.mail,
          visibility: g.visibility,
        })),
      };

      console.log(msUser);
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : "Unknown error";
      console.error("Error fetching user groups:", error);
    }
  }
</script>

<button on:click={handleLogin}>Sign in with Microsoft</button>

{#if errorMessage}
  <p style="color: red;">{errorMessage}</p>
{/if}

{#if accessToken && tennantId}
  <div class="info">
    <p><strong>Access Token:</strong> {accessToken}</p>
    <p><strong>Tennant ID:</strong> {tennantId}</p>
  </div>

  <div>
    <button on:click={getUserInfo}>Get User Info</button>
    {#if msUser}
      <div class="userInfo">
        <p><strong>Mail:</strong> {msUser.mail}</p>
        <p><strong>First Name:</strong> {msUser.givenName}</p>
        <p><strong>Last Name:</strong> {msUser.surname}</p>
        <ul>
          {#each msUser.memberOf as group}
            <li>{group.displayName}</li>
          {/each}
        </ul>
      </div>
    {/if}
  </div>
{/if}

<style>
  .info {
    overflow-wrap: anywhere;
  }
</style>
