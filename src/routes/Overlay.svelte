<script lang="ts">
    import type { Component, Snippet } from "svelte";
    import { stopPropagation } from "svelte/legacy";

    let {
        open,
        onClose,
        children,
    }: { open: boolean; onClose: () => void; children: Snippet } = $props();

    function closeHandler() {
        open = false;
        onClose();
    }
</script>

{#if open}
    <div
        class="overlay"
        role="button"
        tabindex="0"
        onkeydown={(_e) => {}}
        onclick={closeHandler}
    >
        <div
            class="children-wrapper"
            role="button"
            tabindex="0"
            onkeydown={(_e) => {}}
            onclick={(event) => event.stopPropagation()}
        >
            {@render children?.()}
        </div>
    </div>
{/if}

<style>
    .overlay {
        position: fixed;
        inset: 0;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 998;
    }

    .children-wrapper {
        z-index: 999;
    }
</style>
