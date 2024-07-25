<script lang="ts">
  import { onMount } from "svelte";
  import SsoRedirectButton from "$lib/SSORedirectButton.svelte";
  import { msalInstance } from "$lib/msalConfig";
  import type { MSUser } from "$lib/types";
  import type { AccountInfo } from "@azure/msal-browser";

  let msalToken: string | null = null;
  let msUser: MSUser | null = null;

  onMount(async () => {
    await msalInstance.initialize();
    console.log("MSAL: Initialized");

    try {
      const response = await msalInstance.handleRedirectPromise();
      if (response) {
        msalToken = response.accessToken;
        let accounts: AccountInfo[] = msalInstance.getAllAccounts();
        msalInstance.setActiveAccount(accounts[0]);
        await getUserInfo();
      } else {
        console.log("MSAL: Not being redirected");
        let cachedMsalToken = sessionStorage.getItem("msal.account.keys");
        if (cachedMsalToken) {
          msalToken = cachedMsalToken[0];
          let accounts: AccountInfo[] = msalInstance.getAllAccounts();
          msalInstance.setActiveAccount(accounts[0]);
          await getUserInfo();
        }
      }
    } catch (error) {
      console.error("MSAL: Error handling redirect response:", error);
    }
  });

  async function getUserInfo() {
    if (!msalToken) {
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
    } catch (error) {
      msalToken = null;
      console.error("Error fetching user groups:", error);
    }
  }
</script>

<main>
  <h1>Welcome</h1>
  {#if msalToken && msUser}
    <h2>You're logged in as:</h2>
    <p>{msUser.mail}</p>
    <p>{msUser.givenName} {msUser.surname}</p>
    <p>Member of:</p>
    <ul>
      {#each msUser.memberOf as membership}
        <li>{membership.displayName}</li>
      {/each}
    </ul>
  {:else}
    <SsoRedirectButton />
  {/if}
</main>

<style>
  main {
    max-width: 80rem;
  }
</style>
