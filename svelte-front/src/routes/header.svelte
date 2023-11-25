<script lang="ts">
  import { onMount } from "svelte";
  import { login_status_store } from "./store";

  let user_id: string | null = localStorage.getItem("username");
  let userInfo: boolean;

  onMount(() => {
    login_status_store.subscribe((value) => {
      userInfo = value;
    });
  });

  $: if (userInfo) {
    user_id = localStorage.getItem("username");
  }
</script>

<header>
  <div class="headerButton"><a href="/">Home</a></div>
  <h1>Kelvin's Coding Adventures!</h1>
  <div class="headerButton">
    {#if userInfo}
      <a href="/profile/{user_id}">{user_id}</a>
    {:else}
      <a href="/login">Login</a>
    {/if}
  </div>
</header>

<style>
  header {
    background-color: rgb(55 65 81);
    color: rgb(243, 244, 246);
    position: sticky;
    display: flex;
    justify-content: space-between;
    padding-top: 2.5rem;
    padding-bottom: 2.5rem;
  }

  .headerButton {
    padding-left: 2.5rem;
    padding-right: 2.5rem;
  }

  h1 {
    text-align: center;
    font-size: 2.25rem;
    line-height: 2.5rem;
  }
</style>
