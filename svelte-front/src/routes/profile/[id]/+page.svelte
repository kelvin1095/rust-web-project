<script lang="ts">
    import { goto } from "$app/navigation";
    import type { PageData } from "./$types";
    import { login_status_store } from "../../store";

    export let data: PageData;
    const username = data.username;

    const logout = async () => {
        try {
            const response = await fetch("/api/logout", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
            });

            if (!response.ok) {
                throw new Error(
                    `HTTP ${
                        response.status
                    } error! Status: ${await response.text()}`
                );
            }
        } catch (error) {
            console.error("Error:", error);
        }

        localStorage.removeItem("username");
        login_status_store.set(false);
        goto("/");
    };
</script>

<h1>Welcome {username}!</h1>

<p>
    The plan for this page is to show an overview of each language learning
    progress. In addition, there will be some kind of settings page for each
    user as well.
</p>
<p>
    I want to make these two pages directly accessible via a 'hover over' effect
    on the top right
</p>

<div id="logout">
    <button on:click={logout}> Log Out </button>
</div>

<style>
    h1 {
        text-align: center;
    }

    #logout {
        text-align: center;
    }
</style>
