<script>
    // import { appWindow } from '@tauri-apps/api/window' 
    import "../app.css";
    import Titlebar from "$lib/components/window/Titlebar.svelte"
    import Window from "$lib/components/window/Window.svelte"
    import PathSelection from "$lib/components/path/PathSelection.svelte"
    import WallpaperArea from "$lib/components/wallpapers/WallpaperArea.svelte"
    import Wallpaper from "$lib/components/wallpapers/Wallpaper.svelte"
</script>

<Titlebar/>
<Window>
    <PathSelection/>
    <WallpaperArea>
        <Wallpaper/>
        <Wallpaper/>
        <Wallpaper/>
        <Wallpaper/>
        <Wallpaper/>
        <Wallpaper/>
        <Wallpaper/>
        <Wallpaper/>
        <Wallpaper/>
        <Wallpaper/>
    </WallpaperArea>
</Window>

