<script lang="ts">
  let result: string;

  let num1 = "";
  let num2 = "";

  const handleSubmit = async (event: Event) => {
    event.preventDefault();
    const formData = {
      num1: num1,
      num2: num2,
    };

    try {
      const response = await fetch("/api/sum", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(formData),
      });

      if (!response.ok) {
        throw new Error(`Error: ${await response.text()}`);
      }

      const data = await response.json();
      result = data.total;
    } catch (error) {
      console.error("Error:", error);
      result = (error as Error).message;
    }
  };
</script>

<svelte:head>
  <title>Calculator</title>
</svelte:head>

<h1>Login Calculator Test Page</h1>
<p>
  This page is to test out logging in with authorization headers and JWT. If you
  are not logged in, then you will see an error returned. If you are logged in,
  the sum of the two numbers will be returned. Of course, if you enter anything
  that isn't a number, you will see an error
</p>

<h2>Adding two numbers with post requests</h2>
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
