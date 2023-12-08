<script>
    import SearchInput from "$lib/components/path/SearchInput.svelte";
    import PathButton from "./PathButton.svelte";
</script>

<div id="path-selection" class=>
    <PathButton/>
    <SearchInput/>
</div>

<style>
     #path-selection{
        height: 80px;
        width: 100vw;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: flex-start;
        padding: 30px 30px;
        gap: 10px;
        background: linear-gradient(to bottom, #0e0f15, transparent);
        backdrop-filter: blur(10px);
    }
</style>
