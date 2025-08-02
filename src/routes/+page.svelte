<script lang="ts" module>
    import Item from "./Item.svelte";
    import "../style.css";
    import Overlay from "./Overlay.svelte";
    import type { Snippet } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import RequestItemQueue from "./RequestItemQueue.svelte";
    import AddItemPopup from "./AddItemPopup.svelte";

    let popUpSnippet: Snippet | undefined = $state(undefined);
    let popUpOnClose: (() => void) | undefined = $state(undefined);
    let isPopUpOpen: boolean = $state(false);

    let direction: boolean = $state(true);
    let camera_url: string | undefined = $state(undefined);

    let detected_object: DetectObjectResult | undefined = $state(undefined);

    export interface DetectObjectResult {
        name: string;
        percentage: string;
    }

    listen<DetectObjectResult>("update-detected-object", (event) => {
        detected_object = event.payload;
    });

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

    //function openPopUpItemQueue() {
    //    openPopup(requestItemQueuePopUp);
    //}
</script>

{#snippet addItemPopup()}
    <AddItemPopup {camera_url} {detected_object} />
{/snippet}

{#snippet requestItemQueuePopUp()}
    <RequestItemQueue />
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
        <!-- <button
            class="button"
            style="width: 15rem;"
            onclick={openPopUpItemQueue}>Request Queue</button
        > -->
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
</style>
