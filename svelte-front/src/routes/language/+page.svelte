<script lang="ts">
    let languageList = ["mandarin", "japanese", "korean"];

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
        return;
    }

    async function deleteGreeting() {
        await delay(2000);
        while (i > 0) {
            await delay(speed);
            greeting_display = greeting_display.substring(
                0,
                greeting_display.length - 1
            );
            i--;
        }
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
    The current idea is to request a learning session where the backend
    generates 10 questions along with some session id. Answering each question
    will send a post request and receive a message saying correct or incorrect
    appended to some table. After answering the last question, this will 'close'
    off the learning session.
</p>

<div id="greeting-type">
    <span id="greetings">{greeting_display}</span>
    <span id="caret">&nbsp;</span>
</div>

<p>
    The exact method of how to track which was correct/incorrect still needs to
    be figured out later. Will need to track on an individual level so maybe
    will need a table per user. I'm currently thinking all users would be
    tracked via the same table. Can think of this table as a 'recent' progress
    which another process can take this information and save it down more
    efficiently
</p>

<div id="avaliable-language">
    {#each languageList as language}
        <a class="language-options" href="/language/learn-{language}">
            {language.charAt(0).toUpperCase() + language.slice(1)}
        </a>
    {/each}
</div>

<p>Topics to cover are:</p>

<ul>
    <li>greetings/introduction/farewells</li>
    <li>family</li>
    <li>numbers</li>
    <li>time</li>
    <li>duration</li>
    <li>counting</li>
    <li>position</li>
    <li>speed</li>
    <li>space</li>
    <li>brightness</li>
    <li>temperature</li>
    <li>colours</li>
    <li>weather</li>
    <li>places</li>
    <ul>
        <li>countries and languages</li>
        <li>different types of rooms like bedroom, kitchen, toilet</li>
        <li>public places like post office, police station, park</li>
    </ul>
    <li>food</li>
</ul>

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

    #avaliable-language {
        padding: 1rem;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        gap: 2rem;
    }

    .language-options {
        flex: 1;
        font-size: larger;
        text-align: center;
    }
</style>
