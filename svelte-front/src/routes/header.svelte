<script lang="ts">
  import { userInfo_store } from "./store";

  let userInfo: string;
  userInfo_store.subscribe((value) => {
    userInfo = value;
  });

  $: console.log("hello from header: ", userInfo);
</script>

<header>
  <div class="headerButton"><a href="/">Home</a></div>
  <h1>Hello! Welcome to my Svelte project!</h1>
  <div class="headerButton">
    {#if userInfo}
      <a href="/{userInfo}">{userInfo}</a>
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
