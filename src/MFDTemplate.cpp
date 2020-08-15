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
MFDTemplate::MFDTemplate(DWORD w, DWORD h, VESSEL *vessel)
	: MFD2(w, h, vessel), rust_mfd_(create_rust_mfd())
{
	RustMFD *a = create_rust_mfd();
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
	auto label = rust_mfd_->ButtonLabel(bt);
	return (char*)label.data();
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
	rust_mfd_->ConsumeButton(bt, event);
	return true;
}

// Repaint the MFD
bool MFDTemplate::Update (oapi::Sketchpad *skp)
{
	// auto title = rust_mfd_.Title();
	// std::string s_title(title.data(), title.length());
	// // Draws the MFD title
	// Title (skp, s_title.c_str());

	OapiSketchpad sketchpad(skp);
	rust_mfd_->Update(sketchpad, W, H);
	return true;
}

