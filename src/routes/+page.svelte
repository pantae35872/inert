<script lang="ts" module>
    import Item from "./Item.svelte";
    import "../style.css";
    import Overlay from "./Overlay.svelte";
    import type { Snippet } from "svelte";

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
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
        <Item item_name="Motor" item_amount={20} />
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
        gap: 1em;

        justify-content: center;
        max-width: max-content;
        margin: 0 auto;
    }
</style>
