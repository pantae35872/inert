<script lang="ts" module>
    import Item from "./Item.svelte";
    import "../style.css";
    import Overlay from "./Overlay.svelte";
    import { onMount, type Snippet } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import AddItemPopup from "./AddItemPopup.svelte";
    import { type DisplayItem } from "../bindings/DisplayItem";
    import Keyboard from "./Keyboard.svelte";
    import { scale } from "svelte/transition";

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
    let allDisplayItems: DisplayItem[] = $state([]);
    let search_keys: string = $state("");
    let displayItems = $derived(
        search_keys.length == 0
            ? allDisplayItems
            : allDisplayItems.filter((item) => {
                  let res = item.display_name
                      .toLowerCase()
                      .includes(search_keys.toLowerCase());
                  return res;
              }),
    );

    onMount(() => {
        async function fetch_items() {
            allDisplayItems = await invoke<DisplayItem[]>("list_items");
        }

        fetch_items();

        const interval = setInterval(fetch_items, 3000);

        return () => clearInterval(interval);
    });
</script>

{#snippet addItemPopup()}
    <AddItemPopup {camera_url} {detected_object} />
{/snippet}

{#snippet keyboard()}
    <div
        transition:scale={{
            duration: 200,
        }}
        style="display: flex; flex-direction: column; gap: 0.2rem; align-items: center; justify-content: center;"
    >
        <input
            class="search-input"
            style="width: 90%; margin-left: 1.4rem;"
            type="text"
            value={search_keys}
            readonly
        />
        <Keyboard bind:keys={search_keys} keyboardOn={true} closeBtn={false} />
    </div>
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
        <input
            class="search-input"
            type="text"
            onclick={() => openPopup(keyboard)}
            placeholder="Search inventory: "
            value={search_keys}
            readonly
        />
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

    .search-input {
        outline: none;
        border-radius: 0.31rem;
        border: 2px solid var(--border-color);
        background-color: var(--bg-color);
        color: var(--fg-color-2);
        font-size: 0.8rem;
        transition: 0.2s ease;

        padding: 0.8rem;
        margin: 0.3rem;

        width: 80%;
    }

    .search-input::placeholder {
        color: var(--fg-color-2);
    }
</style>
