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
        <button type="button" id="path">Path Selection</button>
        <input id="search" placeholder="Search Wallpaper"/>
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
    </section>
</Window>

<style>
    .hello{
        color: black;
        font-size: 10rem;
    }
    #path{
        border-radius: 10px;
        color: #ffffff99;
        padding: 0 20px 0 20px;
        background-color: #4375C9;
        height: 40px;
    }
    #search{
        background-color: #121319;
        width: 70%;
        height: 40px;
        border-radius: 10px;
        color: #ffffff99;
        padding: 0 20px 0 20px;
        font-weight: lighter;
    }

    #path-selection{
        height: 80px;
        width: 100vw;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: start;
        padding: 0 30px 0 30px;
        gap: 10px;
    }

    #wallpaper-previews{
        padding: 30px 30px;
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
