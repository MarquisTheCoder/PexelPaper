
<script>
    import { wallpaper_store } from "$lib/middleware/store";
    import Wallpaper from "./Wallpaper.svelte";
</script>

<div id="wallpaper-area">
    <div id="wallpaper-container">
        {#each $wallpaper_store as wallpaper}
            <svelte:component this={Wallpaper} src={""} path={wallpaper.path}/>
        {/each}
    </div>
</div>

<style>

    div{
        scrollbar-width: none;
    }

    #wallpaper-container{
        min-width: 100%;
        min-height: 100%;

        display: grid;
        grid-template-columns: repeat(3, 1fr);

        gap: 50px;
        overflow-y: scroll;
        scrollbar-width: none;
        /* background-color: blue; */

        padding: 50px 50px;
    }

    #wallpaper-area{
        display: flex;
        flex-direction: row;
        justify-content: center;
        flex-basis: content;
        min-height: 100%;
        min-width: 100%;
        overflow-y: scroll;
        padding-top: 25px;
        /* background-color: red; */
    }
</style>