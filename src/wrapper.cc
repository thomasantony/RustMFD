#include "wrapper.h"
#include "MFDTemplate.h"
#include <memory>

using std::unique_ptr;

void debugLog(rust::Str s)
{
    sprintf(oapiDebugString(), s.data());
}

static int g_MFDmode; // identifier for new MFD mode
static char mfdName[256];

static unique_ptr<MFDTemplate> g_MFD;

int MsgProc(UINT msg, UINT mfd, WPARAM wparam, LPARAM lparam) 
{
    if (g_MFD.get() == nullptr)
    {
        g_MFD = std::unique_ptr<MFDTemplate>(new MFDTemplate(LOWORD(wparam), HIWORD(wparam), (VESSEL *)lparam));
    }
    return (int)(g_MFD.get());
}

typedef int(*msgproc)(UINT, UINT, WPARAM, LPARAM);
void InitModuleSpec(rust::Str name, unsigned int key)
{
    strcpy_s(mfdName, name.data());
    mfdName[name.length()] = '\0';
    MFDMODESPECEX spec;
    spec.name = mfdName;
    spec.key = OAPI_KEY_T;                // MFD mode selection key
    spec.context = NULL;
    spec.msgproc = MsgProc;               // MFD mode callback function
    g_MFDmode = oapiRegisterMFDMode(spec);
}

void ExitModuleSpec()
{
    oapiUnregisterMFDMode(g_MFDmode);
}