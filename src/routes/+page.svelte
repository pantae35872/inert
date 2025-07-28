<script lang="ts" module>
    import Item from "./Item.svelte";
    import "../style.css";
    import Overlay from "./Overlay.svelte";
    import type { Snippet } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { scale } from "svelte/transition";
    import Numpad from "./Numpad.svelte";

    let popUpSnippet: Snippet | undefined = $state(undefined);
    let popUpOnClose: (() => void) | undefined = $state(undefined);
    let isPopUpOpen: boolean = $state(false);

    let direction: boolean = $state(true);
    let camera_url: string | undefined = $state(undefined);

    let amount: string = $state("");
    let numpadOn: boolean = $state(false);

    async function test_motor() {
        await invoke("test_motor", { direction });
    }

    export function openPopup(snippet: Snippet, onClose?: () => void) {
        popUpSnippet = snippet;
        popUpOnClose = onClose;
        isPopUpOpen = true;
    }

    async function exit() {
        await invoke("exit");
    }

    async function addItem() {
        camera_url = await invoke<string>("serve_rpi_cam");
        openPopup(addItemPopup, async () => {
            await invoke("stop_rpi_cam");
            camera_url = undefined;
        });
    }

    function onPopUpClose() {
        popUpOnClose?.();
        popUpOnClose = undefined;
    }
</script>

{#snippet addItemPopup()}
    <div
        class="add-item-wrapper"
        transition:scale={{
            duration: 200,
        }}
    >
        <div class="add-item">
            <form class="add-item-form">
                <h2>Add Item</h2>
                {#if camera_url}
                    <div class="image-wrapper">
                        <img src={camera_url} alt="Camera Stream" />
                    </div>
                {/if}

                <input
                    class="item-amount-input"
                    placeholder="Item name"
                    type="text"
                    required
                    readonly
                />
                <input
                    class="item-amount-input"
                    placeholder="Amount"
                    type="text"
                    onclick={() => (numpadOn = !numpadOn)}
                    value={amount}
                    required
                    readonly
                />
                <button class="button" type="submit">Add</button>
            </form>
        </div>

        <Numpad bind:amount {numpadOn} />
    </div>
{/snippet}

<main class="container no-select">
    <Overlay bind:open={isPopUpOpen} onClose={onPopUpClose}>
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

    .add-item-wrapper {
        display: flex;
        padding: 10rem;
        width: 100%;
        height: 100%;
        justify-content: center;
        align-items: center;
        pointer-events: none;
    }

    .add-item-wrapper > * {
        pointer-events: auto;
    }

    .add-item {
        max-height: 80%;
        max-width: 50%;

        background-color: var(--bg-color-3);
        padding: 10px;
        border: 1px solid var(--border-color);
        border-radius: 0.5rem;
        box-shadow: 0 2px 2px var(--bg-color-2);

        display: flex;
        flex-direction: column;
        overflow: auto;
    }

    .add-item-form {
        display: flex;
        flex-direction: column;
        overflow: auto;
        gap: 0.8rem;

        max-height: 100%;
        height: 100%;
    }

    .item-amount-input {
        outline: none;
        border-radius: 0.31rem;
        border: 2px solid var(--border-color);
        background-color: var(--bg-color);
        color: var(--fg-color-2);
        padding: 0 1.25rem 0 3.12rem;
        font-size: 1.06rem;
        transition: 0.2s ease;

        padding: 0.5rem;
        margin: 0.1rem;
    }

    .item-amount-input::placeholder {
        color: var(--fg-color-2);
    }

    .image-wrapper {
        flex: 1 1 auto;
        overflow: hidden;
        display: flex;
        align-items: center;
        justify-content: center;

        margin: auto;
        padding: auto;
    }

    .image-wrapper img {
        max-width: 100%;
        max-height: 100%;
        object-fit: contain;

        border-radius: 0.31rem;
    }
</style>
