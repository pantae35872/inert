<script lang="ts">
    import { scale } from "svelte/transition";

    let caps = $state(false);
    let keyboardLayout: string[][] = $derived(
        caps
            ? [
                  ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"],
                  ["A", "S", "D", "F", "G", "H", "J", "K", "L"],
                  ["Z", "X", "C", "V", "B", "N", "M"],
              ]
            : [
                  ["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"],
                  ["a", "s", "d", "f", "g", "h", "j", "k", "l"],
                  ["z", "x", "c", "v", "b", "n", "m"],
              ],
    );

    function toggleCaps() {
        caps = !caps;
    }

    let {
        keys = $bindable(),
        keyboardOn,
        closeBtn = true,
    }: { keys: string; keyboardOn: boolean; closeBtn?: boolean } = $props();

    function appendChar(char: string) {
        keys += caps ? char.toUpperCase() : char.toLowerCase();
    }

    function deleteLast() {
        keys = keys.slice(0, -1);
    }

    function clearInput() {
        keys = "";
    }
</script>

{#if keyboardOn}
    <div
        class="numpad"
        transition:scale={{
            duration: 200,
        }}
    >
        <div class="actual-keyboard">
            <div class="keyboard-row">
                {#each ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"] as char}
                    <button onclick={() => appendChar(char)}>{char}</button>
                {/each}
            </div>

            {#each keyboardLayout as row}
                <div class="keyboard-row">
                    {#each row as char}
                        <button onclick={() => appendChar(char)}>{char}</button>
                    {/each}
                </div>
            {/each}
            <div class="keyboard-row">
                <button onclick={toggleCaps}>Caps</button>
                <button onclick={clearInput}>Clear</button>
                <button onclick={() => appendChar(" ")}>Space</button>
                <button onclick={deleteLast}>⌫</button>
            </div>
        </div>
        {#if closeBtn}
            <button class="button" onclick={() => (keyboardOn = false)}
                >X</button
            >
        {/if}
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

    .actual-keyboard {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .keyboard-row {
        display: flex;
        justify-content: space-between;
        gap: 0.5rem;
    }

    .keyboard-row button {
        flex: 1;
        padding: 0.2rem;
        font-size: 1.25rem;
        min-width: 2rem;
        border: 1px solid;
        background-color: var(--bg-color-4, #ddd);
        border-color: var(--border-color);
        color: var(--fg-color, #000);
        border-radius: 0.5rem;
        cursor: pointer;
        transition: background 0.2s ease;
    }

    .keyboard-row button:hover {
        background-color: var(--bg-color-3, #ccc);
    }
</style>
