<script lang="ts">
    import { openPopup } from "./+page.svelte";

    let { item_name, item_amount }: { item_name: string; item_amount: number } =
        $props();
</script>

<div class="item-container">
    <span class="item-info">{item_name}</span>
    <button class="button">Request</button>

    <div class="item-amount">has {item_amount} left</div>

    {#snippet popup()}
        <h1>Hello from item popup</h1>
    {/snippet}

    <button class="button" onclick={() => openPopup(popup)}>OpenPopup</button>
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
</style>
