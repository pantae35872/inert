<script lang="ts" module>
    import Item from "./Item.svelte";
    import "../style.css";
    import Overlay from "./Overlay.svelte";
    import type { Snippet } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { scale } from "svelte/transition";

    let popUpSnippet: Snippet | undefined = $state(undefined);
    let isPopUpOpen: boolean = $state(false);

    let direction: boolean = $state(true);
    let image_url: string | undefined = $state(undefined);

    async function test_motor() {
        await invoke("test_motor", { direction });
    }

    async function test_camera() {
        const image_64: string = await invoke("test_camera");

        const image_raw = Uint8Array.from(atob(image_64), (c) =>
            c.charCodeAt(0),
        );

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

    function addItem() {
        openPopup(addItemPopup);
    }
</script>

{#snippet addItemPopup()}
    <div
        class="add-item"
        transition:scale={{
            duration: 200,
        }}
    >
        <form class="add-item-form">
            <h2>Add Item</h2>
            <button class="button" type="submit">Add</button>
        </form>
    </div>
{/snippet}

<main class="container no-select">
    <Overlay bind:open={isPopUpOpen}>
        {@render popUpSnippet?.()}
    </Overlay>
    <div
        style="display: flex; align-items: center; text-align: center; justify-content: space-between; padding: 1rem;"
    >
        <button class="button" style="width: 10rem;" onclick={addItem}
            >Add Item</button
        >
        <h1 style="text-align: center;">Inventory</h1>
        <button class="button" style="width: 10rem;" onclick={exit}>Exit</button
        >
    </div>

    <!-- <div class="motor-test">
        <button class="button" onclick={test_motor}>Test Motor</button>
        <input type="checkbox" bind:checked={direction} />
    </div>
    <div class="camera-test">
        <button class="button" onclick={test_camera}>Test Camera</button>
        {#if image_url}
            <img src={image_url} alt="Esp32 Camera" />
        {/if}
    </div> -->
    <div class="items-container" style="padding: 1rem;">
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
    .items-container {
        display: flex;
        flex-wrap: wrap;
        gap: 1rem;

        justify-content: space-between;

        scroll-behavior: smooth;
        overflow-y: auto;
    }

    .add-item {
        background-color: var(--bg-color-3);
        padding: 10px;
        border: 1px solid var(--border-color);
        border-radius: 0.5rem;
        box-shadow: 0 2px 2px var(--bg-color-2);
    }

    .add-item-form {
        display: flex;
        flex-direction: column;
        gap: 0.8rem;

        max-height: 100%; /* Don't exceed popup height */
    }
</style>
