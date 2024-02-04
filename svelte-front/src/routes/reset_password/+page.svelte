<script lang="ts">
    let id: string;
</script>

<svelte:head>
    <title>Reset Password</title>
</svelte:head>

<form id="reset-password">
    <div id="input-fields">
        <label for="id">Username or Email:</label>
        <input id="id" name="id" bind:value={id} />
        <br />
    </div>

    <button id="input-submit">Reset</button>
</form>

<style>
    #reset-password {
        width: 20em;
        margin: 0 auto;
        text-align: center;
    }
</style>
