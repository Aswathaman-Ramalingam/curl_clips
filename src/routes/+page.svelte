<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import Paste from "../icons/paste.svelte";
    import type { YtDlpResponse, YtDlpFormat } from "../constants/types";
    let inputElem: HTMLInputElement;
    let videoFormats: Array<YtDlpFormat> = [];
    let audioFormats: Array<YtDlpFormat> = [];
    let loading = false;
    let error = "";
    let selectedVideoFormat: string | null = null;
    let selectedAudioFormat: string | null = null;

    async function handlePasteIconClick(): Promise<void> {
        try {
            const text = await navigator.clipboard.readText();
            if (text && inputElem) {
                inputElem.value = text;
            }
        } catch (err) {
            error = "Failed to read clipboard contents";
        }
    }

    async function fetchFormats() {
        const url = inputElem.value.trim();
        console.log("Fetching formats for URL:", url);
        if (!url) {
            error = "Please paste a YouTube URL";
            return;
        }
        loading = true;
        error = "";
        videoFormats = [];
        audioFormats = [];
        selectedVideoFormat = null;
        selectedAudioFormat = null;

        try {
            const response: YtDlpResponse = await invoke("fetch_formats", {
                url,
            });
            console.log("Response from backend:", response);
            videoFormats = response.formats
                .filter((f) => f.vcodec !== "none" && f.acodec === "none")
                .sort((a, b) => (b.height || 0) - (a.height || 0));
            audioFormats = response.formats
                .filter((f) => f.acodec !== "none" && f.vcodec === "none")
                .sort((a, b) => (b.tbr || 0) - (a.tbr || 0));
            console.log("Video formats:", videoFormats);
            console.log("Audio formats:", audioFormats);
        } catch (e) {
            error =
                "Failed to fetch formats. Please check the URL or try again.";
        }
        loading = false;
    }

    function formatBytes(bytes: number | undefined, decimals = 2) {
        if (!bytes || bytes === 0) return "0 Bytes";
        const k = 1024;
        const dm = decimals < 0 ? 0 : decimals;
        const sizes = ["Bytes", "KB", "MB", "GB", "TB"];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return (
            parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + " " + sizes[i]
        );
    }

    function handleDownload() {
        console.log("Selected video format:", selectedVideoFormat);
        console.log("Selected audio format:", selectedAudioFormat);
        // TODO: Implement download logic
    }
</script>

<div class="min-h-screen bg-base flex flex-col items-center px-4 py-12">
    <h1
        class="text-5xl md:text-5xl font-semibold leading-tight text-text tracking-tight mb-8 text-center"
    >
        curl-clips
    </h1>
    <div class="flex flex-col gap-4 w-full max-w-md relative">
        <div class="relative">
            <input
                type="text"
                placeholder="Paste your URL here"
                aria-label="YouTube link"
                class="w-full px-4 pr-10 py-3 bg-white backdrop-blur border border-accent/20 rounded-md shadow-sm text-accent placeholder-text focus:outline-none focus:ring-2 focus:ring-surface transition z-10"
                bind:this={inputElem}
            />
            <button
                type="button"
                aria-label="Paste"
                on:click={handlePasteIconClick}
                class="absolute m-3 right-1 top-0 z-20 bg-transparent p-0 cursor-pointer transition-colors"
            >
                <Paste class="text-text w-6 h-6 hover:text-accent " />
            </button>
        </div>

        <button
            type="submit"
            class="py-3 bg-accent text-white text-lg font-medium rounded-md hover:bg-accent/90 transition-colors shadow-sm z-10 disabled:bg-surface disabled:cursor-not-allowed"
            on:click={fetchFormats}
            disabled={loading}
        >
            {#if loading}
                Fetching...
            {:else}
                Fetch Formats
            {/if}
        </button>
    </div>

    {#if error}
        <p class="text-error mt-4">{error}</p>
    {/if}

    {#if !loading && (videoFormats.length > 0 || audioFormats.length > 0)}
        <div class="w-full max-w-4xl mt-8">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                <!-- Video Formats -->
                <div>
                    <h2 class="text-2xl font-bold text-text mb-4">
                        Video Formats
                    </h2>
                    <div class="flex flex-col gap-2">
                        {#each videoFormats as format}
                            <label
                                class="flex items-center gap-4 p-3 rounded-md border border-accent/20 hover:bg-surface transition cursor-pointer"
                            >
                                <input
                                    type="radio"
                                    name="video-format"
                                    value={format.format_id}
                                    bind:group={selectedVideoFormat}
                                    class="radio radio-accent"
                                />
                                <div class="flex-grow">
                                    <p class="font-medium text-text">
                                        {format.height}p{format.fps
                                            ? `@${format.fps}`
                                            : ""}
                                        <span class="text-sm text-subtext"
                                            >({format.ext})</span
                                        >
                                    </p>
                                    <p class="text-sm text-subtext">
                                        {formatBytes(format.filesize)}
                                    </p>
                                </div>
                            </label>
                        {/each}
                    </div>
                </div>

                <!-- Audio Formats -->
                <div>
                    <h2 class="text-2xl font-bold text-text mb-4">
                        Audio Formats
                    </h2>
                    <div class="flex flex-col gap-2">
                        {#each audioFormats as format}
                            <label
                                class="flex items-center gap-4 p-3 rounded-md border border-accent/20 hover:bg-surface transition cursor-pointer"
                            >
                                <input
                                    type="radio"
                                    name="audio-format"
                                    value={format.format_id}
                                    bind:group={selectedAudioFormat}
                                    class="radio radio-accent"
                                />
                                <div class="flex-grow">
                                    <p class="font-medium text-text">
                                        {format.format_note || format.acodec}
                                        <span class="text-sm text-subtext"
                                            >({format.ext})</span
                                        >
                                    </p>
                                    <p class="text-sm text-subtext">
                                        {formatBytes(format.filesize)} | {Math.round(
                                            format.tbr || 0,
                                        )}kbps
                                    </p>
                                </div>
                            </label>
                        {/each}
                    </div>
                </div>
            </div>

            <div class="mt-8 text-center">
                <button
                    class="py-3 px-8 bg-love text-white text-lg font-medium rounded-md hover:bg-love/90 transition-colors shadow-sm disabled:bg-surface disabled:cursor-not-allowed"
                    on:click={handleDownload}
                    disabled={!selectedVideoFormat && !selectedAudioFormat}
                >
                    Download
                </button>
            </div>
        </div>
    {/if}
</div>
