<script lang="ts">
	import PathButton from "$lib/components/path/PathButton.svelte";
	import SearchInput from "$lib/components/path/SearchInput.svelte";
	import Window from "$lib/components/window/Window.svelte";
	import "../app.css";

    import { open } from "@tauri-apps/api/dialog";
    import { writable } from 'svelte/store';
    import { readDir } from '@tauri-apps/api/fs';
	import WallpaperArea from "$lib/components/wallpapers/WallpaperArea.svelte";

    let wallpapers: Object[] = [];
    
    async function readFolderEntries(path: string){
        wallpapers = []
        let acceptedVideoFiles = ['mp4', '3gp', 'avi', 'webm', 'm4v', 'mov'];

        try { 
            const entries = await readDir(path, {recursive: false});
    
            for (const entry of entries){
                let entryPath = entry.path; 
                let entryExtension = entryPath.split(".").pop() || "";
                if(acceptedVideoFiles.includes(entryExtension)){
                    console.log(`entry path is: ${entryPath}`);
                    wallpapers = [...wallpapers, {
                        path: entryPath || "Undefined",
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
        }catch(error){
            console.error("folder path retrieval failed", error.message);
        }
    } 
</script>

<Window>
    <div class="flex flex-col px-4 w-full h-full">
        <div class="flex flex-row gap-5 px-10 py-6 w-full">
            <PathButton on:click={getFolderPath}/>
            <SearchInput/>
        </div>
        <WallpaperArea wallpapers={wallpapers}/> 
    </div>
</Window>



<style>
    button{
        border-radius: 10px;
        color: #ffffff99;
        display: flex;
        justify-content: center;
        align-items: center;
        background-color: #273857;
        padding: 5px;
    }
</style>