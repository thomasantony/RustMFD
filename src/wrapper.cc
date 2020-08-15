#include "wrapper.h"
#include "MFDTemplate.h"


void debugLog(rust::Str s)
{
    sprintf(oapiDebugString(), s.data());
}

static int g_MFDmode; // identifier for new MFD mode
static char mfdName[256];

typedef int(*msgproc)(UINT, UINT, WPARAM, LPARAM);
void InitModuleSpec(rust::Str name, unsigned int key)
{
    strcpy_s(mfdName, name.data());
    mfdName[name.length()] = '\0';
    MFDMODESPECEX spec;
    spec.name = mfdName;
    spec.key = OAPI_KEY_T;                // MFD mode selection key
    spec.context = NULL;
    spec.msgproc = MFDTemplate::MsgProc;  // MFD mode callback function
    g_MFDmode = oapiRegisterMFDMode(spec);
}

void ExitModuleSpec()
{
    oapiUnregisterMFDMode(g_MFDmode);
}