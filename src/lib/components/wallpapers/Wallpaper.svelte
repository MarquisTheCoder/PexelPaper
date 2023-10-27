
<script>
    import Select from "$lib/components/wallpapers/Select.svelte";
    export let src = "";
</script>

<div>
    <img {src}>
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
        min-width: 30%;
        height: 200px;
        background-color: #1a1e27;
        border: none;
        border-radius: 5px;
        cursor: pointer;
        scrollbar-width: none;
        overflow: hidden;
    }
</style>
