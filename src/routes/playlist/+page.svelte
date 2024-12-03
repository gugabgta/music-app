<script>// @ts-nocheck
    import { invoke } from "@tauri-apps/api/core";
    import { fetch } from '@tauri-apps/plugin-http';
    // let { data } = $props();

    let data = $state({})
    function playSong(song_id) {
        // const song_id = e.target.id;
        window.location.href = `/player?id=${song_id}`;
    }

    function get_playlist() {
        const res = fetch("http://127.0.0.1:3000/playlist", {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
                "Access-Control-Allow-Origin": "*",
            },  
        })
            .then(res => {
                // console.log(res)
                return res.json()
            })
            .then(stf => {
                data = stf
                // console.log(data)
            })
            .catch(err => console.error(err));
    }

    get_playlist();
</script>
<h1>Playlist</h1>
    <table>
        <tbody>
            {#each data as song}
                <tr id="{song.id}" onclick={playSong(song.id)}>
                    <td>{song.id}</td>
                    <td>{song.title}</td>
                </tr>
            {/each}
        </tbody>
    </table>
<style>
    tr {
        display: flex;
        justify-content: left;
        gap: 1rem;
        border: 1px solid #f0f0f0;
    }

    tr:hover {
        background-color: #f0f0f0;
        cursor: pointer;
    }

    td {
        padding: 1rem;
    }

    table {
        width: 100%;
    }

    h1 {
        text-align: center;
    }
</style>