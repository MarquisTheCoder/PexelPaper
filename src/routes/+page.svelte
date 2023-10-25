<script>
    // import { appWindow } from '@tauri-apps/api/window' 
    import "../app.css";
    import Titlebar from "$lib/components/window/Titlebar.svelte"
    import Window from "$lib/components/window/Window.svelte"
</script>

<Titlebar/>
<Window>
    <section id="path-selection">
       <input/> 
    </section>
    <section></section>
</Window>

<style>
    .hello{
        color: black;
        font-size: 10rem;
    }
    #path-selection{
        height: 50px;
        width: 100vw;
        background-color: red;
        display: flex;
        flex-direction: row;
    }
</style>
