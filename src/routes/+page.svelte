<script>
    // import { appWindow } from '@tauri-apps/api/window' 
    import "../app.css";
    import Titlebar from "$lib/components/window/Titlebar.svelte"
    import Window from "$lib/components/window/Window.svelte"
    import WallpaperPreview from "$lib/components/video/WallpaperPreview.svelte"
</script>

<Titlebar/>
<Window>
    <section id="path-selection">
    </section>
    <section id="wallpaper-previews">
        <WallpaperPreview />
        <WallpaperPreview />
        <WallpaperPreview />
        <WallpaperPreview />
        <WallpaperPreview />
        <WallpaperPreview />
        <WallpaperPreview />
        <WallpaperPreview />
        <WallpaperPreview />
        <WallpaperPreview />
        <WallpaperPreview />
        <WallpaperPreview />
        <WallpaperPreview />
        <WallpaperPreview />
        <WallpaperPreview />
    </section>
</Window>

<style>
    .hello{
        color: black;
        font-size: 10rem;
    }
    #path-selection{
        height: 80px;
        width: 100vw;
        background-color: red;
        display: flex;
        flex-direction: row;
    }
    #wallpaper-previews{
        padding: 30px 10px 30px 10px;
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        gap: 25px;
        height: 100vh;
        max-height: 100vh;
        width: 700px;
        overflow-y: scroll;
    }
</style>
