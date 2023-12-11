<script lang="ts">
    export let data = [0, 0, 0, 0];
    // First number is for the number
    // Second number is for the colour
    // Third number is for the shading
    // Fourth number is for the shape

    //   let data = [2, 0, 1, 2];

    let none = ["none", "none", "none"];
    let colours = ["red", "green", "purple"];
    let dash_shading = [
        "url(#stripes-c0)",
        "url(#stripes-c1)",
        "url(#stripes-c2)",
    ];
    let shading = [none, dash_shading, colours];
    let shape = ["#diamond", "#squiggle", "#oval"];
</script>

<div class="helpers">
    <div class="shapes">
        <svg id="diamond" viewBox="0 0 8 12" stroke-width="0.3">
            <polygon points="4,1 7,6 4,11 1,6 " />
        </svg>

        <svg id="squiggle" viewBox="0 0 8 12" stroke-width="0.3">
            <path
                d="M6.3,2C6.3,1.4,6,0.9,5.5,0.5C4.7,0,3.6,0.2,3,1c-1.3,1.9-1.7,4-0.6,6c0.4,0.7,0.2,1.3-0.3,1.9c-0.6,0.7-0.5,1.8,0.2,2.5c0.7,0.6,1.8,0.5,2.5-0.2C6.3,9.4,6.6,7.3,5.5,5.3C5.1,4.5,5.4,3.7,6,3C6.2,2.7,6.3,2.3,6.3,2"
            />
        </svg>

        <svg id="oval" viewBox="0 0 8 12" stroke-width="0.3">
            <path
                d="M2,3 v6 c0,1.1 0.9,2 2,2 h0 c1.1,0 2,-0.9 2,-2 V3 c0,-1.1 -0.9,-2 -2,-2 h0 C2.9,1 2,1.9 2,3 v0"
            />
        </svg>
    </div>

    <svg width="0" height="0">
        <defs>
            <pattern
                id="stripes-c0"
                width="12"
                height="8"
                stroke="red"
                stroke-width="0.3"
            >
                <line x1="0" y1="1" x2="8" y2="1" />
                <line x1="0" y1="2" x2="8" y2="2" />
                <line x1="0" y1="3" x2="8" y2="3" />
                <line x1="0" y1="4" x2="8" y2="4" />
                <line x1="0" y1="5" x2="8" y2="5" />
                <line x1="0" y1="6" x2="8" y2="6" />
                <line x1="0" y1="7" x2="8" y2="7" />
                <line x1="0" y1="8" x2="8" y2="8" />
                <line x1="0" y1="9" x2="8" y2="9" />
                <line x1="0" y1="10" x2="8" y2="10" />
                <line x1="0" y1="11" x2="8" y2="11" />
            </pattern>
            <pattern
                id="stripes-c1"
                width="12"
                height="8"
                stroke="green"
                stroke-width="0.3"
            >
                <line x1="0" y1="1" x2="8" y2="1" />
                <line x1="0" y1="2" x2="8" y2="2" />
                <line x1="0" y1="3" x2="8" y2="3" />
                <line x1="0" y1="4" x2="8" y2="4" />
                <line x1="0" y1="5" x2="8" y2="5" />
                <line x1="0" y1="6" x2="8" y2="6" />
                <line x1="0" y1="7" x2="8" y2="7" />
                <line x1="0" y1="8" x2="8" y2="8" />
                <line x1="0" y1="9" x2="8" y2="9" />
                <line x1="0" y1="10" x2="8" y2="10" />
                <line x1="0" y1="11" x2="8" y2="11" />
            </pattern>
            <pattern
                id="stripes-c2"
                width="12"
                height="8"
                stroke="purple"
                stroke-width="0.3"
            >
                <line x1="0" y1="1" x2="8" y2="1" />
                <line x1="0" y1="2" x2="8" y2="2" />
                <line x1="0" y1="3" x2="8" y2="3" />
                <line x1="0" y1="4" x2="8" y2="4" />
                <line x1="0" y1="5" x2="8" y2="5" />
                <line x1="0" y1="6" x2="8" y2="6" />
                <line x1="0" y1="7" x2="8" y2="7" />
                <line x1="0" y1="8" x2="8" y2="8" />
                <line x1="0" y1="9" x2="8" y2="9" />
                <line x1="0" y1="10" x2="8" y2="10" />
                <line x1="0" y1="11" x2="8" y2="11" />
            </pattern>
        </defs>
    </svg>
</div>

<div class="generated-card">
    {#each { length: data[0] + 1 } as _}
        <svg
            class="shape"
            stroke={colours[data[1]]}
            fill={shading[data[2]][data[1]]}
        >
            <use xlink:href={shape[data[3]]} />
        </svg>
    {/each}
</div>

<style>
    .helpers {
        height: 0;
        width: 0;
    }

    .shape {
        height: 100%;
        width: 30%;
    }
</style>
