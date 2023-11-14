<script>
    // import { appWindow } from '@tauri-apps/api/window'
    import PathSelection from "$lib/components/path/PathSelection.svelte";
    import WallpaperArea from "$lib/components/wallpapers/WallpaperArea.svelte";
    import Titlebar from "$lib/components/window/Titlebar.svelte";
    import Window from "$lib/components/window/Window.svelte";
    import { wallpaper_store } from "$lib/middleware/store";
    import "../app.css";

    $wallpaper_store = [...$wallpaper_store];
</script>

<Window>
    <Titlebar/>
    <PathSelection/>
    <WallpaperArea/>
</Window>

