<script>
    // import { appWindow } from '@tauri-apps/api/window'
    import Titlebar from "$lib/components/window/Titlebar.svelte";
    import Window from "$lib/components/window/Window.svelte";
    import "../app.css";
</script>

<Window>
    <Titlebar/>
    <WallpaperArea></WallpaperArea>
</Window>

