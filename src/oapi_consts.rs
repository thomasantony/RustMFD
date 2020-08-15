// Find: #define\s([\w]+)\s*(\w+)(.*)$
// Replace: const $1: i32 = $2; $3
// ======================================================================
/**
 * \ingroup defines
 * \defgroup keycodes Keyboard key identifiers
 */
// ======================================================================
//@{
pub const OAPI_KEY_ESCAPE: u32 = 0x01;   // < Escape key
pub const OAPI_KEY_1: u32 = 0x02;   // < '1' key on main keyboard
pub const OAPI_KEY_2: u32 = 0x03;   // < '2' key on main keyboard
pub const OAPI_KEY_3: u32 = 0x04;   // < '3' key on main keyboard
pub const OAPI_KEY_4: u32 = 0x05;   // < '4' key on main keyboard
pub const OAPI_KEY_5: u32 = 0x06;   // < '5' key on main keyboard
pub const OAPI_KEY_6: u32 = 0x07;   // < '6' key on main keyboard
pub const OAPI_KEY_7: u32 = 0x08;   // < '7' key on main keyboard
pub const OAPI_KEY_8: u32 = 0x09;   // < '8' key on main keyboard
pub const OAPI_KEY_9: u32 = 0x0A;   // < '9' key on main keyboard
pub const OAPI_KEY_0: u32 = 0x0B;   // < '0' key on main keyboard
pub const OAPI_KEY_MINUS: u32 = 0x0C;   // < '-' key on main keyboard
pub const OAPI_KEY_EQUALS: u32 = 0x0D;   // < '=' key on main keyboard
pub const OAPI_KEY_BACK: u32 = 0x0E;   // < backspace key
pub const OAPI_KEY_TAB: u32 = 0x0F;   // < tab key
pub const OAPI_KEY_Q: u32 = 0x10;   // < 'Q' key
pub const OAPI_KEY_W: u32 = 0x11;   // < 'W' key
pub const OAPI_KEY_E: u32 = 0x12;   // < 'E' key
pub const OAPI_KEY_R: u32 = 0x13;   // < 'R' key
pub const OAPI_KEY_T: u32 = 0x14;   // < 'T' key
pub const OAPI_KEY_Y: u32 = 0x15;   // < 'Y' key
pub const OAPI_KEY_U: u32 = 0x16;   // < 'U' key
pub const OAPI_KEY_I: u32 = 0x17;   // < 'I' key
pub const OAPI_KEY_O: u32 = 0x18;   // < 'O' key
pub const OAPI_KEY_P: u32 = 0x19;   // < 'P' key
pub const OAPI_KEY_LBRACKET: u32 = 0x1A;   // < '[' (left bracket) key
pub const OAPI_KEY_RBRACKET: u32 = 0x1B;   // < ']' (right bracket) key
pub const OAPI_KEY_RETURN: u32 = 0x1C;   // < 'Enter' key on main keyboard
pub const OAPI_KEY_LCONTROL: u32 = 0x1D;   // < Left 'Ctrl' key
pub const OAPI_KEY_A: u32 = 0x1E;   // < 'A' key
pub const OAPI_KEY_S: u32 = 0x1F;   // < 'S' key
pub const OAPI_KEY_D: u32 = 0x20;   // < 'D' key
pub const OAPI_KEY_F: u32 = 0x21;   // < 'F' key
pub const OAPI_KEY_G: u32 = 0x22;   // < 'G' key
pub const OAPI_KEY_H: u32 = 0x23;   // < 'H' key
pub const OAPI_KEY_J: u32 = 0x24;   // < 'J' key
pub const OAPI_KEY_K: u32 = 0x25;   // < 'K' key
pub const OAPI_KEY_L: u32 = 0x26;   // < 'L' key
pub const OAPI_KEY_SEMICOLON: u32 = 0x27;   // < ';' (semicolon) key
pub const OAPI_KEY_APOSTROPHE: u32 = 0x28;   // < ' (apostrophe) key
pub const OAPI_KEY_GRAVE: u32 = 0x29;   // < accent grave
pub const OAPI_KEY_LSHIFT: u32 = 0x2A;   // < Left 'Shift' key
pub const OAPI_KEY_BACKSLASH: u32 = 0x2B;   // < '\' (Backslash) key
pub const OAPI_KEY_Z: u32 = 0x2C;   // < 'Z' key
pub const OAPI_KEY_X: u32 = 0x2D;   // < 'X' key
pub const OAPI_KEY_C: u32 = 0x2E;   // < 'C' key
pub const OAPI_KEY_V: u32 = 0x2F;   // < 'V' key
pub const OAPI_KEY_B: u32 = 0x30;   // < 'B' key
pub const OAPI_KEY_N: u32 = 0x31;   // < 'N' key
pub const OAPI_KEY_M: u32 = 0x32;   // < 'M' key
pub const OAPI_KEY_COMMA: u32 = 0x33;   // < ',' (comma) key
pub const OAPI_KEY_PERIOD: u32 = 0x34;   // < '.' key on main keyboard
pub const OAPI_KEY_SLASH: u32 = 0x35;   // < '/' key on main keyboard
pub const OAPI_KEY_RSHIFT: u32 = 0x36;   // < Right 'Shift' key
pub const OAPI_KEY_MULTIPLY: u32 = 0x37;   // < * on numeric keypad
pub const OAPI_KEY_LALT: u32 = 0x38;   // < left Alt
pub const OAPI_KEY_SPACE: u32 = 0x39;   // < 'Space' key
pub const OAPI_KEY_CAPITAL: u32 = 0x3A;   // < caps lock key
pub const OAPI_KEY_F1: u32 = 0x3B;   // < F1 function key
pub const OAPI_KEY_F2: u32 = 0x3C;   // < F2 function key
pub const OAPI_KEY_F3: u32 = 0x3D;   // < F3 function key
pub const OAPI_KEY_F4: u32 = 0x3E;   // < F4 function key
pub const OAPI_KEY_F5: u32 = 0x3F;   // < F5 function key
pub const OAPI_KEY_F6: u32 = 0x40;   // < F6 function key
pub const OAPI_KEY_F7: u32 = 0x41;   // < F7 function key
pub const OAPI_KEY_F8: u32 = 0x42;   // < F8 function key
pub const OAPI_KEY_F9: u32 = 0x43;   // < F9 function key
pub const OAPI_KEY_F10: u32 = 0x44;   // < F10 function key
pub const OAPI_KEY_NUMLOCK: u32 = 0x45;   // < 'Num Lock' key
pub const OAPI_KEY_SCROLL: u32 = 0x46;   // < Scroll lock
pub const OAPI_KEY_NUMPAD7: u32 = 0x47;   // < '7' key on numeric keypad
pub const OAPI_KEY_NUMPAD8: u32 = 0x48;   // < '8' key on numeric keypad
pub const OAPI_KEY_NUMPAD9: u32 = 0x49;   // < '9' key on numeric keypad
pub const OAPI_KEY_SUBTRACT: u32 = 0x4A;   // < '-' key on numeric keypad
pub const OAPI_KEY_NUMPAD4: u32 = 0x4B;   // < '4' key on numeric keypad
pub const OAPI_KEY_NUMPAD5: u32 = 0x4C;   // < '5' key on numeric keypad
pub const OAPI_KEY_NUMPAD6: u32 = 0x4D;   // < '6' key on numeric keypad
pub const OAPI_KEY_ADD: u32 = 0x4E;   // < '+' key on numeric keypad
pub const OAPI_KEY_NUMPAD1: u32 = 0x4F;   // < '1' key on numeric keypad
pub const OAPI_KEY_NUMPAD2: u32 = 0x50;   // < '2' key on numeric keypad
pub const OAPI_KEY_NUMPAD3: u32 = 0x51;   // < '3' key on numeric keypad
pub const OAPI_KEY_NUMPAD0: u32 = 0x52;   // < '0' key on numeric keypad
pub const OAPI_KEY_DECIMAL: u32 = 0x53;   // < '.' key on numeric keypad
pub const OAPI_KEY_OEM_102: u32 = 0x56;   // < | \< \> on UK/German keyboards
pub const OAPI_KEY_F11: u32 = 0x57;   // < F11 function key
pub const OAPI_KEY_F12: u32 = 0x58;   // < F12 function key
pub const OAPI_KEY_NUMPADENTER: u32 = 0x9C;   // < Enter on numeric keypad
pub const OAPI_KEY_RCONTROL: u32 = 0x9D;   // < right Control key
pub const OAPI_KEY_DIVIDE: u32 = 0xB5;   // < '/' key on numeric keypad
pub const OAPI_KEY_SYSRQ: u32 = 0xB7;   // < SysRq/PrtScn key
pub const OAPI_KEY_RALT: u32 = 0xB8;   // < right Alt
pub const OAPI_KEY_PAUSE: u32 = 0xC5;   // < Break/Pause key
pub const OAPI_KEY_HOME: u32 = 0xC7;   // < Home on cursor keypad
pub const OAPI_KEY_UP: u32 = 0xC8;   // < up-arrow on cursor keypad
pub const OAPI_KEY_PRIOR: u32 = 0xC9;   // < PgUp on cursor keypad
pub const OAPI_KEY_LEFT: u32 = 0xCB;   // < left-arrow on cursor keypad
pub const OAPI_KEY_RIGHT: u32 = 0xCD;   // < right-arrow on cursor keypad
pub const OAPI_KEY_END: u32 = 0xCF;   // < End on cursor keypad
pub const OAPI_KEY_DOWN: u32 = 0xD0;   // < down-arrow on cursor keypad
pub const OAPI_KEY_NEXT: u32 = 0xD1;   // < PgDn on cursor keypad
pub const OAPI_KEY_INSERT: u32 = 0xD2;   // < Insert on cursor keypad
pub const OAPI_KEY_DELETE: u32 = 0xD3;   // < Delete on cursor keypad
//@}

// const KEYDOW: i32 = N; (buf,key) (buf[key] & 0x80)
// const RESETKE: i32 = Y; (buf,key) (buf[key] = 0)

// const KEYMOD_LSHIF: i32 = T; (buf)   (KEYDOWN(buf,OAPI_KEY_LSHIFT))
// const KEYMOD_RSHIF: i32 = T; (buf)   (KEYDOWN(buf,OAPI_KEY_RSHIFT))
// const KEYMOD_SHIF: i32 = T; (buf)    (KEYMOD_LSHIFT(buf) || KEYMOD_RSHIFT(buf))
// const KEYMOD_LCONTRO: i32 = L; (buf) (KEYDOWN(buf,OAPI_KEY_LCONTROL))
// const KEYMOD_RCONTRO: i32 = L; (buf) (KEYDOWN(buf,OAPI_KEY_RCONTROL))
// const KEYMOD_CONTRO: i32 = L; (buf)  (KEYMOD_LCONTROL(buf) || KEYMOD_RCONTROL(buf))
// const KEYMOD_LAL: i32 = T; (buf)     (KEYDOWN(buf,OAPI_KEY_LALT))
// const KEYMOD_RAL: i32 = T; (buf)     (KEYDOWN(buf,OAPI_KEY_RALT))
// const KEYMOD_AL: i32 = T; (buf)      (KEYMOD_LALT(buf) || KEYMOD_RALT(buf))

// ======================================================================
/// \ingroup defines
/// \defgroup logical_keys Logical key ids
// ======================================================================
//@{
pub const OAPI_LKEY_CockpitRotateLeft: i32 = 0;  // < rotate camera left in cockpit view
pub const OAPI_LKEY_CockpitRotateRight: i32 = 1;  // < rotate camera right in cockpit view
pub const OAPI_LKEY_CockpitRotateUp: i32 = 2;  // < rotate camera up in cockpit view
pub const OAPI_LKEY_CockpitRotateDown: i32 = 3;  // < rotate camera down in cockpit view
pub const OAPI_LKEY_CockpitDontLean: i32 = 4;  // < return to default cockpit camera position
pub const OAPI_LKEY_CockpitLeanForward: i32 = 5;  // < move cockpit camera forward
pub const OAPI_LKEY_CockpitLeanLeft: i32 = 6;  // < move cockpit camera left
pub const OAPI_LKEY_CockpitLeanRight: i32 = 7;  // < move cockpit camera right
pub const OAPI_LKEY_CockpitResetCam: i32 = 8;  // < rotate and shift cockpit camera back to default
pub const OAPI_LKEY_PanelShiftLeft: i32 = 9;  // < shift 2D instrument panel left
pub const OAPI_LKEY_PanelShiftRight: i32 = 10;  // < shift 2D instrument panel right
pub const OAPI_LKEY_PanelShiftUp: i32 = 11;  // < shift 2D instrument panel up
pub const OAPI_LKEY_PanelShiftDown: i32 = 12;  // < shift 2D instrument panel down
pub const OAPI_LKEY_PanelSwitchLeft: i32 = 13;  // < switch to left neighbour panel
pub const OAPI_LKEY_PanelSwitchRight: i32 = 14;  // < switch to right neighbour panel
pub const OAPI_LKEY_PanelSwitchUp: i32 = 15;  // < switch to upper neighbour panel
pub const OAPI_LKEY_PanelSwitchDown: i32 = 16;  // < switch to lower neighbour panel
pub const OAPI_LKEY_TrackRotateLeft: i32 = 17;  // < turn track view camera left
pub const OAPI_LKEY_TrackRotateRight: i32 = 18;  // < turn track view camera right
pub const OAPI_LKEY_TrackRotateUp: i32 = 19;  // < turn track view camera up
pub const OAPI_LKEY_TrackRotateDown: i32 = 20;  // < turn track view camera down
pub const OAPI_LKEY_TrackAdvance: i32 = 21;  // < advance track view camera towards target
pub const OAPI_LKEY_TrackRetreat: i32 = 22;  // < retreat track view camera from target
pub const OAPI_LKEY_GroundTiltLeft: i32 = 23;  // < tilt camera left in ground view
pub const OAPI_LKEY_GroundTiltRight: i32 = 24;  // < tilt camera right in ground view
pub const OAPI_LKEY_GroundTiltUp: i32 = 25;  // < tilt camera up in ground view
pub const OAPI_LKEY_GroundTiltDown: i32 = 26;  // < tilt camera down in ground view
pub const OAPI_LKEY_IncMainThrust: i32 = 27;  // < increment thrust of main thrusters
pub const OAPI_LKEY_DecMainThrust: i32 = 28;  // < decrement thrust of main thrusters
pub const OAPI_LKEY_KillMainRetro: i32 = 29;  // < kill main and retro thrusters
pub const OAPI_LKEY_FullMainThrust: i32 = 30;  // < temporary full main thrust
pub const OAPI_LKEY_FullRetroThrust: i32 = 31;  // < temporary full retro thrust
pub const OAPI_LKEY_IncHoverThrust: i32 = 32;  // < increment thrust of hover thrusters
pub const OAPI_LKEY_DecHoverThrust: i32 = 33;  // < decrement thrust of hover thrusters
pub const OAPI_LKEY_RCSEnable: i32 = 34;  // < enable/disable RCS (reaction control system)
pub const OAPI_LKEY_RCSMode: i32 = 35;  // < toggle linear/rotational RCS mode
pub const OAPI_LKEY_RCSPitchUp: i32 = 36;  // < rotational RCS: pitch up
pub const OAPI_LKEY_RCSPitchDown: i32 = 37;  // < rotational RCS: pitch down
pub const OAPI_LKEY_RCSYawLeft: i32 = 38;  // < rotational RCS: yaw left
pub const OAPI_LKEY_RCSYawRight: i32 = 39;  // < rotational RCS: yaw right
pub const OAPI_LKEY_RCSBankLeft: i32 = 40;  // < rotational RCS: bank left
pub const OAPI_LKEY_RCSBankRight: i32 = 41;  // < rotational RCS: bank right
pub const OAPI_LKEY_RCSUp: i32 = 42;  // < linear RCS: accelerate up (+y)
pub const OAPI_LKEY_RCSDown: i32 = 43;  // < linear RCS: accelerate down (-y)
pub const OAPI_LKEY_RCSLeft: i32 = 44;  // < linear RCS: accelerate left (-x)
pub const OAPI_LKEY_RCSRight: i32 = 45;  // < linear RCS: accelerate right (+x)
pub const OAPI_LKEY_RCSForward: i32 = 46;  // < linear RCS: accelerate forward (+z)
pub const OAPI_LKEY_RCSBack: i32 = 47;  // < linear RCS: accelerate backward (-z)
pub const OAPI_LKEY_LPRCSPitchUp: i32 = 48;  // < rotational RCS: pitch up 10%
pub const OAPI_LKEY_LPRCSPitchDown: i32 = 49;  // < rotational RCS: pitch down 10%
pub const OAPI_LKEY_LPRCSYawLeft: i32 = 50;  // < rotational RCS: yaw left 10%
pub const OAPI_LKEY_LPRCSYawRight: i32 = 51;  // < rotational RCS: yaw right 10%
pub const OAPI_LKEY_LPRCSBankLeft: i32 = 52;  // < rotational RCS: bank left 10%
pub const OAPI_LKEY_LPRCSBankRight: i32 = 53;  // < rotational RCS: bank right 10%
pub const OAPI_LKEY_LPRCSUp: i32 = 54;  // < linear RCS: accelerate up 10% (+y)
pub const OAPI_LKEY_LPRCSDown: i32 = 55;  // < linear RCS: accelerate down 10% (-y)
pub const OAPI_LKEY_LPRCSLeft: i32 = 56;  // < linear RCS: accelerate left 10% (-x)
pub const OAPI_LKEY_LPRCSRight: i32 = 57;  // < linear RCS: accelerate right 10% (+x)
pub const OAPI_LKEY_LPRCSForward: i32 = 58;  // < linear RCS: accelerate forward 10% (+z)
pub const OAPI_LKEY_LPRCSBack: i32 = 59;  // < linear RCS: accelerate backward 10% (-z)
pub const OAPI_LKEY_NMHoldAltitude: i32 = 60;  // < toggle navmode: hold altitude
pub const OAPI_LKEY_NMHLevel: i32 = 61;  // < toggle navmode: level with horizon
pub const OAPI_LKEY_NMPrograde: i32 = 62;  // < toggle navmode: prograde
pub const OAPI_LKEY_NMRetrograde: i32 = 63;  // < toggle navmode: retrograde
pub const OAPI_LKEY_NMNormal: i32 = 64;  // < toggle navmode: normal to orbital plane
pub const OAPI_LKEY_NMAntinormal: i32 = 65;  // < toggle navmode: antinormal to orbital plane
pub const OAPI_LKEY_NMKillrot: i32 = 66;  // < toggle navmode: kill rotation
pub const OAPI_LKEY_Undock: i32 = 67;  // < undock from docked vessel
pub const OAPI_LKEY_IncElevatorTrim: i32 = 68;  // < increment elevator trim setting
pub const OAPI_LKEY_DecElevatorTrim: i32 = 69;  // < decrement elevator trim setting
pub const OAPI_LKEY_WheelbrakeLeft: i32 = 70;  // < apply wheelbrake left
pub const OAPI_LKEY_WheelbrakeRight: i32 = 71;  // < apply wheelbrake right
pub const OAPI_LKEY_HUD: i32 = 72;  // < toggle HUD on/off
pub const OAPI_LKEY_HUDMode: i32 = 73;  // < switch through HUD modes
pub const OAPI_LKEY_HUDReference: i32 = 74;  // < query reference object for HUD display
pub const OAPI_LKEY_HUDTarget: i32 = 75;  // < query target object for HUD display
pub const OAPI_LKEY_HUDColour: i32 = 76;  // < switch through HUD colours
pub const OAPI_LKEY_IncSimSpeed: i32 = 77;  // < increase simulation speed x10
pub const OAPI_LKEY_DecSimSpeed: i32 = 78;  // < decrease simulation speed x0.1
pub const OAPI_LKEY_IncFOV: i32 = 79;  // < increment field of view
pub const OAPI_LKEY_DecFOV: i32 = 80;  // < decrement field of view
pub const OAPI_LKEY_StepIncFOV: i32 = 81;  // < increment field of view by 10 deg
pub const OAPI_LKEY_StepDecFOV: i32 = 82;  // < decrement field of view by 10 deg
pub const OAPI_LKEY_MainMenu: i32 = 83;  // < open main menu
pub const OAPI_LKEY_DlgHelp: i32 = 84;  // < open help dialog
pub const OAPI_LKEY_DlgCamera: i32 = 85;  // < open camera dialog
pub const OAPI_LKEY_DlgSimspeed: i32 = 86;  // < open simulation speed dialog
pub const OAPI_LKEY_DlgCustomCmd: i32 = 87;  // < open custom command dialog
pub const OAPI_LKEY_DlgVisHelper: i32 = 88;  // < open visual helper dialog
pub const OAPI_LKEY_DlgRecorder: i32 = 89;  // < open flight recorder dialog
pub const OAPI_LKEY_DlgInfo: i32 = 90;  // < open object info dialog
pub const OAPI_LKEY_DlgMap: i32 = 91;  // < open map dialog
pub const OAPI_LKEY_ToggleCamInternal: i32 = 92;  // < switch between cockpit and external camera
pub const OAPI_LKEY_ToggleTrackMode: i32 = 93;  // < switch between track camera modes
pub const OAPI_LKEY_TogglePanelMode: i32 = 94;  // < switch between cockpit modes
pub const OAPI_LKEY_TogglePlanetarium: i32 = 95;  // < toggle celestial marker display on/off
pub const OAPI_LKEY_ToggleRecPlay: i32 = 96;  // < toggle flight recorder/playback on/off
pub const OAPI_LKEY_Pause: i32 = 97;  // < toggle simulation pause on/off
pub const OAPI_LKEY_Quicksave: i32 = 98;  // < quick-save current simulation state
pub const OAPI_LKEY_Quit: i32 = 99;  // < quit simulation session
pub const OAPI_LKEY_DlgSelectVessel: i32 = 100;  // < open vessel selection dialog
pub const OAPI_LKEY_SelectPrevVessel: i32 = 101;  // < switch focus to previous vessel
pub const OAPI_LKEY_DlgCapture: i32 = 102;  // < open screen capture dialog
pub const LKEY_COUNT: i32 = 103;                  // < number of logical key definitions