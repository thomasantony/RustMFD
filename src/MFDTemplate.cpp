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
#include "src/lib.rs.h"

// ==============================================================
// MFD class implementation

// Constructor
MFDTemplate::MFDTemplate(DWORD w, DWORD h, VESSEL *vessel)
	: MFD2(w, h, vessel), rust_mfd_(* create_rust_mfd())
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
}

static MFDBUTTONMENU buttonMenuArray[12];
static std::string menuButtonLine1Array[12];
static std::string menuButtonLine2Array[12];

// Return button menus
int MFDTemplate::ButtonMenu (const MFDBUTTONMENU **menu) const
{
	memset(buttonMenuArray, 0, sizeof(buttonMenuArray));
	auto num_buttons = 0;
	for (auto i = 0; i < 12; i++)
	{
		auto menu_item = rust_mfd_.ButtonMenu(i);
		if (menu_item.selchar == 0)
		{
			num_buttons = i+1;
			break;
		}
		auto& line1 = menu_item.line1;
		auto& line2 = menu_item.line2;
		menuButtonLine1Array[i] = std::string(line1.data(), line1.length());
		menuButtonLine2Array[i] = std::string(line2.data(), line2.length());
		buttonMenuArray[i].line1 = line1.length() == 0 ? 0 : menuButtonLine1Array[i].c_str();
		buttonMenuArray[i].line2 = line2.length() == 0 ? 0 : menuButtonLine2Array[i].c_str();
		buttonMenuArray[i].selchar = (char)menu_item.selchar;
	}

	if (menu) *menu = buttonMenuArray;
	return num_buttons; // return the number of buttons used
}

bool MFDTemplate::ConsumeButton(int bt, int event) { 
	rust_mfd_.ConsumeButton(bt, event);
	return true;
}

// Repaint the MFD
bool MFDTemplate::Update (oapi::Sketchpad *skp)
{
	auto title = rust_mfd_.Title();
	std::string s_title(title.data(), title.length());
	// Draws the MFD title
	Title (skp, s_title.c_str());

	OapiSketchpad sketchpad(skp);
	rust_mfd_.Update(sketchpad, W, H);
	return true;
}

