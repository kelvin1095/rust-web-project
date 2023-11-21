<script lang="ts">
  let id = "";
  let password = "";

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

      // Handle the response as needed
      const data = await response.json();
      console.log("API Response:", data);
      document.cookie = `jwt=${data.access_token}; path=/; secure; samesite=strict`;
    } catch (error) {
      console.error("Error:", error);
    }
  };
</script>

<svelte:head>
  <title>Login</title>
</svelte:head>

<form on:submit={handleSubmit}>
  <label for="id">ID:</label>
  <input id="id" name="id" bind:value={id} />
  <br />

  <label for="password">Password:</label>
  <input id="password" type="password" bind:value={password} />
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
