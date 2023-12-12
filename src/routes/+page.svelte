<script>
	import PathButton from "$lib/components/path/PathButton.svelte";
	import SearchInput from "$lib/components/path/SearchInput.svelte";
	import Window from "$lib/components/window/Window.svelte";
	import "../app.css";
</script>

<Window>
    <div class="flex flex-row w-full">
        <PathButton/>
        <SearchInput/>
    </div>
</Window>

