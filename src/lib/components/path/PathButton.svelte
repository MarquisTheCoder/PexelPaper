<script>
    import {open} from "@tauri-apps/api/dialog";
    import {invoke} from "@tauri-apps/api/tauri";
    import {current_path, wallpaper_store} from "$lib/middleware/store.ts"
    import { readDir, BaseDirectory } from '@tauri-apps/api/fs';
    
    let acceptableFileTypes = ['mp4', '3gp', 'avi', 'webm', 'm4v', 'mov'];

    const readFolderEntries = async (path) =>{
        const entries = await readDir(path, {recursive: false});

        for (const entry of entries){
            let entryExtension = entry.path.split(".").pop();
            if(acceptableFileTypes.includes(entryExtension)){
                console.log(`Acceptable Video Entry Added: ${entry.path}`);
            }
        }
    }
    
    const getFolderPath = async () =>{
        console.log("reading contents");
        try{
            const selectedPath = await open({
                multiple: false,
                title: "Open Wallpaper Folder",
                directory: true,
            });
            await readFolderEntries(selectedPath);
        }catch(err){
            console.error(err);
        }
    }
</script>

<button tabindex="0" class="getFolderPath" id="getFolderPath" on:keypress on:click={() => getFolderPath()}>
    <img id="folder-icon" src="img/folder.svg" alt="search folder location"/>
</button>

<style>
    #folder-icon{
        filter: invert(100%) opacity(60%);
        height: 20px;
        margin: 2px 10px;
    }
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