#include "wrapper.h"
// #include "src/lib.rs.h"
#include "MFDTemplate.h"

void debugLog(rust::Str s)
{
    sprintf(oapiDebugString(), s.data());
}

static int g_MFDmode; // identifier for new MFD mode

void InitModuleSpec()
{
    static char *name = "Rust MFD";   // MFD mode name
    MFDMODESPECEX spec;
    spec.name = name;
    spec.key = OAPI_KEY_T;                // MFD mode selection key
    spec.context = NULL;
    spec.msgproc = MFDTemplate::MsgProc;  // MFD mode callback function
    g_MFDmode = oapiRegisterMFDMode(spec);
}

void ExitModuleSpec()
{
    oapiUnregisterMFDMode(g_MFDmode);
}