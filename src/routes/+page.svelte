<script lang="ts" module>
    import Item from "./Item.svelte";
    import "../style.css";
    import Overlay from "./Overlay.svelte";
    import type { Snippet } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    let popUpSnippet: Snippet | undefined = $state(undefined);
    let isPopUpOpen: boolean = $state(false);

    export function openPopup(snippet: Snippet) {
        popUpSnippet = snippet;
        isPopUpOpen = true;
    }

    async function exit() {
        await invoke("exit");
    }
</script>

<main class="container no-select">
    <Overlay bind:open={isPopUpOpen}>
        {@render popUpSnippet?.()}
    </Overlay>
    <div
        style="display: flex; align-items: center; text-align: center; justify-content: space-between; width: 100%;"
    >
        <div></div>
        <h1 style="text-align: center;">Inventory</h1>
        <button class="button" onclick={exit}>Exit</button>
    </div>
    <div class="items-container">
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
    .container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: flex-end;
        gap: 1em;
        margin-right: 1rem;
        margin-left: 1rem;
    }

    .items-container {
        display: flex;
        flex-wrap: wrap;
        gap: 1rem;

        justify-content: center;
        width: 100%;
        max-width: 100%;
    }
</style>
