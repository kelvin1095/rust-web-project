<script lang="ts">
    import type { PageData } from "./$types";

    export let data: PageData;
    const language = data.selected;
</script>

<h1>Learn {language}</h1>

<p>
    The goal is for someone using this webapp to be able to speak communicate
    simple questions and understand simple responses, good enough to be able to
    travel.
</p>

<p>
    I think I will need to skip the part about learning how to read the
    different scripts for now. This should definitely be something to come back
    in the future to work on.
</p>

<style>
    h1 {
        text-align: center;
    }
</style>
