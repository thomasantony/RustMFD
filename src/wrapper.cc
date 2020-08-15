#include "wrapper.h"
#include "src/lib.rs.h"
#include <memory>
#include <cstring>

using std::unique_ptr;

void debugLog(rust::Str s)
{
    std::string _s(s.data(), s.length());
    sprintf(oapiDebugString(), _s.c_str());
}

static int g_MFDmode; // identifier for new MFD mode
static char mfdName[256];

static MFDTemplate* g_MFD = nullptr;

int MsgProc(UINT msg, UINT mfd, WPARAM wparam, LPARAM lparam) 
{
    switch (msg)
    {
    case OAPI_MSG_MFD_OPENED:
        // Our new MFD mode has been selected, so we create the MFD and
        // return a pointer to it.
        g_MFD = new MFDTemplate(LOWORD(wparam), HIWORD(wparam), (VESSEL *)lparam);
        return (int)(g_MFD);
    }
    return 0;
}

void InitModuleSpec(rust::Str name, unsigned int key)
{
    std::strncpy(mfdName, name.data(), name.length());
    mfdName[name.length()] = '\0';
    MFDMODESPECEX spec;
    spec.name = mfdName;
    spec.key = key;                       // MFD mode selection key
    spec.context = NULL;
    spec.msgproc = MsgProc;               // MFD mode callback function
    g_MFDmode = oapiRegisterMFDMode(spec);
}

void ExitModuleSpec()
{
    oapiUnregisterMFDMode(g_MFDmode);
}