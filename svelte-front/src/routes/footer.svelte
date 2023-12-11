<script lang="ts">
    let contactInfo = [
        {
            id: 1,
            name: "GitHub",
            link: "https://github.com/kelvin1095",
        },
        {
            id: 2,
            name: "LinkedIn",
            link: "https://www.linkedin.com/in/kelvin-w-14679ab4",
        },
        {
            id: 3,
            name: "Instagram",
            link: "https://www.instagram.com/negativeprogress",
        },
        {
            id: 4,
            name: "Threads",
            link: "https://www.threads.net/@negativeprogress",
        },
    ];
</script>

<footer>
    <div id="Links">
        <h1>Keep in Touch</h1>
        <div id="ContactInfo">
            {#each contactInfo as contact (contact.id)}
                {#if contact.id !== 1}
                    <span>&nbsp;|</span>
                {/if}
                <a href={contact.link}>{contact.name}</a>
            {/each}
        </div>
    </div>
</footer>

<style>
    footer {
        background-color: rgb(55 65 81);
        color: rgb(243, 244, 246);
        padding: 2.5rem;
    }

    h1 {
        text-align: center;
        margin-bottom: 2.5rem;
    }

    #ContactInfo {
        text-align: center;
    }
</style>
