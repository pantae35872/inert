<script lang="ts" module>
    import Item from "./Item.svelte";
    import "../style.css";
    import Overlay from "./Overlay.svelte";
    import type { Snippet } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import RequestItemQueue from "./RequestItemQueue.svelte";
    import AddItemPopup from "./AddItemPopup.svelte";
    import { type Direction } from "../bindings/Direction";

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

    const directions: Direction[] = ["North", "South", "East", "West"];

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
