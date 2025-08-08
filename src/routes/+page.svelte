<script lang="ts" module>
    import Item from "./Item.svelte";
    import "../style.css";
    import Overlay from "./Overlay.svelte";
    import { onMount, type Snippet } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import AddItemPopup from "./AddItemPopup.svelte";
    import { type DisplayItem } from "../bindings/DisplayItem";

    let popUpSnippet: Snippet | undefined = $state(undefined);
    let popUpOnClose: (() => void) | undefined = $state(undefined);
    let isPopUpOpen: boolean = $state(false);
    let isCloseable: boolean = $state(true);

    let camera_url: string | undefined = $state(undefined);

    let detected_object: DetectObjectResult | undefined = $state(undefined);

    export interface DetectObjectResult {
        name: string;
        percentage: string;
    }

    listen<DetectObjectResult>("update-detected-object", (event) => {
        detected_object = event.payload;
    });

    export function closePopUp() {
        isCloseable = true;

        popUpSnippet = undefined;
        popUpOnClose?.();
        isPopUpOpen = false;
    }

    export function openPopup(
        snippet: Snippet,
        onClose?: () => void,
        closeable: boolean = true,
    ) {
        popUpSnippet = snippet;
        popUpOnClose = onClose;
        isPopUpOpen = true;
        isCloseable = closeable;
    }

    async function exit() {
        await invoke("exit");
    }

    async function addItem() {
        isCloseable = false;
        camera_url = await invoke<string>("serve_rpi_cam");
        openPopup(addItemPopup, async () => {
            await invoke("stop_rpi_cam");
            camera_url = undefined;
            isCloseable = true;
        });
    }

    function onPopUpClose() {
        popUpOnClose?.();
        popUpOnClose = undefined;
    }
</script>

<script lang="ts">
    let displayItems: DisplayItem[] = $state([]);

    onMount(() => {
        async function fetch_items() {
            displayItems = await invoke<DisplayItem[]>("list_items");
        }

        fetch_items();

        const interval = setInterval(fetch_items, 3000);

        return () => clearInterval(interval);
    });
</script>

{#snippet addItemPopup()}
    <AddItemPopup {camera_url} {detected_object} />
{/snippet}

<main class="container no-select">
    <Overlay bind:open={isPopUpOpen} onClose={onPopUpClose} bind:isCloseable>
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

    <div class="items-container" style="padding: 1rem;">
        {#each displayItems as item}
            <Item
                image_source={item.image_path}
                item_name={item.display_name}
                item_amount={Number(item.amount)}
                item_id={Number(item.id)}
            />
        {/each}
    </div>
</main>

<style>
    .items-container {
        display: flex;
        flex-wrap: wrap;
        gap: 1rem;

        scroll-behavior: smooth;
        overflow-y: auto;
    }
</style>
