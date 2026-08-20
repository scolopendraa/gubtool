#include "handle.h"
#include "ffi.h"
#include <Windows.h>
#include <cstdint>
#include <cstring>
#include <libloaderapi.h>
#include <vector>

bool HandleRequest(char *buffer, SOCKET sock, const sockaddr_in &sender, int senderLen)
{
    std::uint8_t byte = buffer[0];
    Request request = static_cast<Request>(byte);

    switch (request)
    {
    case Request::Handshake:
        return SendConfirmation(sock, sender, senderLen);
    case Request::NullaryFunction:
        CallNullaryFunction(buffer);
        return SendConfirmation(sock, sender, senderLen);
    case Request::ParameterizedFunction:
        CallParameterizedFunction(buffer);
        return SendConfirmation(sock, sender, senderLen);
    case Request::ThreadFunction:
        CallFunctionNewThread(buffer);
        return SendConfirmation(sock, sender, senderLen);
    case Request::LoadLibrary:
        LoadLibrary(buffer);
        return SendConfirmation(sock, sender, senderLen);
    }

    return 1;
}

bool SendConfirmation(SOCKET sock, const sockaddr_in &sender, int senderLen)
{
    const char response = 0x00;

    return sendto(sock, &response, sizeof(response), 0, reinterpret_cast<const sockaddr *>(&sender), senderLen) !=
           SOCKET_ERROR;
}

void CallNullaryFunction(char *buffer)
{
    uintptr_t address;
    std::memcpy(&address, buffer + 1, sizeof(address));
    using Function = void (*)();
    auto function = reinterpret_cast<Function>(address);
    function();
}

DWORD WINAPI ThreadProc(LPVOID param)
{
    CallNullaryFunction(static_cast<char *>(param));
    return 0;
}

void CallFunctionNewThread(char *buffer)
{
    HANDLE thread = CreateThread(nullptr, 0, ThreadProc, buffer, 0, nullptr);
    WaitForSingleObject(thread, INFINITE);
    CloseHandle(thread);
}

ffi_type *FfiTypeFromProtocolCode(std::uint8_t code)
{
    switch (code)
    {
    case 0:
        return &ffi_type_pointer;
    case 1:
        return &ffi_type_uint8;
    case 2:
        return &ffi_type_uint16;
    case 3:
        return &ffi_type_uint32;
    case 4:
        return &ffi_type_uint64;
    case 5:
        return &ffi_type_sint8;
    case 6:
        return &ffi_type_sint16;
    case 7:
        return &ffi_type_sint32;
    case 8:
        return &ffi_type_sint64;
    case 9:
        return &ffi_type_float;
    case 10:
        return &ffi_type_double;
    }

    std::abort();
}

#if !defined(_WIN64)
ffi_abi CallingConventionFromProtocolCode(std::uint8_t code)
{
    switch (code)
    {
    case 0:
        return FFI_MS_CDECL;
    case 1:
        return FFI_STDCALL;
    case 2:
        return FFI_FASTCALL;
    case 3:
        return FFI_THISCALL;
    }

    std::abort();
}
#endif

void CallParameterizedFunction(char *buffer)
{
    std::uint8_t length = buffer[1];

    uintptr_t function_address;
    std::memcpy(&function_address, buffer + 2, sizeof(function_address));

    std::vector<ffi_type *> types;
    std::vector<void *> args;

    for (int i = 0; i < length; i++)
    {
        int arg_base_index = 10 * (i + 1);
        std::uint8_t code = buffer[arg_base_index];
        types.push_back(FfiTypeFromProtocolCode(code));
        args.push_back(&buffer[arg_base_index + 2]);
    }

#if defined(_WIN64)
    ffi_abi calling_convention = FFI_DEFAULT_ABI;
#else
    std::size_t abi_code_index = 10 * (length + 1);
    std::uint8_t abi_code = buffer[abi_code_index];
    ffi_abi calling_convention = CallingConventionFromProtocolCode(abi_code);
#endif

    ffi_cif cif;

    ffi_prep_cif(&cif, calling_convention, static_cast<unsigned>(types.size()), &ffi_type_void, types.data());

    auto function = reinterpret_cast<void (*)()>(function_address);

    ffi_call(&cif, function, nullptr, args.data());
}

void LoadLibrary(char *buffer)
{
    uintptr_t path_address;
    std::memcpy(&path_address, buffer + 1, sizeof(path_address));
    LoadLibraryW(reinterpret_cast<const wchar_t *>(path_address));
}
