<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    async function toggleGpio23(event: Event) {
        await invoke("toggle_gpio_23");
    }
    async function exit() {
        await invoke("exit");
    }
</script>

<main>
    <h1>Welcome to Tauri + Svelte</h1>
    <button class="button" onclick={toggleGpio23}>Toggle GPIO 23</button>
    <button class="button" onclick={exit}>Exit</button>
</main>

<style>
    :root {
        --fg-color-dark: rgba(255, 255, 255, 0.87);
        --fg-color-2-dark: rgba(200, 200, 200, 1);
        --bg-color-dark: #242424;
        --bg-color-2-dark: #3a3a3a;
        --bg-color-3-dark: #1a1a1a;
        --bg-color-4-dark: #161616;

        font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
        font-size: 16px;
        line-height: 24px;
        font-weight: 400;

        color: var(--fg-color-2-dark);
        background-color: var(--bg-color-3-dark);

        font-synthesis: none;
        text-rendering: optimizeLegibility;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        -webkit-text-size-adjust: 100%;
    }

    .button {
        border-radius: 8px;
        border: 1px solid transparent;
        padding: 0.6em 1.2em;
        font-size: 1em;
        font-weight: 500;
        font-family: inherit;
        color: var(--fg-color-2-dark);
        background-color: var(--bg-color-4-dark);
        border-color: var(--bg-color-2-dark);
        transition: border-color 0.25s;
        box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    }
</style>
