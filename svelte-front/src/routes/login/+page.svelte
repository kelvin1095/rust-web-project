<svelte:head>
  <title>Login</title>
</svelte:head>

<form>
  <label for="email"> Email Address: </label>
  <input id="email" type="email" />
  <br />

  <label for="password">Password:</label>
  <input id="password" type="password" />
  <br />

  <button>Login</button>
</form>

<a href="/signup">Create Account</a>
<a href="/passwordreset">Forgot Password?</a>

<style>
  form {
    width: 20em;
    margin: 0 auto;
    text-align: center;
  }

  label {
    display: block;
    text-align: left;
  }

  input {
    width: 100%;
  }
</style>
