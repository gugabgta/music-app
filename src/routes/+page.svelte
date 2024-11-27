<script>
    import { invoke } from "@tauri-apps/api/core";
    
    let name = $state("Gustavo");
    let greetMsg = $state("");
    
    async function greet() {
        greetMsg = await invoke("greet", { name });
    }
</script>

<main class="container">
    <button onclick={greet}>Hello</button>
  <h1>Welcome {greetMsg}</h1>
</main>

<style>

</style>