<script lang="ts">
    import { scale } from "svelte/transition";
    import { closePopUp, openPopup } from "./+page.svelte";
    import Numpad from "./Numpad.svelte";
    import { invoke } from "@tauri-apps/api/core";

    let {
        item_name,
        item_amount,
        image_source,
        item_id,
    }: {
        item_name: string;
        item_amount: number;
        image_source: string;
        item_id: number;
    } = $props();

    type Stage = "Requesting" | "Loading";

    let stage: Stage = $state("Requesting");

    async function requestItem() {
        startLoadingAnimation();
        stage = "Loading";
        await invoke("remove_item", {
            id: item_id,
        });

        stopLoadingAnimation();
        closePopUp();
    }

    let loadingDots: string = $state("");
    let interval: number;

    function startLoadingAnimation() {
        let count = 0;

        interval = setInterval(() => {
            count = (count + 1) % 4;
            loadingDots = ".".repeat(count);
        }, 500); // Change dot every 500ms
    }

    function stopLoadingAnimation() {
        clearInterval(interval);
        loadingDots = "";
    }

    function cancel() {
        closePopUp();
    }
</script>

{#snippet requestPopUp()}
    <div
        class="request-wrapper"
        transition:scale={{
            duration: 200,
        }}
    >
        <div class="item-request-popup">
            {#if stage == "Requesting"}
                <form class="item-request-form" onsubmit={requestItem}>
                    <h2 class="item-header">
                        Request Item: {item_name}, ({item_amount} items)
                    </h2>
                    <div class="image-wrapper">
                        <img src={image_source} alt={item_name} />
                    </div>

                    <button class="button item-button" type="submit"
                        >Confirm Request</button
                    >
                    <button
                        class="button item-button"
                        onclick={closePopUp}
                        type="reset">Cancel</button
                    >
                </form>
            {:else if stage == "Loading"}
                <h1>Loading{loadingDots}</h1>
            {/if}
        </div>
    </div>
{/snippet}

<div class="item-container">
    <span class="item-info">{item_name}</span>

    <div class="image-wrapper">
        <img src={image_source} alt={item_name} />
    </div>

    <button
        class="button"
        style="margin-bottom: 2rem;"
        onclick={() => openPopup(requestPopUp, () => {}, false)}>Request</button
    >
    <div class="item-amount">has {item_amount} items</div>
</div>

<style>
    .item-container {
        display: flex;
        gap: 1em;
        flex-direction: column;
        align-items: center;
        position: relative;

        background-color: var(--bg-color-3);
        padding: 10px;
        border: 1px solid var(--border-color);
        border-radius: 0.5rem;
        box-shadow: 0 5px 5px var(--bg-color-2);

        width: 17.5rem;
        height: 20rem;
        max-width: 18rem;
        max-height: 20rem;
    }

    .item-info {
        text-align: center;
        font-size: 1.4rem;
    }

    .item-amount {
        position: absolute;
        right: 0.4rem;
        bottom: 0.2rem;
    }

    .item-request-popup {
        background-color: var(--bg-color-3);
        padding: 10px;
        border: 1px solid var(--border-color);
        border-radius: 0.5rem;
        box-shadow: 0 2px 2px var(--bg-color-2);
    }

    .item-request-popup {
        min-height: 18rem;
        min-width: 18rem;

        max-width: 20rem;
        max-height: 20rem;

        display: flex;
        flex-direction: column;
        overflow: auto; /* Prevent child overflow */
        height: 100%;

        transition: transform 0.2s ease;
    }

    .item-request-form {
        display: flex;
        flex-direction: column;
        gap: 0.8rem;

        max-height: 100%; /* Don't exceed popup height */
    }

    .request-wrapper {
        display: flex;
        width: 100%;
        height: 100%;
        justify-content: center;
        align-items: center;
        pointer-events: none;
    }

    .request-wrapper > * {
        pointer-events: auto;
    }

    .item-header {
        text-align: center;
        font-size: 1.37rem;
        font-weight: 600;
        margin: 0;
    }

    .image-wrapper {
        flex: 1 1 auto;
        overflow: hidden;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .image-wrapper img {
        max-width: 100%;
        max-height: 100%;
        object-fit: contain;

        border-radius: 0.31rem;
    }
</style>
