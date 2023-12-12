
<script lang="ts">
    import Wallpaper from "./Wallpaper.svelte";
    export let wallpapers: Object[] = [];
</script>

<div id="wallpaper-area" class="flex overflow-y-scroll flex-row justify-center p-6 min-w-full min-h-full">
    <div id="wallpaper-container" class="min-w-full min-h-full gap-[30px] overflow-y-scroll p-5">
        {#each wallpapers as wallpaper}
            <svelte:component this={Wallpaper} src={""} path={wallpaper.path}/>
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