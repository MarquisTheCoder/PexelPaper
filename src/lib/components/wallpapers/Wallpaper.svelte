
<script>
    import Select from "$lib/components/wallpapers/Select.svelte";
    export let imagePath = "";
</script>

<div>
    <img src="{imagePath}"/>
    <Select/>
</div>

<style>
    img{
        position: absolute;
        top:0;
        left:0;
        width: 100%;
        height: 100%;
    }
    div{
        position: relative;
        width: 240px;
        height: 175px;
        background-color: #1a1e27;
        border-radius: 5px;
        cursor: pointer;
        scrollbar-width: none;
    }
</style>
