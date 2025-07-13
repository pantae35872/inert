<script lang="ts">
    import { scale } from "svelte/transition";
    import { openPopup } from "./+page.svelte";

    let {
        item_name,
        item_amount,
        image_source,
    }: { item_name: string; item_amount: number; image_source: string } =
        $props();

    let amount: string = $state("");
    let numpadOn: boolean = $state(false);
    let left_amount = $derived(item_amount - Number(amount));

    function appendNumber(n: number) {
        amount += n;
    }

    function deleteLast() {
        amount = amount.slice(0, -1);
    }

    function clearInput() {
        amount = "";
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
            <form class="item-request-form" onsubmit={() => {}}>
                <h2 class="item-header">Request Item: {item_name}</h2>
                <div class="image-wrapper">
                    <img src={image_source} alt={item_name} />
                </div>

                <input
                    class="item-amount-input"
                    placeholder="Amount"
                    type="text"
                    onclick={() => (numpadOn = !numpadOn)}
                    value={amount}
                    required
                    readonly
                />
                <h1
                    class="item-amount-popup"
                    style="color: {left_amount < 0 ? 'red' : 'inherit'};"
                >
                    {left_amount} left
                </h1>
                <button class="button item-button" type="submit"
                    >Confirm Request</button
                >
            </form>
        </div>

        {#if numpadOn}
            <div
                class="numpad"
                transition:scale={{
                    duration: 200,
                }}
            >
                <div class="actual-numpad">
                    {#each [[1, 2, 3], [4, 5, 6], [7, 8, 9]] as row}
                        <div class="numpad-row">
                            {#each row as n}
                                <button onclick={() => appendNumber(n)}
                                    >{n}</button
                                >
                            {/each}
                        </div>
                    {/each}
                    <div class="numpad-row">
                        <button onclick={clearInput}>C</button>
                        <button onclick={() => appendNumber(0)}>0</button>
                        <button onclick={deleteLast}>⌫</button>
                    </div>
                </div>

                <button class="button" onclick={() => (numpadOn = false)}
                    >X</button
                >
            </div>
        {/if}
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
        onclick={() => openPopup(requestPopUp)}>Request</button
    >

    <div class="item-amount">has {item_amount} left</div>
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
        border: 1px solid var(--bg-color-2);
        border-radius: 0.5rem;
        box-shadow: 0 5px 5px var(--bg-color-2);

        width: 18rem;
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

    .item-request-popup,
    .numpad {
        background-color: var(--bg-color-3);
        padding: 10px;
        border: 1px solid var(--bg-color-2);
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

    .item-amount-input {
        outline: none;
        border-radius: 0.31rem;
        border: 2px solid var(--bg-color-2);
        background-color: var(--bg-color);
        color: var(--fg-color-2);
        padding: 0 1.25rem 0 3.12rem;
        font-size: 1.06rem;
        transition: 0.2s ease;

        padding: 0.5rem;
        margin: 0.1rem;
    }

    .item-amount-input::placeholder {
        color: var(--fg-color);
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
    }

    .numpad {
        display: flex;
        gap: 0.5rem;
        margin-left: 1rem;
    }

    .numpad .button {
        padding: 0.7rem;
    }

    .actual-numpad {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .numpad-row {
        display: flex;
        justify-content: space-between;
        gap: 0.5rem;
    }

    .numpad-row button {
        flex: 1;
        padding: 1rem;
        font-size: 1.25rem;
        border: 1px solid;
        background-color: var(--bg-color-4, #ddd);
        border-color: var(--bg-color-2);
        color: var(--fg-color, #000);
        border-radius: 0.5rem;
        cursor: pointer;
        transition: background 0.2s ease;
    }

    .numpad-row button:hover {
        background-color: var(--bg-color-3, #ccc);
    }

    .item-amount-popup {
        text-align: center;
        font-size: 1.2rem;
        font-weight: 600;
        margin: 0;
    }
</style>
