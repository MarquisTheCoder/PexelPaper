<script lang="ts">
    import { readDir } from '@tauri-apps/api/fs';
    import { writable } from 'svelte/store';
     $wallpaper_store = [];
     
    async function readFolderEntries(path: string){
        
        let acceptedVideoFiles = ['mp4', '3gp', 'avi', 'webm', 'm4v', 'mov'];

        try{
            // $wallpaper_store = [];
            
            const entries = await readDir(path, {recursive: false});
            for (const entry of entries){
                let entryPath = entry.path;
                let entryExtension = entryPath.split(".").pop() || "";
                if(acceptedVideoFiles.includes(entryExtension)){
                    $wallpaper_store = [...$wallpaper_store, {
                        path: entry.path,
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
                await readFolderEntries(String(selectedPath))
        }catch(err){
            console.error("folder path retrieval failed", err.message);
        }
    }
</script>

<button id="readFileContents">
    <img id="folder-icon" src="img/folder.svg" alt="search folder location"
    class="h-[20px]"/>
</button>

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