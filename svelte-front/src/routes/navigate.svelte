<script lang="ts">
  const linkList = [
    {
      id: 1,
      name: "Home",
      link: "/",
    },
    {
      id: 2,
      name: "Pokemon",
      link: "/pokemon",
    },
    {
      id: 3,
      name: "Koi Fish",
      link: "/koi",
    },
    {
      id: 4,
      name: "Set Game",
      link: "/set",
    },
    {
      id: 5,
      name: "Calculator",
      link: "/calc",
    },
    {
      id: 6,
      name: "Learn a Language",
      link: "/language",
    },
    {
      id: 7,
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
