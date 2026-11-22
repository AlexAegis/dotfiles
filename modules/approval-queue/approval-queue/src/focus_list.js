// Brings the window of the list to the current desktop and gives it the focus.
// A window that opens later gets the same, one time.
// Some KWin versions add a number to the caption of a second window with the
// same title.
var listCaption = /^approval-queue( <[0-9]+>)?$/;
var isShown = false;

function isList(window) {
    return window.normalWindow && listCaption.test(window.caption);
}

function show(window) {
    if (isShown || !isList(window)) {
        return;
    }
    isShown = true;
    window.minimized = false;
    if (!window.onAllDesktops) {
        window.desktops = [workspace.currentDesktop];
    }
    workspace.activeWindow = window;
}

workspace.stackingOrder.forEach(show);
workspace.windowAdded.connect(function (window) {
    show(window);
    window.captionChanged.connect(function () {
        show(window);
    });
});
