<script lang="ts">
  import { onMount } from "svelte";

  import { msalInstance } from "$lib/msalConfig";

  onMount(() => {
    msalInstance
      .initialize()
      .then(() => {
        console.log("initialized");

        msalInstance
          .handleRedirectPromise()
          .then((response) => {
            if (response) {
              console.log(
                "Login successful, access token:",
                response.accessToken,
              );
            }
          })
          .catch((error) => {
            console.error("Error handling redirect response:", error);
          });
      })
      .catch((error) => {
        console.log("Could not initialize: ", error);
      });
  });
</script>

<main>
  <h1>You are being redirected...</h1>
</main>
