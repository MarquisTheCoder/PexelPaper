
<div id="wallpaper-area">
    <slot/>
</div>

<style>
    div{
        scrollbar-width: none;
    }

    #wallpaper-area{
        padding: 30px 30px;
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: center;
        gap: 25px;
        height: 100vh;
        max-height: 100vh;
        width: 95%;
        overflow-y: scroll;
        scrollbar-width: none;
    }
</style>