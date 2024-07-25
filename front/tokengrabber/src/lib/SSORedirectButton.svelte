<script lang="ts">
  import { msalInstance } from "$lib/msalConfig";

  async function handleLogin() {
    await msalInstance.initialize();
    msalInstance.loginRedirect();
  }
</script>

<button on:click={handleLogin}>Login</button>
