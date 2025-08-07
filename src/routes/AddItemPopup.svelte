<script lang="ts">
    import { scale } from "svelte/transition";
    import type { DetectObjectResult } from "./+page.svelte";
    import Numpad from "./Numpad.svelte";
    import Keyboard from "./Keyboard.svelte";
    import type { PrepareAddItemStatus } from "../bindings/PrepareAddItemStatus";
    import type { Rectangle } from "../bindings/Rectangle";
    import { invoke } from "@tauri-apps/api/core";

    let {
        camera_url,
        detected_object,
    }: { camera_url?: string; detected_object?: DetectObjectResult } = $props();

    let amount: string = $state("");
    let itemNameKeys: string = $state("");

    let numpadOn: boolean = $state(false);
    let keyboardOn: boolean = $state(false);

    let prepareItemLoading: boolean = $state(false);

    type Stage = "Preparing" | "Loading" | "Confirming";

    let stage: Stage = $state("Preparing");
    let rect: Rectangle | undefined = undefined;

    async function addItem() {
        prepareItemLoading = true;

        startLoadingAnimation();
        stage = "Loading";
        let status = await invoke<PrepareAddItemStatus>("prepare_add_item");
        if (status == "NoSpaceLeft") {
            console.log("TODO");
            return;
        } else if ("Success" in status) {
            rect = status.Success;
        }

        prepareItemLoading = false;
        stopLoadingAnimation();
        stage = "Confirming";
    }

    async function confirmAddItem() {
        await invoke("confirm_add_item", {
            name: itemNameKeys,
            rect,
            amount: Number(amount),
        });
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
</script>

<div
    class="add-item-wrapper"
    transition:scale={{
        duration: 200,
    }}
>
    <div class="add-item">
        {#if stage == "Preparing"}
            <div
                style="margin: 1rem; display: flex; justify-content: center; flex-direction: column; align-items: center;"
            >
                <h1>Add Item ?</h1>
                <button class="button" onclick={addItem}>Confirm</button>
            </div>
        {:else if stage == "Loading"}
            <h1>Loading{loadingDots}</h1>
        {:else if stage == "Confirming"}
            <form class="add-item-form" onsubmit={confirmAddItem}>
                <h2 style="font-size: 1rem; margin: 0.1rem;">
                    Please put your item below the magnetic head
                </h2>
                {#if camera_url}
                    <div class="image-wrapper">
                        <img src={camera_url} alt="Camera Stream" />
                    </div>
                {/if}

                {#if detected_object}
                    <p style="font-size: 0.8rem; margin: 0; padding: 0;">
                        {detected_object.name}
                        {detected_object.percentage}
                    </p>
                {:else}
                    <p style="font-size: 0.8rem; margin: 0; padding: 0;">
                        Detecting...
                    </p>
                {/if}
                <input
                    class="item-amount-input"
                    placeholder="Detecting... (Item name)"
                    value={itemNameKeys.length == 0
                        ? detected_object?.name
                        : itemNameKeys}
                    onclick={async () => {
                        if (numpadOn) {
                            numpadOn = false;

                            await new Promise((resolve) =>
                                setTimeout(resolve, 350),
                            );
                        }
                        keyboardOn = !keyboardOn;
                    }}
                    type="text"
                    required
                    readonly
                />
                <input
                    class="item-amount-input"
                    placeholder="Amount"
                    type="text"
                    onclick={async () => {
                        if (keyboardOn) {
                            keyboardOn = false;

                            await new Promise((resolve) =>
                                setTimeout(resolve, 350),
                            );
                        }
                        numpadOn = !numpadOn;
                    }}
                    value={amount}
                    required
                    readonly
                />
                <button
                    class="button"
                    style="font-size: 0.5rem; width: 84%;"
                    type="submit">Add</button
                >
            </form>
        {/if}
    </div>

    <Numpad bind:amount {numpadOn} />
    <Keyboard bind:keys={itemNameKeys} {keyboardOn} />
</div>

<style>
    .add-item-wrapper {
        display: flex;
        padding: 10rem;
        width: 100%;
        height: 100%;
        justify-content: center;
        align-items: center;
        pointer-events: none;
    }

    .add-item-wrapper > * {
        pointer-events: auto;
    }

    .add-item {
        max-height: 80%;
        max-width: 40%;

        background-color: var(--bg-color-3);
        padding: 10px;
        border: 1px solid var(--border-color);
        border-radius: 0.5rem;
        box-shadow: 0 2px 2px var(--bg-color-2);

        display: flex;
        flex-direction: column;
        overflow: auto;
    }

    .add-item-form {
        display: flex;
        flex-direction: column;
        overflow: auto;
        gap: 0.3rem;

        align-items: center;
        justify-content: center;

        max-height: 100%;
        height: 100%;
    }

    .item-amount-input {
        outline: none;
        border-radius: 0.31rem;
        border: 2px solid var(--border-color);
        background-color: var(--bg-color);
        color: var(--fg-color-2);
        font-size: 0.8rem;
        transition: 0.2s ease;

        padding: 0.3rem;
        margin: 0.1rem;

        width: 80%;
    }

    .item-amount-input::placeholder {
        color: var(--fg-color-2);
    }

    .image-wrapper {
        max-width: 80%;

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
