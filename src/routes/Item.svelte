<script lang="ts">
    import { openPopup } from "./+page.svelte";

    let { item_name, item_amount }: { item_name: string; item_amount: number } =
        $props();
</script>

{#snippet requestPopUp()}
    <div class="item-request-popup">
        <form class="item-request-form" onsubmit={() => {}}>
            <h2 class="item-header">Request Item: {item_name}</h2>

            <div class="image-wrapper">
                <img
                    src="https://www.w3schools.com/css/paris.jpg"
                    alt={item_name}
                />
            </div>

            <input
                class="item-amount-input"
                placeholder="Amount"
                type="text"
                required
            />
            <button class="button item-button" type="submit"
                >Confirm Request</button
            >
        </form>
    </div>
{/snippet}

<div class="item-container">
    <span class="item-info">{item_name}</span>
    <button class="button" onclick={() => openPopup(requestPopUp)}
        >Request</button
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

        min-width: 10em;
        min-height: 8em;
        background-color: var(--bg-color-3);
        padding: 10px;
        border: 1px solid var(--bg-color-2);
        border-radius: 0.5rem;
        box-shadow: 0 5px 5px var(--bg-color-2);
    }

    .item-info {
        text-align: center;
        font-size: 1.4rem;
    }

    .item-amount {
        position: absolute;
        right: 0.4em;
        bottom: 0.2em;
    }

    .item-request-popup {
        min-height: 18rem;
        min-width: 18rem;

        max-width: 30rem;
        max-height: 20rem;

        background-color: var(--bg-color-3);
        padding: 10px;
        border: 1px solid var(--bg-color-2);
        border-radius: 0.5rem;
        box-shadow: 0 2px 2px var(--bg-color-2);

        display: flex;
        flex-direction: column;
        overflow: auto; /* Prevent child overflow */

        height: 100%;
    }

    .item-request-form {
        display: flex;
        flex-direction: column;
        gap: 0.8rem;

        max-height: 100%; /* Don't exceed popup height */
    }

    .item-header {
        text-align: center;
        font-size: 1.37rem;
        font-weight: 600;
        margin-bottom: 1.87rem;
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
</style>
