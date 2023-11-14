<script>
    import Select from "$lib/components/wallpapers/Select.svelte";
    export let src = "";
    export let path = "";
</script>

<div data-wallpaper_path={path}>
    <img {src} alt="{path}">
    <Select/>
</div>

<style>
    img{
        position: absolute;
        top:0;
        left:0;
        width: 100%;
        height: 100%;
        border: none;
    }

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

        -webkit-box-shadow:0px 0px 28px 8px rgba(73,134,199,0.17);
        -moz-box-shadow: 0px 0px 28px 8px rgba(73,134,199,0.17);
        box-shadow: 0px 0px 28px 8px rgba(73,134,199,0.17);
    }
</style>
