<script lang="ts" module>
    import Item from "./Item.svelte";
    import "../style.css";
    import Overlay from "./Overlay.svelte";
    import { onMount, type Snippet } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import RequestItemQueue from "./RequestItemQueue.svelte";
    import AddItemPopup from "./AddItemPopup.svelte";
    import { type Direction } from "../bindings/Direction";
    import { type DisplayItem } from "../bindings/DisplayItem";
    import type { PrepareAddItemStatus } from "../bindings/PrepareAddItemStatus";
    import type { Rectangle } from "../bindings/Rectangle";
    import { FetchableDevEnvironment } from "vite";

    let popUpSnippet: Snippet | undefined = $state(undefined);
    let popUpOnClose: (() => void) | undefined = $state(undefined);
    let isPopUpOpen: boolean = $state(false);

    let camera_url: string | undefined = $state(undefined);

    let detected_object: DetectObjectResult | undefined = $state(undefined);

    let magnet_state: boolean = false;

    let direction: Direction = $state("North");

    let move_to_x: number = $state(0);
    let move_to_y: number = $state(0);

    let move_by_amount: number = $state(0);

    export interface DetectObjectResult {
        name: string;
        percentage: string;
    }

    listen<DetectObjectResult>("update-detected-object", (event) => {
        detected_object = event.payload;
    });

    async function test_magnet() {
        await invoke("test_magnet", { state: magnet_state });
        magnet_state = !magnet_state;
    }

    async function homing() {
        await invoke("homing");
    }

    async function test_actuator_extend() {
        await invoke("actuator_extend");
    }

    async function test_actuator_contract() {
        await invoke("actuator_contract");
    }

    export function openPopup(snippet: Snippet, onClose?: () => void) {
        popUpSnippet = snippet;
        popUpOnClose = onClose;
        isPopUpOpen = true;
    }

    async function exit() {
        await invoke("exit");
    }

    async function move_to_test() {
        await invoke("move_to", {
            x: move_to_x,
            y: move_to_y,
        });
    }

    async function move_by_test() {
        await invoke("move_by", {
            direction,
            amount: move_by_amount,
        });
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

    let rect: Rectangle | undefined = undefined;

    const directions: Direction[] = ["North", "South", "East", "West"];
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
        <h1 style="text-align: center;">Inventory</h1>
        <button class="button" style="width: 10rem;" onclick={exit}>Exit</button
        >
    </div>

    <button class="button" onclick={homing}>Homeing</button>

    <div class="move-by-test">
        <button class="button" onclick={move_by_test}>Move by</button>

        <select id="dir" bind:value={direction}>
            {#each directions as dir}
                <option value={dir}>{dir}</option>
            {/each}
        </select>

        <input type="number" bind:value={move_by_amount} />
    </div>

    <div class="move-to-test">
        <button class="button" onclick={move_to_test}>Move to</button>

        <b>X</b>
        <input type="number" bind:value={move_to_x} />
        <b>Y</b>
        <input type="number" bind:value={move_to_y} />
    </div>

    <div class="actuator-test">
        <button class="button" onclick={test_actuator_extend}
            >Extend actuator</button
        >
        <button class="button" onclick={test_actuator_contract}
            >Contract actuator</button
        >
    </div>
    <div class="magnet-test">
        <button class="button" onclick={test_magnet}>Toggle magnet</button>
    </div>
    <div class="items-container" style="padding: 1rem;">
        {#each displayItems as item}
            <Item
                image_source={item.image_path}
                item_name={item.display_name}
                item_amount={Number(item.amount)}
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
