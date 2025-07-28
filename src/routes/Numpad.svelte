<script lang="ts">
    import { scale } from "svelte/transition";

    let {
        amount = $bindable(),
        numpadOn,
    }: { amount: string; numpadOn: boolean } = $props();

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
                        <button onclick={() => appendNumber(n)}>{n}</button>
                    {/each}
                </div>
            {/each}
            <div class="numpad-row">
                <button onclick={clearInput}>C</button>
                <button onclick={() => appendNumber(0)}>0</button>
                <button onclick={deleteLast}>⌫</button>
            </div>
        </div>

        <button class="button" onclick={() => (numpadOn = false)}>X</button>
    </div>
{/if}

<style>
    .numpad {
        background-color: var(--bg-color-3);
        padding: 10px;
        border: 1px solid var(--border-color);
        border-radius: 0.5rem;
        box-shadow: 0 2px 2px var(--bg-color-2);

        display: flex;
        gap: 0.5rem;
        margin-left: 1rem;

        pointer-events: auto;
    }

    .numpad > * {
        pointer-events: auto;
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
        border-color: var(--border-color);
        color: var(--fg-color, #000);
        border-radius: 0.5rem;
        cursor: pointer;
        transition: background 0.2s ease;
    }

    .numpad-row button:hover {
        background-color: var(--bg-color-3, #ccc);
    }
</style>
