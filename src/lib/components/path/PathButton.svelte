<script>
    import {open} from "@tauri-apps/api/dialog";
    import {invoke} from "@tauri-apps/api/tauri";

    async function log(inputString) {
        await invoke('log', {inputString});
    }
    const readFileContents = async () =>{
        try{
            const selectedPath = await open({
                multiple: false,
                title: "Open Wallpaper Folder",
                directory: true,
            });
            log(selectedPath);
        }catch(err){
            console.error(err);
        }
    }
</script>

<button tabindex="0" class="readFileContens" id="readFileContents" on:keypress on:click={() => readFileContents()}>
    Path
</button>

<style>
    button{
        border-radius: 10px;
        color: #ffffff99;
        padding: 0 20px 0 20px;
        background-color: #4375C9;
        height: 40px;
    }
</style>