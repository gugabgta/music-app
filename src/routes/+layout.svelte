<svelte:body onclick={ toogleNavVisibility } />
<script>
    let { children, } = $props();
    let nav_visible = $state(true);

    function toogleNavVisibility() {
        nav_visible = !nav_visible;
    }
</script>
{#if nav_visible}
    <nav style="">
        <a href="/">Home</a>
        <a href="/player">Player</a>
        <a href="/settings">Settings</a>
    </nav>
{/if}

<style>
    nav {
        width: 100vw;
        position: fixed;
        display: flex;
        justify-content: center;
        gap: 1rem;
        padding: 1rem;
        background-color: #f0f0f0;
    }
    a {
        text-decoration: none;
        color: #333;
    }
    a:hover {
        text-decoration: underline;
    }
</style>
{@render children()}
