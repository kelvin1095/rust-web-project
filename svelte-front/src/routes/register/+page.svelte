<script lang="ts">
  type NewUser = {
    username: string;
    name: string;
    email: string;
    password: string;
  };

  let id = "";
  let password = "";
  let passwordRepeat = "";
  let email = "";
  let preferName = "";

  const handleSubmit = async () => {
    if (password !== passwordRepeat) {
      alert("mismatch password");
    }

    let newUser: NewUser = {
      username: id,
      name: preferName,
      email: email,
      password: password,
    };

    try {
      const response = await fetch("/api/register", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(newUser),
      });

      console.log(response);

      if (!response.ok) {
        throw new Error("ERROR");
      }
    } catch (error) {
      console.error("Error: ", error);
    }
  };
</script>

<svelte:head>
  <title>Register</title>
</svelte:head>

<form on:submit={handleSubmit}>
  <label for="id">Username:</label>
  <input id="id" name="id" bind:value={id} />
  <br />

  <label for="email">Email:</label>
  <input id="email" name="email" type="email" bind:value={email} />
  <br />

  <label for="preferName">Preferred Name:</label>
  <input id="preferName" name="preferName" bind:value={preferName} />
  <br />

  <label for="psd">Password:</label>
  <input id="psd" name="psd" type="password" bind:value={password} />
  <br />

  <label for="psw-repeat">Retype Password:</label>
  <input
    id="psw-repeat"
    name="psw-repeat"
    type="password"
    bind:value={passwordRepeat}
  />
  <br />

  <button>Register</button>
</form>

<p>Already have an account?<a href="/login">Login</a></p>
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
