<main>
    <div class="album-background">
        <div class="white-space"></div>
        <div class="song-info">
            <span class="song-name">Nome</span>
        </div>
        <div class="play-pause">
            <button>Play</button>
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
<script>
    function home() {
        window.location.href = "/";
    }
</script>
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
    
    .play-pause button {
        padding: 10px 20px;
        font-size: 16px;
        cursor: pointer;
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
</style>
