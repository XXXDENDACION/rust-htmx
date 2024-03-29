window.htmx.onLoad((content) => {
    const formEl = content.querySelector('.sortable');
    const sortable = new Draggable.Sortable(formEl, {
        draggable: '.sorting-element',
    });
    sortable.on('drag:start', () => console.log('drag:start'));
    sortable.on('drag:move', () => console.log('drag:move'));
    sortable.on('drag:stop', () => {
        console.log('drag:stop')
    });
    console.log("TEST ~!");



});