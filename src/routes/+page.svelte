<script>
    // import { appWindow } from '@tauri-apps/api/window'
    import PathSelection from "$lib/components/path/PathSelection.svelte";
    import Wallpaper from "$lib/components/wallpapers/Wallpaper.svelte";
    import WallpaperArea from "$lib/components/wallpapers/WallpaperArea.svelte";
    import Titlebar from "$lib/components/window/Titlebar.svelte";
    import Window from "$lib/components/window/Window.svelte";
    import { wallpaper_store } from "$lib/middleware/store";
    import "../app.css";
</script>

<Window>
    <Titlebar/>
    <PathSelection/>
    <WallpaperArea> 
        {#each $wallpaper_store as path}
            <svelte:component this={Wallpaper} src={""} {path}/>
        {/each}
    </WallpaperArea> 
</Window>

