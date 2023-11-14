<script>
    // import { appWindow } from '@tauri-apps/api/window'
    import Titlebar from "$lib/components/window/Titlebar.svelte";
    import Window from "$lib/components/window/Window.svelte";
    import "../app.css";
</script>

<Window>
    <Titlebar/>
    <!-- <PathSelection/>
    <WallpaperArea>
        {#each $wallpaper_store as path}
            <svelte:component this={Wallpaper} src={""} {path}/>
        {/each}
    </WallpaperArea> -->
</Window>

