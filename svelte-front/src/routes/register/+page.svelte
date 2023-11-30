<script lang="ts">
  type NewUser = {
    username: string;
    email: string;
    password: string;
  };

  let id = "";
  let password = "";
  let passwordRepeat = "";
  let email = "";

  const handleSubmit = async () => {
    const minLength = 8;
    const uppercaseRegex = /[A-Z]/;
    const lowercaseRegex = /[a-z]/;
    const digitRegex = /\d/;

    if (password !== passwordRepeat) {
      alert("mismatch password");
      return;
    }

    if (password.length < minLength) {
      alert("password must be greater than 8 characters long");
      return;
    }

    if (!uppercaseRegex.test(password)) {
      alert("password must contain an uppercase character");
      return;
    }

    if (!lowercaseRegex.test(password)) {
      alert("password must contain a lowercase character");
      return;
    }

    if (!digitRegex.test(password)) {
      alert("password must contain a numeric character");
      return;
    }

    if (!id.length) {
      alert("please enter a username");
      return;
    }

    if (!email.length) {
      alert("please enter a email");
      return;
    }

    let newUser: NewUser = {
      username: id,
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

<p id="register-options">
  Already have an account? <a href="/login">Login Here</a>
</p>

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

  #register-options {
    text-align: center;
  }
</style>
