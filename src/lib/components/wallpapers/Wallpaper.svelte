<script lang="ts">
    import Select from "$lib/components/wallpapers/Select.svelte";
    export let path: string = "";
</script>

<div data-wallpaper_path={path}>
    <!-- <img {src} alt="{path}"> -->
    <p class="text-2xl text-white">{path}</p>
    <Select/>
</div>

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
