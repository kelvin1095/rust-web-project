<script lang="ts">
    let languageList = ["Mandarin", "Japanese", "Korean"];

    //   https://codepen.io/josephwong2004/pen/ExgoKde

    const greetings = [
        "Hello",
        // "Hola",
        // "Bonjour",
        // "Hallo",
        // "Ciao",
        "こんにちは",
        "안녕하세요",
        "你好",
        // "Привет",
        // "مرحبا",
    ];

    let greeting_display: string = "";
    let status: string = "idle";

    let i = 0;
    let j = 0;
    let greeting = greetings[0];
    let speed = 50;

    function delay(ms: number) {
        return new Promise((resolve) => setTimeout(resolve, ms));
    }

    async function typeGreeting() {
        await delay(1000);
        status = "typing";
        while (i < greeting.length) {
            await delay(speed);
            greeting_display += greeting.charAt(i);
            i++;
        }
        status = "idle";
        return;
    }

    async function deleteGreeting() {
        await delay(2000);
        status = "typing";
        while (i > 0) {
            await delay(speed);
            greeting_display = greeting_display.substring(
                0,
                greeting_display.length - 1
            );
            i--;
        }
        status = "idle";
        return;
    }

    async function cycleGreeting() {
        while (true) {
            while (j < greetings.length) {
                greeting = greetings[j];
                await typeGreeting();
                await deleteGreeting();
                j++;
            }
            j = 0;
        }
    }

    cycleGreeting();
</script>

<svelte:head>
    <title>Language</title>
</svelte:head>

<h1>Learn a Language!</h1>
<p>
    I want to put a game here thats basically a mix and match using cards
    similar to my set game
</p>

<p>
    The end goal of this is for someone who is using this webapp to be able to
    ask for directions and understand the directions.
</p>

<p>
    For getting the site up and running, I think I will need to skip how to read
    katakana, hiragana and hangul
</p>

<div id="greeting-type">
    <span id="greetings">{greeting_display}</span>
    <span id="caret">&nbsp;</span>
</div>

<p>
    The current idea is to request a learning session where the backend
    generates 10 questions along with some session id. Answering each question
    will send a post request and a message saying correct or incorrect will be
    returned. After answering the last question, this will 'close' off the
    learning session.
</p>

<p>
    The exact method of how to track which was correct/incorrect still needs to
    be figured out later. Will need to track on an individual level so maybe
    will need a table per user.
</p>

<ul>
    <li>greetings/introduction/farewells</li>
    <li>family</li>
    <ul>
        <li>mother, father, brother, sister</li>
        <li>grandmother, grandfather, aunt, uncle</li>
        <li>son, daughter, grandson, granddaughter</li>
    </ul>
    <li>numbers</li>
    <ul>
        <li>time</li>
        <li>days of the week, months</li>
        <li>duration</li>
        <li>money</li>
        <li>counting</li>
    </ul>
    <li>adjectives</li>
    <ul>
        <li>front, back, left, right</li>
        <li>early, late, fast, slow</li>
        <li>big, small, wide, narrow</li>
        <li>heavy, light, tall, short</li>
        <li>hot, cold, warm, cool</li>
        <li>colours</li>
    </ul>
    <li>weather</li>
    <li>places</li>
    <ul>
        <li>countries and languages</li>
        <li>different types of rooms like bedroom, kitchen, toilet</li>
        <li>public places like post office, police station, park</li>
    </ul>
    <li>food</li>
</ul>

{#each languageList as language, i}
    <a href="/language/learn-{language}">{language}</a>
{/each}

<style>
    @font-face {
        font-family: "Noto Sans SC";
        font-style: normal;
        font-weight: 400;
        src: url("/font/NotoSansSC-Regular.ttf");
    }
    /* @import url("https://fonts.googleapis.com/css2?family=Noto+Sans+JP&family=Noto+Sans+KR&family=Noto+Sans+SC&display=swap"); */

    h1 {
        text-align: center;
    }

    ul {
        list-style-type: disc;
        padding: 0 2rem;
    }

    #greeting-type {
        font-family: "Noto Sans SC";
        font-size: xx-large;
        height: 5rem;
        border-style: solid;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    #caret {
        display: inline-block;
        width: 1px;
        height: 2.5rem;
        background-color: black;
        animation: blink 1s infinite;
    }

    @keyframes blink {
        0% {
            opacity: 1;
        }
        40% {
            opacity: 1;
        }
        60% {
            opacity: 0;
        }
        100% {
            opacity: 0;
        }
    }
</style>
