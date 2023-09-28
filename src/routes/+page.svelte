<script>
    // import { appWindow } from '@tauri-apps/api/window'
    import Titlebar from "$lib/components/window/Titlebar.svelte"
    import Window from "$lib/components/window/Window.svelte"
import { P } from "@tauri-apps/api/event-41a9edf5"
</script>




<Window>
    <Titlebar/>
    <section>
        <p>hello</p>
    </section>
</Window>

<style>
    section{
        color: black;
    }
</style>