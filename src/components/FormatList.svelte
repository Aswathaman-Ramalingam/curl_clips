<script lang="ts">
    import type { YtDlpFormat } from "../constants/types";
    import { formatBytes } from "../lib/utils";
    export let title: string;
    export let formats: YtDlpFormat[];
    export let group: string | null;
    export let name: string;
    export let onSelectionChange: () => void;
</script>

<div>
    <h2 class="text-2xl font-bold text-text my-2 px-4">{title}</h2>
    <div class="flex flex-col gap-2 max-h-120 px-4 py-2 overflow-scroll">
        {#each formats as format}
            <label
                class="flex items-center gap-4 p-3 rounded-md border border-accent/20 hover:bg-surface transition cursor-pointer"
            >
                <input
                    type="radio"
                    {name}
                    value={format.format_id}
                    bind:group
                    on:change={onSelectionChange}
                    class="radio radio-accent"
                />
                <div class="flex-grow">
                    <slot {format} {formatBytes} />
                </div>
            </label>
        {/each}
    </div>
</div>
