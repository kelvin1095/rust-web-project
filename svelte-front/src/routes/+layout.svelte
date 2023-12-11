<script lang="ts">
    import { fade } from "svelte/transition";

    import Header from "./header.svelte";
    import Footer from "./footer.svelte";
    import Navigate from "./navigate.svelte";

    // export const prerender = true;
    // export const ssr = false;

    export let data;
</script>

<div id="root">
    <div id="header">
        <Header />
    </div>

    <div id="mainContent">
        <div id="leftColumn">
            <Navigate />
        </div>

        <div id="mainColumn">
            {#key data.url}
                <div
                    id="transition"
                    out:fade={{ delay: 0, duration: 100 }}
                    in:fade={{ delay: 100, duration: 100 }}
                >
                    <slot />
                </div>
            {/key}
        </div>

        <div id="rightColumn">
            <p>
                My current plan for this column is to use as a score or
                something for the learn language aspect.
            </p>
        </div>
    </div>

    <div id="footer">
        <Footer />
    </div>
</div>

<style>
    @font-face {
        font-family: Ysabeau;
        font-style: normal;
        font-weight: 400;
        src: url("/font/Ysabeau-Regular.ttf");
    }

    @font-face {
        font-family: Cormorant;
        font-style: normal;
        font-weight: 400;
        src: url("/font/Cormorant-Regular.ttf");
    }

    :global(body) {
        font-family: "Ysabeau";
        font-weight: normal;
        padding: 0;
        margin: 0;
    }

    :global(h1) {
        font-family: "Cormorant";
        font-weight: normal;
        padding: 0;
        margin: 0;
    }

    :global(h2) {
        font-family: "Cormorant";
        font-weight: normal;
        padding: 0;
        margin: 0;
    }

    :global(ul) {
        list-style: none;
        padding: 0;
    }

    :global(a) {
        text-decoration: none;
        color: inherit;
    }

    #root {
        display: flex;
        flex-direction: column;
        min-height: 100vh;
    }

    #mainContent {
        flex-grow: 1;
        width: 1024px;
        display: grid;
        grid-template-columns: 1fr 5fr 1fr;
        margin: 0.5rem auto;
        column-gap: 0.5rem;
    }

    #leftColumn {
        grid-column: 1;
    }

    #mainColumn {
        grid-column: 2;
    }

    #rightColumn {
        grid-column: 3;
    }
</style>
