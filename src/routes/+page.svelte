<script>
    // import { appWindow } from '@tauri-apps/api/window'
    import PathSelection from "$lib/components/path/PathSelection.svelte";
    import Wallpaper from "$lib/components/wallpapers/Wallpaper.svelte";
    import WallpaperArea from "$lib/components/wallpapers/WallpaperArea.svelte";
    import Titlebar from "$lib/components/window/Titlebar.svelte";
    import Window from "$lib/components/window/Window.svelte";
    import "../app.css";
</script>

<Window>
    <Titlebar/>
    <PathSelection/>
    <WallpaperArea> 
        <Wallpaper src={"/img/example2.png"}/>
        <Wallpaper src={"/img/example2.png"}/>
        <Wallpaper src={"/img/example2.png"}/>
        <Wallpaper src={"/img/example2.png"}/>
        <Wallpaper src={"/img/example2.png"}/>
        
    </WallpaperArea> 
</Window>

