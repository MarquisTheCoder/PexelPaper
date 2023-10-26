<script>
    import PathButton from "$lib/components/path/PathButton.svelte"
    import SearchInput from "$lib/components/path/SearchInput.svelte"
</script>

<div id="path-selection">
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
        justify-content: start;
        padding: 30px 30px 0 30px;
        gap: 10px;
    }
</style>
