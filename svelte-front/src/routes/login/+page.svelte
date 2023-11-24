<script lang="ts">
  import { goto } from "$app/navigation";
  import { login_status_store } from "../store";

  let id = "";
  let password = "";
  let error_message = "";

  const handleSubmit = async (event: Event) => {
    event.preventDefault();
    const formData = {
      id: id,
      password: password,
    };

    console.log(JSON.stringify(formData));

    try {
      const response = await fetch("/api/auth", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(formData),
      });

      if (!response.ok) {
        throw new Error(
          `HTTP ${response.status} error! Status: ${await response.text()}`
        );
      }

      console.log("API Response:", response.status);
      login_status_store.set(true);
      localStorage.setItem("username", id);

      console.log("hello from login: ", id);
      goto("/");
    } catch (error) {
      console.error("Error:", error);
      error_message = (error as Error).message;
    }
  };
</script>

<svelte:head>
  <title>Login</title>
</svelte:head>

<form on:submit={handleSubmit}>
  <label for="id">Username:</label>
  <input id="id" name="id" bind:value={id} />
  <br />

  <label for="password">Password:</label>
  <input id="password" type="password" bind:value={password} />
  <br />

  <button>Login</button>
</form>

<a href="/register">Create Account</a>
<a href="/passwordreset">Forgot Password?</a>

{error_message}

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
