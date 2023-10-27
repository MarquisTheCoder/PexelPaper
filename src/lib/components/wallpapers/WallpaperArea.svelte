
<div id="wallpaper-area">
    <div id="wallpaper-container">
        <slot/>
    </div>
</div>

<style>
    div{
        scrollbar-width: none;
    }
    #wallpaper-container{
        max-width: 90%;
        min-width: 90%;
        min-height: 100%;

        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        flex-basis: content;
        justify-content: flex-start;

        gap: 25px;
        overflow-y: scroll;
        scrollbar-width: none;
        /* background-color: blue; */
        flex-grow:0;
    }

    #wallpaper-area{
        display: flex;
        flex-direction: row;
        justify-content: center;
        flex-basis: content;
        min-height: 100%;
        min-width: 100%;
        overflow-y: scroll;
        /* background-color: red; */
    }
</style>