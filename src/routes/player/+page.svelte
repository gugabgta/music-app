<script>
// @ts-nocheck

    import { invoke } from "@tauri-apps/api/core"
    import { fetch } from '@tauri-apps/plugin-http';

    let name = $state("idk yet")
    let default_url = ""
    let song = $state(new Audio("songs/test.mp3"))
    let song_id = $state(100)
    let song_state = $state("paused")
    let progress = $state(0)
    let blobURL = ""
    let slider

    async function importSong() {
        await defaultUrl()
        const idk = fetch(`${default_url}/song?id=${song_id}`, {
            method: 'GET',
            headers: {
                'Content-Type': 'audio/mpeg',
                'Access-Control-Allow-Origin': '*',
            }
        })

        const response = await idk;

        if (!response.ok) {
            console.error(await response.json())
            return false
        }

        const blob = await response.blob();

        blobURL = window.URL.createObjectURL(blob);
        pause()
        song = new Audio(blobURL);
        song.addEventListener("ended", () => pause())
        song.addEventListener("timeupdate", () => {
            progress = Math.ceil(song.currentTime / song.duration * 1000)
            slider.style.background = `linear-gradient(to right, green ${slider.value/10}%, #ccc ${slider.value/10}%)`;
        })
        return true
    }

    async function defaultUrl() {
        default_url = await invoke("get_env", { key: "API_URL" })
    }

    async function play() {
        song_state = "playing"
        song.play();
    }

    async function previous() {
        song.currentTime = 0
    }

    async function next() {
        song.currentTime = song.duration
    }

    async function repeat() {
        song.currentTime = 0
    }

    async function random() {
        song.currentTime = Math.random() * song.duration
    }

    function pause() {
        song_state = "paused"
        song.pause()
    }

    function moveProgress(event) {
        event.target.style.background = `linear-gradient(to right, green ${event.target.value/10}%, #ccc ${event.target.value/10}%)`;
        song.currentTime = event.target.value / 1000 * song.duration
    }

</script>
<main>
    <div class="album-background">
        <div class="white-space"></div>
        <div class="song-info">
            <span class="song-name">{name}</span>
            <input type="number" bind:value={song_id}>
            <button onclick={importSong}>Load</button>
        </div>
        <div class="play-pause">
            {#if song_state === "paused"}
                <div role="button" tabindex="0" onclick={play} onkeydown={(e) => e.key === 'Enter' && play()} class="svg-button" aria-label="Play">
                    <img src=icons/play.svg alt="Play button" />
                </div>
            {:else}
                <div role="button" tabindex="0" onclick={pause} onkeydown={(e) => e.key === 'Enter' && pause()} class="svg-button" aria-label="Pause">
                    <img src=icons/pause.svg alt="Pause button" />
                </div>
            {/if}
        </div>
        <div class="song-progress">
            <input id="slider" type="range" class="range-input" min="0" max="1000" value={progress} oninput={moveProgress} bind:this={slider}/>
        </div>
        <div class="media-buttons row">
            <div role="button" tabindex="0" onclick={random} onkeydown={(e) => e.key === 'Enter' && random()} class="svg-button" aria-label="Random">
                <img src="icons/random.svg" alt="Random" />
            </div>
            <div class="button-group">
                <div role="button" tabindex="0" onclick={previous} onkeydown={(e) => e.key === 'Enter' && previous()} class="svg-button" aria-label="Previous">
                    <img src="icons/previous.svg" alt="Previous" />
                </div>
                <div role="button" tabindex="0" onclick={next} onkeydown={(e) => e.key === 'Enter' && next()} class="svg-button" aria-label="Next">
                    <img src="icons/next.svg" alt="Next" />
                </div>
            </div>
            <div role="button" tabindex="0" onclick={repeat} onkeydown={(e) => e.key === 'Enter' && repeat()} class="svg-button" aria-label="Repeat">
                <img src="icons/repeat.svg" alt="Repeat" />
            </div>
        </div>
        <div class="lyrics">
            <p>Lyrics</p>
        </div>
    </div>
</main>
<style>
    .album-background {
        height: 100vh;
        width: 100vw;
        background-image: url("rem.jpg");
        background-color:rgba(0, 0, 0, 0.2);
    }

    input[type="range"] {
        appearance: none; 
        -webkit-appearance: none;
        width: 80%;
        cursor: pointer;
        outline: none;
        border-radius: 20px;
        height: 0.5rem;
        background: #ccc;
    }

    input[type="range"]::-webkit-slider-thumb {
        -webkit-appearance: none;
        appearance: none; 
        height: 1.3rem;
        width: 1.3rem;
        background-color: #f50;
        border-radius: 50%;
        transition: .2s ease-in-out;
    }

    input[type="range"]::-webkit-slider-thumb:hover {
        box-shadow: 0 0 0 10px rgba(255, 85, 0, .1)
    }

    input[type="range"]:active::-webkit-slider-thumb {
        box-shadow: 0 0 0 13px rgba(255, 85, 0, .2)
    }

    input[type="range"]:focus::-webkit-slider-thumb {
        box-shadow: 0 0 0 13px rgba(255, 85, 0, .2)
    }

    .white-space {
        height: 40vh;
    }

    .song-info {
        display: flex;
        justify-content: center;
        align-items: center;
        margin: 20px 0;
    }

    .song-info span {
        font-size: 24px;
    }

    .play-pause {
        display: flex;
        justify-content: center;
        align-items: center;
        margin: 20px 0;
    }

    .button-group {
        display: flex;
        align-items: center;
    }

    .song-progress {
        display: flex;
        justify-content: center;
        align-items: center;
        margin: 20px 0;
    }

    .song-progress progress {
        width: 80%;
        height: 20px;
    }

    .media-buttons {
        width: 100%;
        display: flex;
        justify-content: space-around;
        margin: 20px 0;
    }

    .media-buttons .svg-button {
        width: 30px;
        height: 30px;
        padding: 10px 10px;
        font-size: 16px;
        cursor: pointer;
    }

    .lyrics {
        display: flex;
        justify-content: center;
        align-items: center;
        margin: 20px 0;
    }

    .lyrics p {
        font-size: 18px;
        text-align: center;
    }

    .svg-button {
        width: 50px;
        height: 50px;
    }

    .svg-button:hover {
        cursor: pointer;
    }

    .svg-button img {
        filter: invert(0%) sepia(97%) saturate(21%) hue-rotate(64deg) brightness(95%) contrast(100%);
        width: 100%;
        height: 100%;
    }

    .svg-button:click {
        filter: invert(58%) sepia(37%) saturate(5463%) hue-rotate(1deg) brightness(104%) contrast(104%);
    }

    .svg-button img:click {
        filter: invert(58%) sepia(37%) saturate(5463%) hue-rotate(1deg) brightness(104%) contrast(104%);
    }
</style>
