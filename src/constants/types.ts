export type YtDlpFormat = {
    format_id: string;
    format_note: string;
    ext: string;
    vcodec: string;
    acodec: string;
    filesize?: number;
    height?: number;
    width?: number;
    fps?: number;
    tbr?: number;
}

export type YtDlpResponse = {
    id: string;
    title: string;
    formats: YtDlpFormat[];
}



