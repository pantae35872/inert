<script lang="ts" module>
    import { invoke } from "@tauri-apps/api/core";
    import Item from "./Item.svelte";
    import "../style.css";
    import Overlay from "./Overlay.svelte";
    import type { Snippet } from "svelte";

    async function toggleGpio23(event: Event) {
        await invoke("toggle_gpio_23");
    }
    async function exit() {
        await invoke("exit");
    }

    let popUpSnippet: Snippet | undefined = $state(undefined);
    let isPopUpOpen: boolean = $state(false);

    export function openPopup(snippet: Snippet) {
        popUpSnippet = snippet;
        isPopUpOpen = true;
    }
</script>

<main class="container">
    <Overlay bind:open={isPopUpOpen}>
        {@render popUpSnippet?.()}
    </Overlay>

    <h1>Inventory</h1>

    <div class="items-container">
        <Item item_name="Resistor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
    </div>
    <!-- <button class="button" onclick={toggleGpio23}>Toggle GPIO 23</button> 
    <button class="button" onclick={exit}>Exit</button> -->
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
        gap: 1em;

        justify-content: center;
        max-width: max-content;
        margin: 0 auto;
    }
</style>
