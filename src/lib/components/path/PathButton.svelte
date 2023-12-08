<script lang="ts">

    import { wallpaper_store } from "$lib/middleware/store";
    import { open } from "@tauri-apps/api/dialog";
    import { readDir } from '@tauri-apps/api/fs';

    let acceptedVideoFiles = ['mp4', '3gp', 'avi', 'webm', 'm4v', 'mov'];
 
    const resetWallpaper = () =>{
        $wallpaper_store = [{}];
    }
    
    const readFolderEntries = async (path: string) =>{

        try{
            resetWallpaper();
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
        }catch(error){
            console.error("reading folder entries failed");

        }
    }
    
    const getFolderPath = async (): Promise<void> => {
        return new Promise<void>((resolve, reject) =>   {
            const selectedPath = await open({
                multiple: false,
                title: "Open Wallpaper Folder",
                directory: true,
            });
            await readFolderEntries(String(selectedPath));

            setTimeout(() => {
            // Once the task is completed
            // Resolve if successful or reject if there's an error
                resolve(); // If there is no value to return
            // reject(new Error('Some error occurred')); // If there's an error
            }, 1000);
        });
</script>

    <button tabindex="0" class="readFileContens" id="readFileContents" on:keypress on:click={() => {
            getFolderPath()
                .then(() => {
                    console.log("folders retrieved successfully");
                })
                .catch((error) => {
                    console.error("folder retrieval failed");
                })
        }}>
        
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