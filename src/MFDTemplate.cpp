// ==============================================================
//                 ORBITER MODULE: DialogTemplate
//                  Part of the ORBITER SDK
//          Copyright (C) 2003-2016 Martin Schweiger
//                   All rights reserved
//
// MFDTemplate.cpp
//
// This module demonstrates how to build an Orbiter plugin which
// inserts a new MFD (multi-functional display) mode. The code
// is not very useful in itself, but it can be used as a starting
// point for your own MFD developments.
// ==============================================================

#define STRICT
#define ORBITER_MODULE
#include "windows.h"
#include "orbitersdk.h"

#include "wrapper.h"

// ==============================================================
// MFD class implementation

// Constructor
MFDTemplate::MFDTemplate(DWORD w, DWORD h, VESSEL *vessel, MFDBridge *mfd_bridge)
	: MFD2(w, h, vessel), rust_mfd_(*(mfd_bridge->mfd.into_raw()))
{
	font = oapiCreateFont (w/20, true, "Arial", FONT_NORMAL, 450);
	// Add MFD initialisation here
}

// Destructor
MFDTemplate::~MFDTemplate ()
{
	oapiReleaseFont (font);
	// Add MFD cleanup code here
}

// Return button labels
char *MFDTemplate::ButtonLabel (int bt)
{
	auto label = rust_mfd_.ButtonLabel(bt);
	return (char*)label.data();
	// The labels for the two buttons used by our MFD mode
	// static char *label[2] = {"UP", "DN"};
	// return (bt < 2 ? label[bt] : 0);
}


// Return button menus
int MFDTemplate::ButtonMenu (const MFDBUTTONMENU **menu) const
{
	// The menu descriptions for the two buttons
	static const MFDBUTTONMENU mnu[2] = {
		{"Move up", 0, '['},
		{"Move down", 0, ']'}
	};
	if (menu) *menu = mnu;
	return 2; // return the number of buttons used
}

bool MFDTemplate::ConsumeButton(int bt, int event) { 
	rust_mfd_.ConsumeButton(bt, event);
	return true;
}

// Repaint the MFD
bool MFDTemplate::Update (oapi::Sketchpad *skp)
{
	Title (skp, "MFD Template in Rust");
	// // Draws the MFD title

	// skp->SetFont (font);
	// skp->SetTextAlign (oapi::Sketchpad::CENTER, oapi::Sketchpad::BASELINE);
	// skp->SetTextColor (0x00FFFF);
	// skp->Text (W/2, H/2,"Display area", 12);
	// skp->Rectangle (W/4, H/4, (3*W)/4, (3*H)/4);

	// // Add MFD display routines here.
	// // Use the device context (hDC) for Windows GDI paint functions.
	OapiSketchpad sketchpad(skp);
	rust_mfd_.Update(sketchpad, W, H);
	return true;
}

