<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import Item from "./Item.svelte";
    import "../style.css";

    async function toggleGpio23(event: Event) {
        await invoke("toggle_gpio_23");
    }
    async function exit() {
        await invoke("exit");
    }
</script>

<main class="container">
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
