<script lang="ts">
  let languageList = ["Mandarin", "Japanese", "Korean"];
</script>

<svelte:head>
  <title>Language</title>
</svelte:head>

<h1>Welcome to my new test language page!</h1>
<p>
  I want to put a game here thats basically a mix and match using cards similar
  to my set game
</p>

<p>
  The end goal of this is for someone who is using this webapp to be able to ask
  for directions and understand the directions.
</p>

<ul>
  <li>greetings/introduction/farewells</li>
  <li>numbers</li>
  <li>time and duration</li>
  <li>adjectives</li>
  <ul><li>front, back, left, right</li></ul>
  <ul><li>early, late, fast, slow</li></ul>
  <ul><li>big, small, wide, narrow</li></ul>
  <li>colours</li>
  <li>places</li>
  <ul><li>restaurant, hotel, station</li></ul>
  <ul><li>post office, police station, park</li></ul>
  <li>food</li>
</ul>

{#each languageList as language, i}
  <a href="/language/learn-{language}">{language}</a>
{/each}

<style>
  h1 {
    text-align: center;
  }

  ul {
    list-style-type: disc;
    padding: 0 2rem;
  }
</style>
