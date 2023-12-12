<script>
      import { appWindow } from "@tauri-apps/api/window";
</script>

<div data-tauri-drag-region class="rounded-sm titlebar">
  
    <div role="button" tabindex="0" class="titlebar-button" id="titlebar-minimize" on:keypress on:click={async () => appWindow.minimize()}>
      <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="#fff" d="M20 14H4v-4h16"/></svg>
        <!-- <img src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize" /> -->
    </div>
    
    <div role="button" tabindex="0" class="h-10 titlebar-button" id="titlebar-maximize" on:keypress on:click={async () => appWindow.toggleMaximize()}>
      <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="#fff" d="M4 4h16v16H4zm2 4v10h12V8z"/></svg> 
    </div>
    
    <div role="button" tabindex="0" class="titlebar-button" id="titlebar-close" on:keypress on:click={async () => appWindow.close()}>
      <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="#fff" d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12z"/></svg>
    </div>
    
</div>

<style>
  div{
    z-index: 10;
  }

  img{
    filter: invert(100%);
    opacity: 0.6;
  }

  .titlebar {
    height: 30px;
    width: 100vw;
    background: transparent;
    user-select: none;
    display: flex;
    justify-content: flex-end;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    display: flex;
    justify-content: flex-start;
    border: 1px;
    border-color: white;
    z-index: 2;
    padding-left: 2px;
  }

.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
  z-index: 2;
  border-radius: 100%;
}

.titlebar-button:hover {
  background: #14223c;
}

</style>