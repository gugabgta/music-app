// @ts-nocheck
import { invoke } from "@tauri-apps/api/core";
import { fetch } from '@tauri-apps/plugin-http';

let default_url = ""

export const load = async ({ params }) => {
    return {
        song: await importSong(),
    };
};

async function importSong() {
    await defaultUrl()
    console.log(window.location)
    const song_id = new URL(window.location).searchParams.get("id")
    console.log(song_id)
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

    return window.URL.createObjectURL(blob);
}

async function defaultUrl() {
    default_url = await invoke("get_env", { key: "API_URL" })
}
