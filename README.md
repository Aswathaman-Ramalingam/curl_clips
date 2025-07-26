# Curl Clips

Curl Clips is a desktop application that allows you to download video and audio from YouTube. It's built with Tauri, Svelte, and Rust, and it uses `yt-dlp` to fetch and download the media.

## Features

-   Paste a YouTube URL to fetch available formats.
-   Separate lists for video-only, audio-only, and default (video+audio) formats.
-   Select your desired format and download the media.
-   Downloads are saved to your system's default downloads directory.

## Tech Stack

-   **Frontend:** Svelte, Tailwind CSS
-   **Backend:** Rust, Tauri
-   **Core Dependency:** `yt-dlp`

## Prerequisites

-   [Node.js](https://nodejs.org/) and [Rust](https://www.rust-lang.org/tools/install) installed.
-   `yt-dlp` installed and available in your system's `PATH`.

## Getting Started

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/your-username/curl_clips.git
    cd curl_clips
    ```

2.  **Install the dependencies:**

    ```bash
    npm install
    ```

3.  **Run the application in development mode:**

    ```bash
    npm run tauri dev
    ```

4.  **Build the application for production:**

    ```bash
    npm run tauri build
    ```

## How It Works

The application uses a Svelte frontend to interact with a Rust backend through Tauri's API. When you paste a URL and click "Fetch Formats," the `fetch_formats` command is invoked in the Rust backend. This command runs `yt-dlp` with the `--dump-json` flag to get a list of available formats for the given URL. The formats are then parsed and sent back to the frontend to be displayed.

When you click "Download," the `download` command is invoked with the selected format(s). The Rust backend then uses `yt-dlp` to download the media to your system's default downloads directory.
