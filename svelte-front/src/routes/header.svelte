<header>
  <div class="headerButton"><a href="/">Home</a></div>
  <h1>Hello! Welcome to my Svelte project!</h1>
  <div class="headerButton"><a href="/login">Login</a></div>
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
