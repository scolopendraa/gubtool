#include "port.h"
#include <Psapi.h>
#include <processthreadsapi.h>

void WritePortAddress(u_short port)
{
    HMODULE hMain = GetModuleHandleW(nullptr);
    intptr_t moduleBase = reinterpret_cast<intptr_t>(hMain);

    wchar_t processName[MAX_PATH];
    GetModuleBaseNameW(GetCurrentProcess(), nullptr, processName, MAX_PATH);

    intptr_t address = 0;

    if (wcscmp(processName, L"eldenring.exe") == 0 || wcscmp(processName, L"start_protected_game.exe") == 0 ||
        wcscmp(processName, L"start_protected") == 0)
    {
        address = moduleBase + 0x4000000;
    }
    else if (wcscmp(processName, L"DarkSoulsII.exe") == 0)
    {
#if defined(_WIN64)
        address = moduleBase + 0x1800000;
#else
        address = moduleBase + 0x1250000;
#endif
    }

    *reinterpret_cast<u_short *>(address) = port;
}
