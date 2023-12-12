
<script lang="ts">
    import Titlebar from "./Titlebar.svelte";

    
    import { open } from "@tauri-apps/api/dialog";
    import { writable } from 'svelte/store';
    import { onMount } from 'svelte';
    // export const wallpaper_store = writable([]);

    import { readDir } from '@tauri-apps/api/fs';
    let wallpapers = [{}];

    async function readFolderEntries(path){
        
        let acceptedVideoFiles = ['mp4', '3gp', 'avi', 'webm', 'm4v', 'mov'];

        try {
            // $wallpaper_store = [];
            
            const entries = await readDir(path, {recursive: false});
    
            for (const entry of entries){
                let entryPath = entry.path;
                let entryExtension = entryPath.split(".").pop() || "";
                if(acceptedVideoFiles.includes(entryExtension)){
                    wallpapers = [...wallpapers, {
                        path: entry.path || "",
                        name: entry.name || "No Name",
                    }];
                    }
            }
        }catch(err){
            console.error("reading folder entries failed", error.message);
        }
    }

    async function getFolderPath(){
        try{
            const selectedPath = await open({
                    multiple: false,
                    title: "Open Wallpaper Folder",
                    directory: true,
                });
                console.log(String(selectedPath));
                await readFolderEntries(String(selectedPath))
                await loadWallpapers([{}]);
        }catch(error){
            console.error("folder path retrieval failed", error.message);
        }
    }
</script>

<section class="bg-black">
    <div id="background"></div>
    <Titlebar/>
    <slot/>
</section>

<style>
    
    #background{
        position: absolute;
        height: 100vh;
        width: 100vw    ;
        top: 0;
        left: 0;
        z-index: -1;
        background: radial-gradient(circle at center, #0e1018, #050508);
        backdrop-filter: blur(100px);
        border-radius: 10px;
    }
    
    section{
        width: 100vw;
        height: 100vh;
        background-color: #1A1C25;
        display: flex;
        flex-direction: column;
        margin: 0;
        padding-top: 30px;
        z-index: 2;
        position: relative;
    }
</style>