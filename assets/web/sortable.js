let listener;
function beforeSwapEvent(sortable) {
    sortable.off("drag:start");
    sortable.off("drag:move");
    sortable.off("drag:stop");
    sortable.destroy();
}

window.htmx.onLoad((content) => {
    document.body.removeEventListener('htmx:beforeSwap', listener);

    const formEl = content.querySelector('.sortable');
    let sortable = new Draggable.Sortable(formEl || content, {
        draggable: '.sorting-element',
    });
    sortable.on('drag:start', () => console.log('drag:start'));
    sortable.on('drag:move', () => console.log('drag:move'));
    sortable.on('drag:stop', () => {
        console.log('drag:stop')
    });

    listener = () => beforeSwapEvent(sortable);
    document.body.addEventListener('htmx:beforeSwap', listener)
})