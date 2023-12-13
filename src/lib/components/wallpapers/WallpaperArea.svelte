
<script lang="ts">
    import Wallpaper from "./Wallpaper.svelte";
    export let wallpapers: Object[] = [{}];
</script>

<div id="wallpaper-area" class="flex overflow-y-scroll flex-row justify-center min-w-full min-h-full">
    <div id="wallpaper-container" class="overflow-y-scroll p-5 min-w-full min-h-full">

        {#each wallpapers as wallpaper}
            <svelte:component this={Wallpaper} path={wallpaper.path}/>
        {/each}
    </div>
</div>

<style>

    div{
        scrollbar-width: none;
    }

    #wallpaper-container{
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        scrollbar-width: none;
    }

    #wallpaper-area{
        flex-basis: content;
        /* background-color: red; */
    }
</style>