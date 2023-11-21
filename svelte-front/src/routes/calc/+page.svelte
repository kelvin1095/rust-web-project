<script lang="ts">
  let result: string;

  let num1 = "";
  let num2 = "";

  function getJwtFromCookie() {
    const cookieValue = document.cookie
      .split("; ")
      .find((row) => row.startsWith("jwt="))
      ?.split("=")[1];
    return cookieValue || null;
  }

  const handleSubmit = async (event: Event) => {
    event.preventDefault();
    const formData = {
      num1: num1,
      num2: num2,
    };

    let jwt = getJwtFromCookie();
    console.log(jwt);

    console.log(JSON.stringify(formData));
    console.log(JSON.stringify(formData));

    try {
      const response = await fetch("/api/sum", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${jwt}`,
        },
        body: JSON.stringify(formData),
      });

      // Handle the response as needed
      const data = await response.json();
      console.log("API Response:", data);
      result = data.total;
    } catch (error) {
      console.error("Error:", error);
      result = "error";
    }
  };
</script>

<svelte:head>
  <title>Calculator</title>
</svelte:head>

<h1>Welcome to my Calculator Page!</h1>
<p>This page is to test out get/post requests in forms</p>

<!-- <h2>GET Requests</h2>
<form method="GET" action="/api/sum">
  <label for="num1">First Number</label>
  <input id="num1" name="num1" bind:value={num1} type="input" />

  <label for="num2">Second Number</label>
  <input id="num2" name="num2" bind:value={num2} type="input" />

  <button>Submit!</button>
</form> -->

<h2>POST Requests</h2>
<form on:submit={handleSubmit}>
  <label for="num1">First Number</label>
  <input id="num1" name="num1" bind:value={num1} type="input" />

  <label for="num2">Second Number</label>
  <input id="num2" name="num2" bind:value={num2} type="input" />

  <button>Submit!</button>
</form>

{#if result}
  <div id="Result">
    <p>The result of the sum is: {result}</p>
  </div>
{/if}

<style>
  h1 {
    text-align: center;
  }
</style>
