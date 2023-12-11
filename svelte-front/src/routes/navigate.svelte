<script lang="ts">
    const linkList = [
        {
            name: "Pokemon",
            link: "/pokemon",
        },
        {
            name: "Koi Fish",
            link: "/koi",
        },
        {
            name: "Set Game",
            link: "/set",
        },
        {
            name: "Login Test Calc",
            link: "/calc",
        },
        {
            name: "Learn a Language",
            link: "/language",
        },
        {
            name: "About",
            link: "/about",
        },
    ];
</script>

<nav id="navBox">
    <ul>
        {#each linkList as item, i}
            <li>
                <a href={item.link}>{item.name}</a>
            </li>
        {/each}
    </ul>
</nav>

<style>
</style>
