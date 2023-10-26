

<div id="wallpaper-area">
    <slot/>
</div>

<style> 
    #wallpaper-area{
        padding: 30px 30px;
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        gap: 25px;
        height: 100vh;
        max-height: 100vh;
        width: 70%;
        overflow-y: scroll;
    }
</style>