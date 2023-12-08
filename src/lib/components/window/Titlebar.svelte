<script>
      import { appWindow } from "@tauri-apps/api/window";
</script>

<div data-tauri-drag-region class="rounded-sm titlebar">
  
    <div role="button" tabindex="0" class="titlebar-button" id="titlebar-minimize" on:keypress on:click={async () => appWindow.minimize()}>
        <img src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize" />
    </div>
    
    <div role="button" tabindex="0" class="h-10 titlebar-button" id="titlebar-maximize" on:keypress on:click={async () => appWindow.toggleMaximize()}>
        <img src="https://api.iconify.design/mdi:window-maximize.svg" alt="maximize" />
    </div>
    
    <div role="button" tabindex="0" class="titlebar-button" id="titlebar-close" on:keypress on:click={async () => appWindow.close()}>
        <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
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