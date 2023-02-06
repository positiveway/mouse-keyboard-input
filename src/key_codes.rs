pub const INPUT_PROP_POINTER:        i32 = 0x00; /* needs a pointer */
pub const INPUT_PROP_DIRECT:         i32 = 0x01; /* direct input devices */
pub const INPUT_PROP_BUTTONPAD:      i32 = 0x02; /* has button: i32 = s under pad */
pub const INPUT_PROP_SEMI_MT:        i32 = 0x03; /* touch rectangle only */
pub const INPUT_PROP_TOPBUTTONPAD:   i32 = 0x04; /* softbuttons at top of pad */
pub const INPUT_PROP_POINTING_STICK: i32 = 0x05; /* is a pointing stick */
pub const INPUT_PROP_ACCELEROMETER:  i32 = 0x06; /* has accelerometer */

pub const INPUT_PROP_MAX: i32 = 0x1f;
pub const INPUT_PROP_CNT: i32 = INPUT_PROP_MAX + 1;

/*
 * Event types
 */

pub const EV_SYN:       i32 = 0x00;
pub const EV_KEY:       i32 = 0x01;
pub const EV_REL:       i32 = 0x02;
pub const EV_ABS:       i32 = 0x03;
pub const EV_MSC:       i32 = 0x04;
pub const EV_SW:        i32 = 0x05;
pub const EV_LED:       i32 = 0x11;
pub const EV_SND:       i32 = 0x12;
pub const EV_REP:       i32 = 0x14;
pub const EV_FF:        i32 = 0x15;
pub const EV_PWR:       i32 = 0x16;
pub const EV_FF_STATUS: i32 = 0x17;
pub const EV_MAX:       i32 = 0x1f;
pub const EV_CNT:       i32 = EV_MAX + 1;

/*
 * Synchronization events.
 */

pub const SYN_REPORT:    i32 = 0;
pub const SYN_CONFIG:    i32 = 1;
pub const SYN_MT_REPORT: i32 = 2;
pub const SYN_DROPPED:   i32 = 3;
pub const SYN_MAX:       i32 = 0xf;
pub const SYN_CNT:       i32 = SYN_MAX + 1;

/*
 * Keys and buttons
 *
 * Most of the keys/buttons are modeled after USB HUT 1.12 (see
 * http://www.usb.org/developers/hidpage).
 * Abbreviations in the comments:
 * AC - Application Control
 * AL - Application Launch Button
 * SC - System Control
 */

pub const KEY_RESERVED:   i32 = 0;
pub const KEY_ESC:        i32 = 1;
pub const KEY_1:          i32 = 2;
pub const KEY_2:          i32 = 3;
pub const KEY_3:          i32 = 4;
pub const KEY_4:          i32 = 5;
pub const KEY_5:          i32 = 6;
pub const KEY_6:          i32 = 7;
pub const KEY_7:          i32 = 8;
pub const KEY_8:          i32 = 9;
pub const KEY_9:          i32 = 10;
pub const KEY_10:         i32 = 11;
pub const KEY_MINUS:      i32 = 12;
pub const KEY_EQUAL:      i32 = 13;
pub const KEY_BACKSPACE:  i32 = 14;
pub const KEY_TAB:        i32 = 15;
pub const KEY_Q:          i32 = 16;
pub const KEY_W:          i32 = 17;
pub const KEY_E:          i32 = 18;
pub const KEY_R:          i32 = 19;
pub const KEY_T:          i32 = 20;
pub const KEY_Y:          i32 = 21;
pub const KEY_U:          i32 = 22;
pub const KEY_I:          i32 = 23;
pub const KEY_O:          i32 = 24;
pub const KEY_P:          i32 = 25;
pub const KEY_LEFTBRACE:  i32 = 26;
pub const KEY_RIGHTBRACE: i32 = 27;
pub const KEY_ENTER:      i32 = 28;
pub const KEY_LEFTCTRL:   i32 = 29;
pub const KEY_A:          i32 = 30;
pub const KEY_S:          i32 = 31;
pub const KEY_D:          i32 = 32;
pub const KEY_F:          i32 = 33;
pub const KEY_G:          i32 = 34;
pub const KEY_H:          i32 = 35;
pub const KEY_J:          i32 = 36;
pub const KEY_K:          i32 = 37;
pub const KEY_L:          i32 = 38;
pub const KEY_SEMICOLON:  i32 = 39;
pub const KEY_APOSTROPHE: i32 = 40;
pub const KEY_GRAVE:      i32 = 41;
pub const KEY_LEFTSHIFT:  i32 = 42;
pub const KEY_BACKSLASH:  i32 = 43;
pub const KEY_Z:          i32 = 44;
pub const KEY_X:          i32 = 45;
pub const KEY_C:          i32 = 46;
pub const KEY_V:          i32 = 47;
pub const KEY_B:          i32 = 48;
pub const KEY_N:          i32 = 49;
pub const KEY_M:          i32 = 50;
pub const KEY_COMMA:      i32 = 51;
pub const KEY_DOT:        i32 = 52;
pub const KEY_SLASH:      i32 = 53;
pub const KEY_RIGHTSHIFT: i32 = 54;
pub const KEY_KPASTERISK: i32 = 55;
pub const KEY_LEFTALT:    i32 = 56;
pub const KEY_SPACE:      i32 = 57;
pub const KEY_CAPSLOCK:   i32 = 58;
pub const KEY_F1:         i32 = 59;
pub const KEY_F2:         i32 = 60;
pub const KEY_F3:         i32 = 61;
pub const KEY_F4:         i32 = 62;
pub const KEY_F5:         i32 = 63;
pub const KEY_F6:         i32 = 64;
pub const KEY_F7:         i32 = 65;
pub const KEY_F8:         i32 = 66;
pub const KEY_F9:         i32 = 67;
pub const KEY_F10:        i32 = 68;
pub const KEY_NUMLOCK:    i32 = 69;
pub const KEY_SCROLLLOCK: i32 = 70;
pub const KEY_KP7:        i32 = 71;
pub const KEY_KP8:        i32 = 72;
pub const KEY_KP9:        i32 = 73;
pub const KEY_KPMINUS:    i32 = 74;
pub const KEY_KP4:        i32 = 75;
pub const KEY_KP5:        i32 = 76;
pub const KEY_KP6:        i32 = 77;
pub const KEY_KPPLUS:     i32 = 78;
pub const KEY_KP1:        i32 = 79;
pub const KEY_KP2:        i32 = 80;
pub const KEY_KP3:        i32 = 81;
pub const KEY_KP0:        i32 = 82;
pub const KEY_KPDOT:      i32 = 83;

pub const KEY_ZENKAKUHANKAKU:   i32 = 85;
pub const KEY_102ND:            i32 = 86;
pub const KEY_F11:              i32 = 87;
pub const KEY_F12:              i32 = 88;
pub const KEY_RO:               i32 = 89;
pub const KEY_KATAKANA:         i32 = 90;
pub const KEY_HIRAGANA:         i32 = 91;
pub const KEY_HENKAN:           i32 = 92;
pub const KEY_KATAKANAHIRAGANA: i32 = 93;
pub const KEY_MUHENKAN:         i32 = 94;
pub const KEY_KPJPCOMMA:        i32 = 95;
pub const KEY_KPENTER:          i32 = 96;
pub const KEY_RIGHTCTRL:        i32 = 97;
pub const KEY_KPSLASH:          i32 = 98;
pub const KEY_SYSRQ:            i32 = 99;
pub const KEY_RIGHTALT:         i32 = 100;
pub const KEY_LINEFEED:         i32 = 101;
pub const KEY_HOME:             i32 = 102;
pub const KEY_UP:               i32 = 103;
pub const KEY_PAGEUP:           i32 = 104;
pub const KEY_LEFT:             i32 = 105;
pub const KEY_RIGHT:            i32 = 106;
pub const KEY_END:              i32 = 107;
pub const KEY_DOWN:             i32 = 108;
pub const KEY_PAGEDOWN:         i32 = 109;
pub const KEY_INSERT:           i32 = 110;
pub const KEY_DELETE:           i32 = 111;
pub const KEY_MACRO:            i32 = 112;
pub const KEY_MUTE:             i32 = 113;
pub const KEY_VOLUMEDOWN:       i32 = 114;
pub const KEY_VOLUMEUP:         i32 = 115;
pub const KEY_POWER:            i32 = 116; /* SC System Power Down */
pub const KEY_KPEQUAL:          i32 = 117;
pub const KEY_KPPLUSMINUS:      i32 = 118;
pub const KEY_PAUSE:            i32 = 119;
pub const KEY_SCALE:            i32 = 120; /* AL Compiz Scale : i32 = Expose */

pub const KEY_KPCOMMA:   i32 = 121;
pub const KEY_HANGEUL:   i32 = 122;
pub const KEY_HANGUEL:   i32 = KEY_HANGEUL;
pub const KEY_HANJA:     i32 = 123;
pub const KEY_YEN:       i32 = 124;
pub const KEY_LEFTMETA:  i32 = 125;
pub const KEY_RIGHTMETA: i32 = 126;
pub const KEY_COMPOSE:   i32 = 127;

pub const KEY_STOP:           i32 = 128; /* AC Stop */
pub const KEY_AGAIN:          i32 = 129;
pub const KEY_PROPS:          i32 = 130; /* AC Properties */
pub const KEY_UNDO:           i32 = 131; /* AC Undo */
pub const KEY_FRONT:          i32 = 132;
pub const KEY_COPY:           i32 = 133; /* AC Copy */
pub const KEY_OPEN:           i32 = 134; /* AC Open */
pub const KEY_PASTE:          i32 = 135; /* AC Paste */
pub const KEY_FIND:           i32 = 136; /* AC Search */
pub const KEY_CUT:            i32 = 137; /* AC Cut */
pub const KEY_HELP:           i32 = 138; /* AL Integrated Help Center */
pub const KEY_MENU:           i32 = 139; /* Menu : i32 = show menu */
pub const KEY_CALC:           i32 = 140; /* AL Calculator */
pub const KEY_SETUP:          i32 = 141;
pub const KEY_SLEEP:          i32 = 142; /* SC System Sleep */
pub const KEY_WAKEUP:         i32 = 143; /* System Wake Up */
pub const KEY_FILE:           i32 = 144; /* AL Local Machine Browser */
pub const KEY_SENDFILE:       i32 = 145;
pub const KEY_DELETEFILE:     i32 = 146;
pub const KEY_XFER:           i32 = 147;
pub const KEY_PROG1:          i32 = 148;
pub const KEY_PROG2:          i32 = 149;
pub const KEY_WWW:            i32 = 150; /* AL Internet Browser */
pub const KEY_MSDOS:          i32 = 151;
pub const KEY_COFFEE:         i32 = 152; /* AL Terminal Lock/Screensaver */
pub const KEY_SCREENLOCK:     i32 = KEY_COFFEE;
pub const KEY_ROTATE_DISPLAY: i32 = 153; /* Display orientation for e.g. tablets */
pub const KEY_DIRECTION:      i32 = KEY_ROTATE_DISPLAY;
pub const KEY_CYCLEWINDOWS:   i32 = 154;
pub const KEY_MAIL:           i32 = 155;
pub const KEY_BOOKMARKS:      i32 = 156; /* AC Bookmarks */
pub const KEY_COMPUTER:       i32 = 157;
pub const KEY_BACK:           i32 = 158; /* AC Back */
pub const KEY_FORWARD:        i32 = 159; /* AC Forward */
pub const KEY_CLOSECD:        i32 = 160;
pub const KEY_EJECTCD:        i32 = 161;
pub const KEY_EJECTCLOSECD:   i32 = 162;
pub const KEY_NEXTSONG:       i32 = 163;
pub const KEY_PLAYPAUSE:      i32 = 164;
pub const KEY_PREVIOUSSONG:   i32 = 165;
pub const KEY_STOPCD:         i32 = 166;
pub const KEY_RECORD:         i32 = 167;
pub const KEY_REWIND:         i32 = 168;
pub const KEY_PHONE:          i32 = 169; /* Media Select Telephone */
pub const KEY_ISO:            i32 = 170;
pub const KEY_CONFIG:         i32 = 171; /* AL Consumer Control Configuration */
pub const KEY_HOMEPAGE:       i32 = 172; /* AC Home */
pub const KEY_REFRESH:        i32 = 173; /* AC Refresh */
pub const KEY_EXIT:           i32 = 174; /* AC Exit */
pub const KEY_MOVE:           i32 = 175;
pub const KEY_EDIT:           i32 = 176;
pub const KEY_SCROLLUP:       i32 = 177;
pub const KEY_SCROLLDOWN:     i32 = 178;
pub const KEY_KPLEFTPAREN:    i32 = 179;
pub const KEY_KPRIGHTPAREN:   i32 = 180;
pub const KEY_NEW:            i32 = 181; /* AC New */
pub const KEY_REDO:           i32 = 182; /* AC Redo/Repeat */

pub const KEY_F13: i32 = 183;
pub const KEY_F14: i32 = 184;
pub const KEY_F15: i32 = 185;
pub const KEY_F16: i32 = 186;
pub const KEY_F17: i32 = 187;
pub const KEY_F18: i32 = 188;
pub const KEY_F19: i32 = 189;
pub const KEY_F20: i32 = 190;
pub const KEY_F21: i32 = 191;
pub const KEY_F22: i32 = 192;
pub const KEY_F23: i32 = 193;
pub const KEY_F24: i32 = 194;

pub const KEY_PLAYCD:         i32 = 200;
pub const KEY_PAUSECD:        i32 = 201;
pub const KEY_PROG3:          i32 = 202;
pub const KEY_PROG4:          i32 = 203;
pub const KEY_DASHBOARD:      i32 = 204; /* AL Dashboard */
pub const KEY_SUSPEND:        i32 = 205;
pub const KEY_CLOSE:          i32 = 206; /* AC Close */
pub const KEY_PLAY:           i32 = 207;
pub const KEY_FASTFORWARD:    i32 = 208;
pub const KEY_BASSBOOST:      i32 = 209;
pub const KEY_PRINT:          i32 = 210; /* AC Print */
pub const KEY_HP:             i32 = 211;
pub const KEY_CAMERA:         i32 = 212;
pub const KEY_SOUND:          i32 = 213;
pub const KEY_QUESTION:       i32 = 214;
pub const KEY_EMAIL:          i32 = 215;
pub const KEY_CHAT:           i32 = 216;
pub const KEY_SEARCH:         i32 = 217;
pub const KEY_CONNECT:        i32 = 218;
pub const KEY_FINANCE:        i32 = 219; /* AL Checkbook/Finance */
pub const KEY_SPORT:          i32 = 220;
pub const KEY_SHOP:           i32 = 221;
pub const KEY_ALTERASE:       i32 = 222;
pub const KEY_CANCEL:         i32 = 223; /* AC Cancel */
pub const KEY_BRIGHTNESSDOWN: i32 = 224;
pub const KEY_BRIGHTNESSUP:   i32 = 225;
pub const KEY_MEDIA:          i32 = 226;

pub const KEY_SWITCHVIDEOMODE: i32 = 227; /* Cycle between available video outputs (Monitor/LCD/TV-out/etc) */
pub const KEY_KBDILLUMTOGGLE:  i32 = 228;
pub const KEY_KBDILLUMDOWN:    i32 = 229;
pub const KEY_KBDILLUMUP:      i32 = 230;

pub const KEY_SEND:        i32 = 231; /* AC Send */
pub const KEY_REPLY:       i32 = 232; /* AC Reply */
pub const KEY_FORWARDMAIL: i32 = 233; /* AC Forward Msg */
pub const KEY_SAVE:        i32 = 234; /* AC Save */
pub const KEY_DOCUMENTS:   i32 = 235;

pub const KEY_BATTERY: i32 = 236;

pub const KEY_BLUETOOTH: i32 = 237;
pub const KEY_WLAN:      i32 = 238;
pub const KEY_UWB:       i32 = 239;

pub const KEY_UNKNOWN: i32 = 240;

pub const KEY_VIDEO_NEXT:       i32 = 241; /* drive next video source */
pub const KEY_VIDEO_PREV:       i32 = 242; /* drive previous video source */
pub const KEY_BRIGHTNESS_CYCLE: i32 = 243; /* brightness up, after max is min */
pub const KEY_BRIGHTNESS_AUTO:  i32 = 244; /* Set Auto Brightness: manual brightness control is off, rely on ambient */
pub const KEY_BRIGHTNESS_ZERO:  i32 = KEY_BRIGHTNESS_AUTO;
pub const KEY_DISPLAY_OFF:      i32 = 245; /* display device to off state */

pub const KEY_WWAN:   i32 = 246; /* Wireless WAN : i32 = LTE, UMTS, GSM, etc. */
pub const KEY_WIMAX:  i32 = KEY_WWAN;
pub const KEY_RFKILL: i32 = 247; /* Key that controls all radios */

pub const KEY_MICMUTE: i32 = 248; /* Mute / unmute the microphone */

/* Code 255 is reserved for special needs of AT keyboard driver */

pub const BTN_MISC: i32 = 0x100;
pub const BTN_0:    i32 = 0x100;
pub const BTN_1:    i32 = 0x101;
pub const BTN_2:    i32 = 0x102;
pub const BTN_3:    i32 = 0x103;
pub const BTN_4:    i32 = 0x104;
pub const BTN_5:    i32 = 0x105;
pub const BTN_6:    i32 = 0x106;
pub const BTN_7:    i32 = 0x107;
pub const BTN_8:    i32 = 0x108;
pub const BTN_9:    i32 = 0x109;

pub const BTN_MOUSE:   i32 = 0x110;
pub const BTN_LEFT:    i32 = 0x110;
pub const BTN_RIGHT:   i32 = 0x111;
pub const BTN_MIDDLE:  i32 = 0x112;
pub const BTN_SIDE:    i32 = 0x113;
pub const BTN_EXTRA:   i32 = 0x114;
pub const BTN_FORWARD: i32 = 0x115;
pub const BTN_BACK:    i32 = 0x116;
pub const BTN_TASK:    i32 = 0x117;

pub const BTN_JOYSTICK: i32 = 0x120;
pub const BTN_TRIGGER:  i32 = 0x120;
pub const BTN_THUMB:    i32 = 0x121;
pub const BTN_THUMB2:   i32 = 0x122;
pub const BTN_TOP:      i32 = 0x123;
pub const BTN_TOP2:     i32 = 0x124;
pub const BTN_PINKIE:   i32 = 0x125;
pub const BTN_BASE:     i32 = 0x126;
pub const BTN_BASE2:    i32 = 0x127;
pub const BTN_BASE3:    i32 = 0x128;
pub const BTN_BASE4:    i32 = 0x129;
pub const BTN_BASE5:    i32 = 0x12a;
pub const BTN_BASE6:    i32 = 0x12b;
pub const BTN_DEAD:     i32 = 0x12f;

pub const BTN_GAMEPAD: i32 = 0x130;
pub const BTN_SOUTH:   i32 = 0x130;
pub const BTN_A:       i32 = BTN_SOUTH;
pub const BTN_EAST:    i32 = 0x131;
pub const BTN_B:       i32 = BTN_EAST;
pub const BTN_C:       i32 = 0x132;
pub const BTN_NORTH:   i32 = 0x133;
pub const BTN_X:       i32 = BTN_NORTH;
pub const BTN_WEST:    i32 = 0x134;
pub const BTN_Y:       i32 = BTN_WEST;
pub const BTN_Z:       i32 = 0x135;
pub const BTN_TL:      i32 = 0x136;
pub const BTN_TR:      i32 = 0x137;
pub const BTN_TL2:     i32 = 0x138;
pub const BTN_TR2:     i32 = 0x139;
pub const BTN_SELECT:  i32 = 0x13a;
pub const BTN_START:   i32 = 0x13b;
pub const BTN_MODE:    i32 = 0x13c;
pub const BTN_THUMBL:  i32 = 0x13d;
pub const BTN_THUMBR:  i32 = 0x13e;

pub const BTN_DIGI:           i32 = 0x140;
pub const BTN_TOOL_PEN:       i32 = 0x140;
pub const BTN_TOOL_RUBBER:    i32 = 0x141;
pub const BTN_TOOL_BRUSH:     i32 = 0x142;
pub const BTN_TOOL_PENCIL:    i32 = 0x143;
pub const BTN_TOOL_AIRBRUSH:  i32 = 0x144;
pub const BTN_TOOL_FINGER:    i32 = 0x145;
pub const BTN_TOOL_MOUSE:     i32 = 0x146;
pub const BTN_TOOL_LENS:      i32 = 0x147;
pub const BTN_TOOL_QUINTTAP:  i32 = 0x148; /* Five fingers on trackpad */
pub const BTN_TOUCH:          i32 = 0x14a;
pub const BTN_STYLUS:         i32 = 0x14b;
pub const BTN_STYLUS2:        i32 = 0x14c;
pub const BTN_TOOL_DOUBLETAP: i32 = 0x14d;
pub const BTN_TOOL_TRIPLETAP: i32 = 0x14e;
pub const BTN_TOOL_QUADTAP:   i32 = 0x14f; /* Four fingers on trackpad */

pub const BTN_WHEEL:     i32 = 0x150;
pub const BTN_GEAR_DOWN: i32 = 0x150;
pub const BTN_GEAR_UP:   i32 = 0x151;

pub const KEY_OK:                i32 = 0x160;
pub const KEY_SELECT:            i32 = 0x161;
pub const KEY_GOTO:              i32 = 0x162;
pub const KEY_CLEAR:             i32 = 0x163;
pub const KEY_POWER2:            i32 = 0x164;
pub const KEY_OPTION:            i32 = 0x165;
pub const KEY_INFO:              i32 = 0x166; /* AL OEM Features/Tips/Tutorial */
pub const KEY_TIME:              i32 = 0x167;
pub const KEY_VENDOR:            i32 = 0x168;
pub const KEY_ARCHIVE:           i32 = 0x169;
pub const KEY_PROGRAM:           i32 = 0x16a; /* Media Select Program Guide */
pub const KEY_CHANNEL:           i32 = 0x16b;
pub const KEY_FAVORITES:         i32 = 0x16c;
pub const KEY_EPG:               i32 = 0x16d;
pub const KEY_PVR:               i32 = 0x16e; /* Media Select Home */
pub const KEY_MHP:               i32 = 0x16f;
pub const KEY_LANGUAGE:          i32 = 0x170;
pub const KEY_TITLE:             i32 = 0x171;
pub const KEY_SUBTITLE:          i32 = 0x172;
pub const KEY_ANGLE:             i32 = 0x173;
pub const KEY_ZOOM:              i32 = 0x174;
pub const KEY_MODE:              i32 = 0x175;
pub const KEY_KEYBOARD:          i32 = 0x176;
pub const KEY_SCREEN:            i32 = 0x177;
pub const KEY_PC:                i32 = 0x178; /* Media Select Computer */
pub const KEY_TV:                i32 = 0x179; /* Media Select TV */
pub const KEY_TV2:               i32 = 0x17a; /* Media Select Cable */
pub const KEY_VCR:               i32 = 0x17b; /* Media Select VCR */
pub const KEY_VCR2:              i32 = 0x17c; /* VCR Plus */
pub const KEY_SAT:               i32 = 0x17d; /* Media Select Satellite */
pub const KEY_SAT2:              i32 = 0x17e;
pub const KEY_CD:                i32 = 0x17f; /* Media Select CD */
pub const KEY_TAPE:              i32 = 0x180; /* Media Select Tape */
pub const KEY_RADIO:             i32 = 0x181;
pub const KEY_TUNER:             i32 = 0x182; /* Media Select Tuner */
pub const KEY_PLAYER:            i32 = 0x183;
pub const KEY_TEXT:              i32 = 0x184;
pub const KEY_DVD:               i32 = 0x185; /* Media Select DVD */
pub const KEY_AUX:               i32 = 0x186;
pub const KEY_MP3:               i32 = 0x187;
pub const KEY_AUDIO:             i32 = 0x188; /* AL Audio Browser */
pub const KEY_VIDEO:             i32 = 0x189; /* AL Movie Browser */
pub const KEY_DIRECTORY:         i32 = 0x18a;
pub const KEY_LIST:              i32 = 0x18b;
pub const KEY_MEMO:              i32 = 0x18c; /* Media Select Messages */
pub const KEY_CALENDAR:          i32 = 0x18d;
pub const KEY_RED:               i32 = 0x18e;
pub const KEY_GREEN:             i32 = 0x18f;
pub const KEY_YELLOW:            i32 = 0x190;
pub const KEY_BLUE:              i32 = 0x191;
pub const KEY_CHANNELUP:         i32 = 0x192; /* Channel Increment */
pub const KEY_CHANNELDOWN:       i32 = 0x193; /* Channel Decrement */
pub const KEY_FIRST:             i32 = 0x194;
pub const KEY_LAST:              i32 = 0x195; /* Recall Last */
pub const KEY_AB:                i32 = 0x196;
pub const KEY_NEXT:              i32 = 0x197;
pub const KEY_RESTART:           i32 = 0x198;
pub const KEY_SLOW:              i32 = 0x199;
pub const KEY_SHUFFLE:           i32 = 0x19a;
pub const KEY_BREAK:             i32 = 0x19b;
pub const KEY_PREVIOUS:          i32 = 0x19c;
pub const KEY_DIGITS:            i32 = 0x19d;
pub const KEY_TEEN:              i32 = 0x19e;
pub const KEY_TWEN:              i32 = 0x19f;
pub const KEY_VIDEOPHONE:        i32 = 0x1a0; /* Media Select Video Phone */
pub const KEY_GAMES:             i32 = 0x1a1; /* Media Select Games */
pub const KEY_ZOOMIN:            i32 = 0x1a2; /* AC Zoom In */
pub const KEY_ZOOMOUT:           i32 = 0x1a3; /* AC Zoom Out */
pub const KEY_ZOOMRESET:         i32 = 0x1a4; /* AC Zoom */
pub const KEY_WORDPROCESSOR:     i32 = 0x1a5; /* AL Word Processor */
pub const KEY_EDITOR:            i32 = 0x1a6; /* AL Text Editor */
pub const KEY_SPREADSHEET:       i32 = 0x1a7; /* AL Spreadsheet */
pub const KEY_GRAPHICSEDITOR:    i32 = 0x1a8; /* AL Graphics Editor */
pub const KEY_PRESENTATION:      i32 = 0x1a9; /* AL Presentation App */
pub const KEY_DATABASE:          i32 = 0x1aa; /* AL Database App */
pub const KEY_NEWS:              i32 = 0x1ab; /* AL Newsreader */
pub const KEY_VOICEMAIL:         i32 = 0x1ac; /* AL Voicemail */
pub const KEY_ADDRESSBOOK:       i32 = 0x1ad; /* AL Contacts/Address Book */
pub const KEY_MESSENGER:         i32 = 0x1ae; /* AL Instant Messaging */
pub const KEY_DISPLAYTOGGLE:     i32 = 0x1af; /* Turn display : i32 = LCD on and off */
pub const KEY_BRIGHTNESS_TOGGLE: i32 = KEY_DISPLAYTOGGLE;
pub const KEY_SPELLCHECK:        i32 = 0x1b0; /* AL Spell Check */
pub const KEY_LOGOFF:            i32 = 0x1b1; /* AL Logoff */

pub const KEY_DOLLAR: i32 = 0x1b2;
pub const KEY_EURO:   i32 = 0x1b3;

pub const KEY_FRAMEBACK:      i32 = 0x1b4; /* Consumer - transport controls */
pub const KEY_FRAMEFORWARD:   i32 = 0x1b5;
pub const KEY_CONTEXT_MENU:   i32 = 0x1b6; /* GenDesc - system context menu */
pub const KEY_MEDIA_REPEAT:   i32 = 0x1b7; /* Consumer - transport control */
pub const KEY_10CHANNELSUP:   i32 = 0x1b8; /* 10 channels up : i32 = 10+ */
pub const KEY_10CHANNELSDOWN: i32 = 0x1b9; /* 10 channels down : i32 = 10- */
pub const KEY_IMAGES:         i32 = 0x1ba; /* AL Image Browser */

pub const KEY_DEL_EOL:  i32 = 0x1c0;
pub const KEY_DEL_EOS:  i32 = 0x1c1;
pub const KEY_INS_LINE: i32 = 0x1c2;
pub const KEY_DEL_LINE: i32 = 0x1c3;

pub const KEY_FN:     i32 = 0x1d0;
pub const KEY_FN_ESC: i32 = 0x1d1;
pub const KEY_FN_F1:  i32 = 0x1d2;
pub const KEY_FN_F2:  i32 = 0x1d3;
pub const KEY_FN_F3:  i32 = 0x1d4;
pub const KEY_FN_F4:  i32 = 0x1d5;
pub const KEY_FN_F5:  i32 = 0x1d6;
pub const KEY_FN_F6:  i32 = 0x1d7;
pub const KEY_FN_F7:  i32 = 0x1d8;
pub const KEY_FN_F8:  i32 = 0x1d9;
pub const KEY_FN_F9:  i32 = 0x1da;
pub const KEY_FN_F10: i32 = 0x1db;
pub const KEY_FN_F11: i32 = 0x1dc;
pub const KEY_FN_F12: i32 = 0x1dd;
pub const KEY_FN_1:   i32 = 0x1de;
pub const KEY_FN_2:   i32 = 0x1df;
pub const KEY_FN_D:   i32 = 0x1e0;
pub const KEY_FN_E:   i32 = 0x1e1;
pub const KEY_FN_F:   i32 = 0x1e2;
pub const KEY_FN_S:   i32 = 0x1e3;
pub const KEY_FN_B:   i32 = 0x1e4;

pub const KEY_BRL_DOT1:  i32 = 0x1f1;
pub const KEY_BRL_DOT2:  i32 = 0x1f2;
pub const KEY_BRL_DOT3:  i32 = 0x1f3;
pub const KEY_BRL_DOT4:  i32 = 0x1f4;
pub const KEY_BRL_DOT5:  i32 = 0x1f5;
pub const KEY_BRL_DOT6:  i32 = 0x1f6;
pub const KEY_BRL_DOT7:  i32 = 0x1f7;
pub const KEY_BRL_DOT8:  i32 = 0x1f8;
pub const KEY_BRL_DOT9:  i32 = 0x1f9;
pub const KEY_BRL_DOT10: i32 = 0x1fa;

pub const KEY_NUMERIC_0:     i32 = 0x200; /* used by phones, remote controls, */
pub const KEY_NUMERIC_1:     i32 = 0x201; /* and other keypads */
pub const KEY_NUMERIC_2:     i32 = 0x202;
pub const KEY_NUMERIC_3:     i32 = 0x203;
pub const KEY_NUMERIC_4:     i32 = 0x204;
pub const KEY_NUMERIC_5:     i32 = 0x205;
pub const KEY_NUMERIC_6:     i32 = 0x206;
pub const KEY_NUMERIC_7:     i32 = 0x207;
pub const KEY_NUMERIC_8:     i32 = 0x208;
pub const KEY_NUMERIC_9:     i32 = 0x209;
pub const KEY_NUMERIC_STAR:  i32 = 0x20a;
pub const KEY_NUMERIC_POUND: i32 = 0x20b;
pub const KEY_NUMERIC_A:     i32 = 0x20c; /* Phone key A - HUT Telephony 0xb9 */
pub const KEY_NUMERIC_B:     i32 = 0x20d;
pub const KEY_NUMERIC_C:     i32 = 0x20e;
pub const KEY_NUMERIC_D:     i32 = 0x20f;

pub const KEY_CAMERA_FOCUS: i32 = 0x210;
pub const KEY_WPS_BUTTON:   i32 = 0x211; /* WiFi Protected Setup key */

pub const KEY_TOUCHPAD_TOGGLE: i32 = 0x212; /* Request switch touchpad on or off */
pub const KEY_TOUCHPAD_ON:     i32 = 0x213;
pub const KEY_TOUCHPAD_OFF:    i32 = 0x214;

pub const KEY_CAMERA_ZOOMIN:  i32 = 0x215;
pub const KEY_CAMERA_ZOOMOUT: i32 = 0x216;
pub const KEY_CAMERA_UP:      i32 = 0x217;
pub const KEY_CAMERA_DOWN:    i32 = 0x218;
pub const KEY_CAMERA_LEFT:    i32 = 0x219;
pub const KEY_CAMERA_RIGHT:   i32 = 0x21a;

pub const KEY_ATTENDANT_ON:     i32 = 0x21b;
pub const KEY_ATTENDANT_OFF:    i32 = 0x21c;
pub const KEY_ATTENDANT_TOGGLE: i32 = 0x21d; /* Attendant call on or off */
pub const KEY_LIGHTS_TOGGLE:    i32 = 0x21e; /* Reading light on or off */

pub const BTN_DPAD_UP:    i32 = 0x220;
pub const BTN_DPAD_DOWN:  i32 = 0x221;
pub const BTN_DPAD_LEFT:  i32 = 0x222;
pub const BTN_DPAD_RIGHT: i32 = 0x223;

pub const KEY_ALS_TOGGLE: i32 = 0x230; /* Ambient light sensor */

pub const KEY_BUTTONCONFIG: i32 = 0x240;  /* AL Button Configuration */
pub const KEY_TASKMANAGER:  i32 = 0x241;  /* AL Task/Project Manager */
pub const KEY_JOURNAL:      i32 = 0x242;  /* AL Log/Journal/Timecard */
pub const KEY_CONTROLPANEL: i32 = 0x243;  /* AL Control Panel */
pub const KEY_APPSELECT:    i32 = 0x244;  /* AL Select Task/Application */
pub const KEY_SCREENSAVER:  i32 = 0x245;  /* AL Screen Saver */
pub const KEY_VOICECOMMAND: i32 = 0x246;  /* Listening Voice Command */

pub const KEY_BRIGHTNESS_MIN: i32 = 0x250; /* Set Brightness to Minimum */
pub const KEY_BRIGHTNESS_MAX: i32 = 0x251; /* Set Brightness to Maximum */

pub const KEY_KBDINPUTASSIST_PREV:      i32 = 0x260;
pub const KEY_KBDINPUTASSIST_NEXT:      i32 = 0x261;
pub const KEY_KBDINPUTASSIST_PREVGROUP: i32 = 0x262;
pub const KEY_KBDINPUTASSIST_NEXTGROUP: i32 = 0x263;
pub const KEY_KBDINPUTASSIST_ACCEPT:    i32 = 0x264;
pub const KEY_KBDINPUTASSIST_CANCEL:    i32 = 0x265;

pub const BTN_TRIGGER_HAPPY:   i32 = 0x2c0;
pub const BTN_TRIGGER_HAPPY1:  i32 = 0x2c0;
pub const BTN_TRIGGER_HAPPY2:  i32 = 0x2c1;
pub const BTN_TRIGGER_HAPPY3:  i32 = 0x2c2;
pub const BTN_TRIGGER_HAPPY4:  i32 = 0x2c3;
pub const BTN_TRIGGER_HAPPY5:  i32 = 0x2c4;
pub const BTN_TRIGGER_HAPPY6:  i32 = 0x2c5;
pub const BTN_TRIGGER_HAPPY7:  i32 = 0x2c6;
pub const BTN_TRIGGER_HAPPY8:  i32 = 0x2c7;
pub const BTN_TRIGGER_HAPPY9:  i32 = 0x2c8;
pub const BTN_TRIGGER_HAPPY10: i32 = 0x2c9;
pub const BTN_TRIGGER_HAPPY11: i32 = 0x2ca;
pub const BTN_TRIGGER_HAPPY12: i32 = 0x2cb;
pub const BTN_TRIGGER_HAPPY13: i32 = 0x2cc;
pub const BTN_TRIGGER_HAPPY14: i32 = 0x2cd;
pub const BTN_TRIGGER_HAPPY15: i32 = 0x2ce;
pub const BTN_TRIGGER_HAPPY16: i32 = 0x2cf;
pub const BTN_TRIGGER_HAPPY17: i32 = 0x2d0;
pub const BTN_TRIGGER_HAPPY18: i32 = 0x2d1;
pub const BTN_TRIGGER_HAPPY19: i32 = 0x2d2;
pub const BTN_TRIGGER_HAPPY20: i32 = 0x2d3;
pub const BTN_TRIGGER_HAPPY21: i32 = 0x2d4;
pub const BTN_TRIGGER_HAPPY22: i32 = 0x2d5;
pub const BTN_TRIGGER_HAPPY23: i32 = 0x2d6;
pub const BTN_TRIGGER_HAPPY24: i32 = 0x2d7;
pub const BTN_TRIGGER_HAPPY25: i32 = 0x2d8;
pub const BTN_TRIGGER_HAPPY26: i32 = 0x2d9;
pub const BTN_TRIGGER_HAPPY27: i32 = 0x2da;
pub const BTN_TRIGGER_HAPPY28: i32 = 0x2db;
pub const BTN_TRIGGER_HAPPY29: i32 = 0x2dc;
pub const BTN_TRIGGER_HAPPY30: i32 = 0x2dd;
pub const BTN_TRIGGER_HAPPY31: i32 = 0x2de;
pub const BTN_TRIGGER_HAPPY32: i32 = 0x2df;
pub const BTN_TRIGGER_HAPPY33: i32 = 0x2e0;
pub const BTN_TRIGGER_HAPPY34: i32 = 0x2e1;
pub const BTN_TRIGGER_HAPPY35: i32 = 0x2e2;
pub const BTN_TRIGGER_HAPPY36: i32 = 0x2e3;
pub const BTN_TRIGGER_HAPPY37: i32 = 0x2e4;
pub const BTN_TRIGGER_HAPPY38: i32 = 0x2e5;
pub const BTN_TRIGGER_HAPPY39: i32 = 0x2e6;
pub const BTN_TRIGGER_HAPPY40: i32 = 0x2e7;

/* We avoid low common keys in module aliases so they don't get huge. */
pub const KEY_MIN_INTERESTING: i32 = KEY_MUTE;
pub const KEY_MAX:             i32 = 0x2ff;
pub const KEY_CNT:             i32 = KEY_MAX + 1;

/*
 * Relative axes
 */

pub const REL_X:      i32 = 0x00;
pub const REL_Y:      i32 = 0x01;
pub const REL_Z:      i32 = 0x02;
pub const REL_RX:     i32 = 0x03;
pub const REL_RY:     i32 = 0x04;
pub const REL_RZ:     i32 = 0x05;
pub const REL_HWHEEL: i32 = 12;
pub const REL_DIAL:   i32 = 0x07;
pub const REL_WHEEL:  i32 = 11;
pub const REL_MISC:   i32 = 0x09;
pub const REL_MAX:    i32 = 0x0f;
pub const REL_CNT:    i32 = REL_MAX + 1;

/*
 * Absolute axes
 */

pub const ABS_X:          i32 = 0x00;
pub const ABS_Y:          i32 = 0x01;
pub const ABS_Z:          i32 = 0x02;
pub const ABS_RX:         i32 = 0x03;
pub const ABS_RY:         i32 = 0x04;
pub const ABS_RZ:         i32 = 0x05;
pub const ABS_THROTTLE:   i32 = 0x06;
pub const ABS_RUDDER:     i32 = 0x07;
pub const ABS_WHEEL:      i32 = 0x08;
pub const ABS_GAS:        i32 = 0x09;
pub const ABS_BRAKE:      i32 = 0x0a;
pub const ABS_HAT0X:      i32 = 0x10;
pub const ABS_HAT0Y:      i32 = 0x11;
pub const ABS_HAT1X:      i32 = 0x12;
pub const ABS_HAT1Y:      i32 = 0x13;
pub const ABS_HAT2X:      i32 = 0x14;
pub const ABS_HAT2Y:      i32 = 0x15;
pub const ABS_HAT3X:      i32 = 0x16;
pub const ABS_HAT3Y:      i32 = 0x17;
pub const ABS_PRESSURE:   i32 = 0x18;
pub const ABS_DISTANCE:   i32 = 0x19;
pub const ABS_TILT_X:     i32 = 0x1a;
pub const ABS_TILT_Y:     i32 = 0x1b;
pub const ABS_TOOL_WIDTH: i32 = 0x1c;

pub const ABS_VOLUME: i32 = 0x20;

pub const ABS_MISC: i32 = 0x28;

pub const ABS_MT_SLOT:        i32 = 0x2f; /* MT slot being modified */
pub const ABS_MT_TOUCH_MAJOR: i32 = 0x30; /* Major axis of touching ellipse */
pub const ABS_MT_TOUCH_MINOR: i32 = 0x31; /* Minor axis : i32 = omit if circular */
pub const ABS_MT_WIDTH_MAJOR: i32 = 0x32; /* Major axis of approaching ellipse */
pub const ABS_MT_WIDTH_MINOR: i32 = 0x33; /* Minor axis : i32 = omit if circular */
pub const ABS_MT_ORIENTATION: i32 = 0x34; /* Ellipse orientation */
pub const ABS_MT_POSITION_X:  i32 = 0x35; /* Center X touch position */
pub const ABS_MT_POSITION_Y:  i32 = 0x36; /* Center Y touch position */
pub const ABS_MT_TOOL_TYPE:   i32 = 0x37; /* Type of touching device */
pub const ABS_MT_BLOB_ID:     i32 = 0x38; /* Group a set of packets as a blob */
pub const ABS_MT_TRACKING_ID: i32 = 0x39; /* Unique ID of initiated contact */
pub const ABS_MT_PRESSURE:    i32 = 0x3a; /* Pressure on contact area */
pub const ABS_MT_DISTANCE:    i32 = 0x3b; /* Contact hover distance */
pub const ABS_MT_TOOL_X:      i32 = 0x3c; /* Center X tool position */
pub const ABS_MT_TOOL_Y:      i32 = 0x3d; /* Center Y tool position */


pub const ABS_MAX: i32 = 0x3f;
pub const ABS_CNT: i32 = ABS_MAX + 1;

/*
 * Switch events
 */

pub const SW_LID:                  i32 = 0x00;  /* set = lid shut */
pub const SW_TABLET_MODE:          i32 = 0x01;  /* set = tablet mode */
pub const SW_HEADPHONE_INSERT:     i32 = 0x02;  /* set = inserted */
pub const SW_RFKILL_ALL:           i32 = 0x03;  /* rfkill master switch, type "any" set = radio enabled */
pub const SW_RADIO:                i32 = SW_RFKILL_ALL; /* deprecated */
pub const SW_MICROPHONE_INSERT:    i32 = 0x04;  /* set = inserted */
pub const SW_DOCK:                 i32 = 0x05;  /* set = plugged into dock */
pub const SW_LINEOUT_INSERT:       i32 = 0x06;  /* set = inserted */
pub const SW_JACK_PHYSICAL_INSERT: i32 = 0x07;  /* set = mechanical switch set */
pub const SW_VIDEOOUT_INSERT:      i32 = 0x08;  /* set = inserted */
pub const SW_CAMERA_LENS_COVER:    i32 = 0x09;  /* set = lens covered */
pub const SW_KEYPAD_SLIDE:         i32 = 0x0a;  /* set = keypad slide out */
pub const SW_FRONT_PROXIMITY:      i32 = 0x0b;  /* set = front proximity sensor active */
pub const SW_ROTATE_LOCK:          i32 = 0x0c;  /* set = rotate locked/disabled */
pub const SW_LINEIN_INSERT:        i32 = 0x0d;  /* set = inserted */
pub const SW_MUTE_DEVICE:          i32 = 0x0e;  /* set = device disabled */
pub const SW_MAX:                  i32 = 0x0f;
pub const SW_CNT:                  i32 = SW_MAX + 1;

/*
 * Misc events
 */

pub const MSC_SERIAL:    i32 = 0x00;
pub const MSC_PULSELED:  i32 = 0x01;
pub const MSC_GESTURE:   i32 = 0x02;
pub const MSC_RAW:       i32 = 0x03;
pub const MSC_SCAN:      i32 = 0x04;
pub const MSC_TIMESTAMP: i32 = 0x05;
pub const MSC_MAX:       i32 = 0x07;
pub const MSC_CNT:       i32 = MSC_MAX + 1;

/*
 * LEDs
 */

pub const LED_NUML:     i32 = 0x00;
pub const LED_CAPSL:    i32 = 0x01;
pub const LED_SCROLLL:  i32 = 0x02;
pub const LED_COMPOSE:  i32 = 0x03;
pub const LED_KANA:     i32 = 0x04;
pub const LED_SLEEP:    i32 = 0x05;
pub const LED_SUSPEND:  i32 = 0x06;
pub const LED_MUTE:     i32 = 0x07;
pub const LED_MISC:     i32 = 0x08;
pub const LED_MAIL:     i32 = 0x09;
pub const LED_CHARGING: i32 = 0x0a;
pub const LED_MAX:      i32 = 0x0f;
pub const LED_CNT:      i32 = LED_MAX + 1;

/*
 * Autorepeat values
 */

pub const REP_DELAY:  i32 = 0x00;
pub const REP_PERIOD: i32 = 0x01;
pub const REP_MAX:    i32 = 0x01;
pub const REP_CNT:    i32 = REP_MAX + 1;

/*
 * Sounds
 */

pub const SND_CLICK: i32 = 0x00;
pub const SND_BELL:  i32 = 0x01;
pub const SND_TONE:  i32 = 0x02;
pub const SND_MAX:   i32 = 0x07;
pub const SND_CNT:   i32 = SND_MAX + 1;
