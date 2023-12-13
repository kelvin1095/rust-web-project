<script lang="ts">
    type CalculationResult = {
        total: number;
    };

    type ApiResponseTemplate<T> = {
        status: string;
        message: string;
        data: T;
    };

    type ApiResponse = ApiResponseTemplate<CalculationResult>;

    let result: string;

    let num1 = "";
    let num2 = "";

    const handleSubmit = (event: Event) => {
        event.preventDefault();
        const formData = {
            num1: num1,
            num2: num2,
        };

        fetch("/api/sum", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(formData),
        })
            .then((response) => {
                if (response.ok) {
                    return response.json();
                }
                return response.json().then((errorData: ApiResponse) => {
                    throw errorData;
                });
            })
            .then((body: ApiResponse) => {
                result = `The result is: ${body.data.total}`;
            })
            .catch((error: ApiResponse) => {
                result = `Error: ${error.message}`;
            })
            .finally(() => {
                console.log("finally");
            });
    };
</script>

<svelte:head>
    <title>Calculator</title>
</svelte:head>

<h1>Login Calculator Test Page</h1>
<p>
    This page is to test out logging in with authorization headers and JWT. If
    you are not logged in, then you will see an error returned. If you are
    logged in, the sum of the two numbers will be returned. Of course, if you
    enter anything that isn't a number, you will see an error
</p>

<h2>Adding two numbers</h2>
<form on:submit={handleSubmit}>
    <label for="num1">First Number</label>
    <input id="num1" name="num1" bind:value={num1} type="input" />

    <label for="num2">Second Number</label>
    <input id="num2" name="num2" bind:value={num2} type="input" />

    <button>Submit!</button>
</form>

{#if result}
    <div id="Result">
        <p>{result}</p>
    </div>
{/if}

<style>
    h1 {
        text-align: center;
    }
</style>
