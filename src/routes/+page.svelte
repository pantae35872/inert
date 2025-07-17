<script lang="ts" module>
    import Item from "./Item.svelte";
    import "../style.css";
    import Overlay from "./Overlay.svelte";
    import type { Snippet } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    let popUpSnippet: Snippet | undefined = $state(undefined);
    let isPopUpOpen: boolean = $state(false);

    let steps: number = $state(0);
    let direction: boolean = $state(true);
    let image_url: string | undefined = $state(undefined);

    async function test_motor() {
        await invoke("test_motor", { steps, direction });
    }

    async function test_camera() {
        const image_raw: Uint8Array = await invoke("test_camera");

        const blob = new Blob([image_raw], { type: "image/jpeg" });
        image_url = URL.createObjectURL(blob);
    }

    export function openPopup(snippet: Snippet) {
        popUpSnippet = snippet;
        isPopUpOpen = true;
    }

    async function exit() {
        await invoke("exit");
    }
</script>

<main class="container no-select">
    <Overlay bind:open={isPopUpOpen}>
        {@render popUpSnippet?.()}
    </Overlay>
    <div
        style="display: flex; align-items: center; text-align: center; justify-content: space-between; width: 100%;"
    >
        <div></div>
        <h1 style="text-align: center;">Inventory</h1>
        <button class="button" onclick={exit}>Exit</button>
    </div>

    <div class="motor-test">
        <button class="button" onclick={test_motor}>Test Motor</button>
        <input type="number" bind:value={steps} />
        <input type="checkbox" bind:checked={direction} />
    </div>
    <div class="camera-test">
        <button class="button" onclick={test_camera}>Test Camera</button>
        {#if image_url}
            <img src={image_url} alt="Esp32 Camera" />
        {/if}
    </div>
    <div class="items-container">
        {#each Array(100) as _}
            <Item
                image_source="https://t3.ftcdn.net/jpg/00/77/77/16/360_F_77771611_BCUZR6NW73NVdiLgmOeIzzSh4RP2U3aV.jpg"
                item_name="Resistor"
                item_amount={20}
            />
        {/each}
    </div>
</main>

<style>
    .container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 1em;
    }

    .items-container {
        display: flex;
        flex-wrap: wrap;
        gap: 1rem;

        justify-content: center;
        width: 100%;
        max-width: 100%;

        scroll-behavior: smooth;
        overflow-y: auto;

        -webkit-overflow-scrolling: touch;
    }
</style>
