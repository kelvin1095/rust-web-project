<script lang="ts">
    import { goto } from "$app/navigation";
    import { login_status_store } from "../store";

    type UserDetails = {
        username: string;
    };

    type ApiResponseTemplate<T> = {
        status: string;
        message: string;
        data: T;
    };

    type ApiResponse = ApiResponseTemplate<UserDetails>;

    let id = "";
    let password = "";
    let error_message = "";

    const handleSubmit = (event: Event) => {
        event.preventDefault();

        fetch("/api/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                Authorization: `Basic ${btoa(id + ":" + password)}`,
            },
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
                login_status_store.set(true);
                localStorage.setItem("username", body.data.username);
                goto("/");
            })
            .catch((error: ApiResponse) => {
                console.error("Error:", error);
                error_message = `Error: ${error.message}`;
            });
    };
</script>

<svelte:head>
    <title>Login</title>
</svelte:head>

<form id="login-form" on:submit={handleSubmit}>
    <div id="input-fields">
        <label for="id">Username or Email:</label>
        <input id="id" name="id" bind:value={id} />
        <br />

        <label for="password">Password:</label>
        <input id="password" type="password" bind:value={password} />
        <br />
    </div>

    <button id="input-submit">Login</button>
</form>

<div id="login-options">
    <a href="/register">Create Account</a>
    <a href="/reset_password">Forgot Password?</a>
</div>

{error_message}

<style>
    form {
        width: 24em;
        margin: 0 auto;
    }

    label {
        display: block;
    }

    input {
        width: 90%;
    }

    #login-form {
        display: grid;
        grid-template-columns: 2fr 1fr;
        column-gap: 0.5rem;
    }

    #input-submit {
        margin-top: 1.5rem;
    }

    #login-options {
        text-align: center;
        padding-top: 0.5rem;
    }
</style>
