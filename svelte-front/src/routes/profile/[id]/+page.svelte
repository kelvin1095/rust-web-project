<script lang="ts">
  import { goto } from "$app/navigation";
  import type { PageData } from "./$types";
  import { login_status_store } from "../../store";

  export let data: PageData;
  const username = data.username;

  const logout = () => {
    localStorage.removeItem("username");
    document.cookie = `auth-token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;`;
    login_status_store.set(false);
    goto("/");
  };
</script>

<h1>Welcome {username}!</h1>

<button on:click={logout}> Log Out </button>

<style>
  h1 {
    text-align: center;
  }
</style>
