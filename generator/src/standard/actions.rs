pub static CONTEXT: &'static str = "Actions";
pub static ICONS: &'static [&'static str] = &[
    "address-book-new", // The icon used for the action to create a new address book.
    "application-exit", // The icon used for exiting an application. Typically this is seen in the application's menus as File->Quit.
    "appointment-new", // The icon used for the action to create a new appointment in a calendaring application.
    "call-start", // The icon used for initiating or accepting a call. Should be similar to the standard cellular call pickup icon, a green handset with ear and mouth pieces facing upward.
    "call-stop", // The icon used for stopping a current call. Should be similar to the standard cellular call hangup icon, a red handset with ear and mouth pieces facing downward.
    "contact-new", // The icon used for the action to create a new contact in an address book application.
    "document-new", // The icon used for the action to create a new document.
    "document-open", // The icon used for the action to open a document.
    "document-open-recent", // The icon used for the action to open a document that was recently opened.
    "document-page-setup",  // The icon for the page setup action of a document editor.
    "document-print",       // The icon for the print action of an application.
    "document-print-preview", // The icon for the print preview action of an application.
    "document-properties", // The icon for the action to view the properties of a document in an application.
    "document-revert", // The icon for the action of reverting to a previous version of a document.
    "document-save", // The icon for the save action. Should be an arrow pointing down and toward a hard disk.
    "document-save-as", // The icon for the save as action.
    "document-send", // The icon for the send action. Should be an arrow pointing up and away from a hard disk.
    "edit-clear",    // The icon for the clear action.
    "edit-copy",     // The icon for the copy action.
    "edit-cut",      // The icon for the cut action.
    "edit-delete",   // The icon for the delete action.
    "edit-find",     // The icon for the find action.
    "edit-find-replace", // The icon for the find and replace action.
    "edit-paste",    // The icon for the paste action.
    "edit-redo",     // The icon for the redo action.
    "edit-select-all", // The icon for the select all action.
    "edit-undo",     // The icon for the undo action.
    "folder-new",    // The icon for creating a new folder.
    "format-indent-less", // The icon for the decrease indent formatting action.
    "format-indent-more", // The icon for the increase indent formatting action.
    "format-justify-center", // The icon for the center justification formatting action.
    "format-justify-fill", // The icon for the fill justification formatting action.
    "format-justify-left", // The icon for the left justification formatting action.
    "format-justify-right", // The icon for the right justification action.
    "format-text-direction-ltr", // The icon for the left-to-right text formatting action.
    "format-text-direction-rtl", // The icon for the right-to-left formatting action.
    "format-text-bold", // The icon for the bold text formatting action.
    "format-text-italic", // The icon for the italic text formatting action.
    "format-text-underline", // The icon for the underlined text formatting action.
    "format-text-strikethrough", // The icon for the strikethrough text formatting action.
    "go-bottom",     // The icon for the go to bottom of a list action.
    "go-down",       // The icon for the go down in a list action.
    "go-first",      // The icon for the go to the first item in a list action.
    "go-home",       // The icon for the go to home location action.
    "go-jump",       // The icon for the jump to action.
    "go-last",       // The icon for the go to the last item in a list action.
    "go-next",       // The icon for the go to the next item in a list action.
    "go-previous",   // The icon for the go to the previous item in a list action.
    "go-top",        // The icon for the go to the top of a list action.
    "go-up",         // The icon for the go up in a list action.
    "help-about",    // The icon for the About item in the Help menu.
    "help-contents", // The icon for Contents item in the Help menu.
    "help-faq",      // The icon for the FAQ item in the Help menu.
    "insert-image",  // The icon for the insert image action of an application.
    "insert-link",   // The icon for the insert link action of an application.
    "insert-object", // The icon for the insert object action of an application.
    "insert-text",   // The icon for the insert text action of an application.
    "list-add",      // The icon for the add to list action.
    "list-remove",   // The icon for the remove from list action.
    "mail-forward",  // The icon for the forward action of an electronic mail application.
    "mail-mark-important", // The icon for the mark as important action of an electronic mail application.
    "mail-mark-junk", // The icon for the mark as junk action of an electronic mail application.
    "mail-mark-notjunk", // The icon for the mark as not junk action of an electronic mail application.
    "mail-mark-read",    // The icon for the mark as read action of an electronic mail application.
    "mail-mark-unread", // The icon for the mark as unread action of an electronic mail application.
    "mail-message-new", // The icon for the compose new mail action of an electronic mail application.
    "mail-reply-all",   // The icon for the reply to all action of an electronic mail application.
    "mail-reply-sender", // The icon for the reply to sender action of an electronic mail application.
    "mail-send",         // The icon for the send action of an electronic mail application.
    "mail-send-receive", // The icon for the send and receive action of an electronic mail application.
    "media-eject",       // The icon for the eject action of a media player or file manager.
    "media-playback-pause", // The icon for the pause action of a media player.
    "media-playback-start", // The icon for the start playback action of a media player.
    "media-playback-stop", // The icon for the stop action of a media player.
    "media-record",      // The icon for the record action of a media application.
    "media-seek-backward", // The icon for the seek backward action of a media player.
    "media-seek-forward", // The icon for the seek forward action of a media player.
    "media-skip-backward", // The icon for the skip backward action of a media player.
    "media-skip-forward", // The icon for the skip forward action of a media player.
    "object-flip-horizontal", // The icon for the action to flip an object horizontally.
    "object-flip-vertical", // The icon for the action to flip an object vertically.
    "object-rotate-left", // The icon for the rotate left action performed on an object.
    "object-rotate-right", // The icon for the rotate rigt action performed on an object.
    "process-stop", // The icon used for the \u201cStop\u201d action in applications with actions that may take a while to process, such as web page loading in a browser.
    "system-lock-screen", // The icon used for the \u201cLock Screen\u201d item in the desktop's panel application.
    "system-log-out", // The icon used for the \u201cLog Out\u201d item in the desktop's panel application.
    "system-run", // The icon used for the \u201cRun Application...\u201d item in the desktop's panel application.
    "system-search", // The icon used for the \u201cSearch\u201d item in the desktop's panel application.
    "system-reboot", // The icon used for the \u201cReboot\u201d item in the desktop's panel application.
    "system-shutdown", // The icon used for the \u201cShutdown\u201d item in the desktop's panel application.
    "tools-check-spelling", // The icon used for the \u201cCheck Spelling\u201d item in the application's \u201cTools\u201d menu.
    "view-fullscreen", // The icon used for the \u201cFullscreen\u201d item in the application's \u201cView\u201d menu.
    "view-refresh", // The icon used for the \u201cRefresh\u201d item in the application's \u201cView\u201d menu.
    "view-restore", // The icon used by an application for leaving the fullscreen view, and returning to a normal windowed view.
    "view-sort-ascending", // The icon used for the \u201cSort Ascending\u201d item in the application's \u201cView\u201d menu, or in a button for changing the sort method for a list.
    "view-sort-descending", // The icon used for the \u201cSort Descending\u201d item in the application's \u201cView\u201d menu, or in a button for changing the sort method for a list.
    "window-close", // The icon used for the \u201cClose Window\u201d item in the application's \u201cWindows\u201d menu.
    "window-new", // The icon used for the \u201cNew Window\u201d item in the application's \u201cWindows\u201d menu.
    "zoom-fit-best", // The icon used for the \u201cBest Fit\u201d item in the application's \u201cView\u201d menu.
    "zoom-in", // The icon used for the \u201cZoom in\u201d item in the application's \u201cView\u201d menu.
    "zoom-original", // The icon used for the \u201cOriginal Size\u201d item in the application's \u201cView\u201d menu.
    "zoom-out", // The icon used for the \u201cZoom Out\u201d item in the application's \u201cView\u201d menu.
];
