pub static CONTEXT: &'static str = "Status";
pub static ICONS: &'static [&'static str] = &[
    "appointment-missed",        // The icon used when an appointment was missed.
    "appointment-soon",          // The icon used when an appointment will occur soon.
    "audio-volume-high",         // The icon used to indicate high audio volume.
    "audio-volume-low",          // The icon used to indicate low audio volume.
    "audio-volume-medium",       // The icon used to indicate medium audio volume.
    "audio-volume-muted",        // The icon used to indicate the muted state for audio playback.
    "battery-caution",           // The icon used when the battery is below 40%.
    "battery-low",               // The icon used when the battery is below 20%.
    "dialog-error", // The icon used when a dialog is opened to explain an error condition to the user.
    "dialog-information", // The icon used when a dialog is opened to give information to the user that may be pertinent to the requested action.
    "dialog-password", // The icon used when a dialog requesting the authentication credentials for a user is opened.
    "dialog-question", // The icon used when a dialog is opened to ask a simple question of the user.
    "dialog-warning", // The icon used when a dialog is opened to warn the user of impending issues with the requested action.
    "folder-drag-accept", // The icon used for a folder while an object is being dragged onto it, that is of a type that the directory can contain.
    "folder-open", // The icon used for folders, while their contents are being displayed within the same window. This icon would normally be shown in a tree or list view, next to the main view of a folder's contents.
    "folder-visiting", // The icon used for folders, while their contents are being displayed in another window. This icon would typically be used when using multiple windows to navigate the hierarchy, such as in Nautilus's spatial mode.
    "image-loading", // The icon used when another image is being loaded, such as thumnails for larger images in the file manager.
    "image-missing", // The icon used when another image could not be loaded.
    "mail-attachment", // The icon used for an electronic mail that contains attachments.
    "mail-unread",   // The icon used for an electronic mail that is unread.
    "mail-read",     // The icon used for an electronic mail that is read.
    "mail-replied",  // The icon used for an electronic mail that has been replied to.
    "mail-signed",   // The icon used for an electronic mail that contains a signature.
    "mail-signed-verified", // The icon used for an electronic mail that contains a signature which has also been verified by the security system.
    "media-playlist-repeat", // The icon for the repeat mode of a media player.
    "media-playlist-shuffle", // The icon for the shuffle mode of a media player.
    "network-error", // The icon used when an error occurs trying to intialize the network connection of the computing device. This icon should be two computers, one in the background, with the screens of both computers, colored black, and with the theme's style element for errors, overlayed on top of the icon.
    "network-idle", // The icon used when no data is being transmitted or received, while the computing device is connected to a network. This icon should be two computers, one in the background, with the screens of both computers, colored black.
    "network-offline", // The icon used when the computing device is disconnected from the network. This icon should be a computer in the background, with a screen colored black, and the theme's icon element to show that a device is not accessible, in the foreground.
    "network-receive", // The icon used when data is being received, while the computing device is connected to a network. This icon should be two computers, one in the background, with its screen colored green, and the screen of the computer in the foreground, colored black.
    "network-transmit", // The icon used when data is being transmitted, while the computing device is connected to a network. This icon should be two computers, one in the background, with its screen colored black, and the screen of the computer in the foreground, colored green.
    "network-transmit-receive", // The icon used data is being both transmitted and received simultaneously, while the computing device is connected to a network. This icon should be two computers, one in the background, with the screens of both computers, colored green.
    "printer-error", // The icon used when an error occurs while attempting to print. This icon should be the theme's printer device icon, with the theme's style element for errors, overlayed on top of the icon.
    "printer-printing", // The icon used while a print job is successfully being spooled to a printing device. This icon should be the theme's printer device icon, with a document emerging from the printing device.
    "security-high", // The icon used to indicate that the security level of a connection is known to be secure, using strong encryption and a valid certificate.
    "security-medium", // The icon used to indicate that the security level of a connection is presumed to be secure, using strong encryption, and a certificate that could not be automatically verified, but which the user has chosen to trust.
    "security-low", // The icon used to indicate that the security level of a connection is presumed to be insecure, either by using weak encryption, or by using a certificate that the could not be automatically verified, and which the user has not chosent to trust.
    "software-update-available", // The icon used when an update is available for software installed on the computing device, through the system software update program.
    "software-update-urgent", // The icon used when an urgent update is available through the system software update program.
    "sync-error", // The icon used when an error occurs while attempting to synchronize data from the computing device, to another device.
    "sync-synchronizing", // The icon used while data is successfully synchronizing to another device.
    "task-due",           // The icon used when a task is due soon.
    "task-past-due",      // The icon used when a task that was due, has been left incomplete.
    "user-available", // The icon used when a user on a chat network is available to initiate a conversation with.
    "user-away", // The icon used when a user on a chat network is away from their keyboard and the chat program.
    "user-idle", // The icon used when a user on a chat network has not been an active participant in any chats on the network, for an extended period of time.
    "user-offline", // The icon used when a user on a chat network is not available.
    "user-trash-full", // The icon for the user's “Trash” in the desktop's file manager, when there are items in the “Trash” waiting for disposal or recovery.
    "weather-clear",   // The icon used while the weather for a region is “clear skies”.
    "weather-clear-night", // The icon used while the weather for a region is “clear skies” during the night.
    "weather-few-clouds",  // The icon used while the weather for a region is “partly cloudy”.
    "weather-few-clouds-night", // The icon used while the weather for a region is “partly cloudy” during the night.
    "weather-fog",              // The icon used while the weather for a region is “foggy”.
    "weather-overcast",         // The icon used while the weather for a region is “overcast”.
    "weather-severe-alert", // The icon used while a sever weather alert is in effect for a region.
    "weather-showers",      // The icon used while rain showers are occurring in a region.
    "weather-showers-scattered", // The icon used while scattered rain showers are occurring in a region.
    "weather-snow",              // The icon used while snow showers are occurring in a region.
    "weather-storm",             // The icon used while storms are occurring in a region.
];
