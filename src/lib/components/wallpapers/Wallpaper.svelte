

<script lang="ts">
    import Select from "$lib/components/wallpapers/Select.svelte";
    // When using the Tauri API npm package:
    import { invoke } from '@tauri-apps/api/tauri' 

    export let path: string = "";
    
    function path_to_raw_image_data(video_path: string){
        let data = "error"
        invoke('raw_data_to_base64', { wallpaper_video_path: video_path})
        .then((data) => data = data)
        return data 
    }
</script>

<div data-wallpaper_path={path} class="mr-10">
     <p>{path_to_raw_image_data(path)}</p>
    <Select/>
</div   >

<style>
    div{
        position: relative;
        min-width: 250px;
        height: 200px;
        background-color: #1a1e27;
        border: none;
        border-radius: 5px;
        cursor: pointer;
        scrollbar-width: none;
        overflow: hidden;

        -webkit-box-shadow:0px 0px 8px 8px rgba(172, 196, 221, 0.326);
        -moz-box-shadow: 0px 0px 8px 8px rgba(136, 149, 165, 0.084);
        box-shadow: 0px 0px 8px 8px rgba(132, 154, 178, 0.085);
    }
</style>
