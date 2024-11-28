<script>
    // @ts-ignore
    import { invoke } from "@tauri-apps/api/core"
    import { exists, BaseDirectory } from '@tauri-apps/plugin-fs'
    import { appDataDir } from '@tauri-apps/api/path'

    let name = $state("")
    let greetMsg = $state("")
    let default_url = ""
    let song = $state(new Audio("songs/test.mp3"))

    // @ts-ignore
    async function greet(event) {
        event.preventDefault()
        greetMsg = await invoke("greet", { name })
    }

    async function defaultUrl() {
        default_url = await invoke("get_env", { key: "API_URL" })
    }

    async function play() {
        default_url = await invoke("get_env", { key: "API_URL" })
        greetMsg = default_url/*  ?? "No URL" */
        // song.play()
    }

    function pause() {
        song.pause()
    }

    // async function importSong(song) {}
</script>
<main>
    <div class="album-background">
        <div class="white-space"></div>
        <div class="song-info">
            <span class="song-name">{greetMsg}</span>
        </div>
        <div class="play-pause">
            <div role="button" tabindex="0" onclick={play} onkeydown={(e) => e.key === 'Enter' && play()} class="svg-button" aria-label="Play">
                <img src="icons/play-button.svg" alt="Play button" />
            </div>
        </div>
        <div class="song-progress">
            <progress value="42" max="100"> 42% </progress>
        </div>
        <div class="media-buttons">
            <button class="round-button fine-line">Random</button>
            <button class="round-button fine-line">Previous</button>
            <button class="round-button fine-line">Next</button>
            <button class="round-button fine-line">Repeat</button>
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
        display: flex;
        justify-content: space-around;
        margin: 20px 0;
    }

    .media-buttons button {
        padding: 10px 20px;
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

    .round-button {
        width: 50px;
        height: 50px;
        border-radius: 50%;
        padding: 10px;
    }

    .fine-line {
        border: 1px solid #000;
        box-shadow: none;
        background-color: transparent;
    }

    .svg-button {
        width: 50px;
        height: 50px;
    }

    .svg-button:hover {
        cursor: pointer;
    }

    .svg-button img {
        filter: invert(58%) sepia(37%) saturate(5463%) hue-rotate(1deg) brightness(104%) contrast(104%);
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
