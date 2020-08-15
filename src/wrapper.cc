#include "wrapper.h"
#include <memory>

using std::unique_ptr;

void debugLog(rust::Str s)
{
    sprintf(oapiDebugString(), s.data());
}

static int g_MFDmode; // identifier for new MFD mode
static char mfdName[256];

static MFDTemplate* g_MFD = nullptr;

static MFDBridge* g_mfd_bridge;

int MsgProc(UINT msg, UINT mfd, WPARAM wparam, LPARAM lparam) 
{
    switch (msg)
    {
    case OAPI_MSG_MFD_OPENED:
        // Our new MFD mode has been selected, so we create the MFD and
        // return a pointer to it.
        g_MFD = new MFDTemplate(LOWORD(wparam), HIWORD(wparam), (VESSEL *)lparam, g_mfd_bridge);
        return (int)(g_MFD);
    }
    return 0;
}

void InitModuleSpec(rust::Str name, unsigned int key, rust::Box<MFDBridge> mfd_bridge)
{
    g_mfd_bridge = mfd_bridge.into_raw();
    strcpy_s(mfdName, name.data());
    mfdName[name.length()] = '\0';
    MFDMODESPECEX spec;
    spec.name = mfdName;
    spec.key = key;                       // MFD mode selection key
    spec.context = g_mfd_bridge;
    spec.msgproc = MsgProc;               // MFD mode callback function
    g_MFDmode = oapiRegisterMFDMode(spec);
}

void ExitModuleSpec()
{
    oapiUnregisterMFDMode(g_MFDmode);
}