#include <cstdint>
#include <ffi.h>
#include <winsock.h>

enum class Request : std::uint8_t
{
    Handshake = 0,
    NullaryFunction = 1,
    ParameterizedFunction = 2,
    LoadLibrary = 3,
};

enum class FunctionArg : std::uint8_t
{
    uintptr_t = 0,
    uint8_t = 1,
    uint16_t = 2,
    uint32_t = 3,
    uint64_t = 4,
    int8_t = 5,
    int16_t = 6,
    int32_t = 7,
    int64_t = 8,
    float_t = 9,
    double_t = 10,
};

bool HandleRequest(char *buffer, SOCKET sock, const sockaddr_in &sender, int senderLen);

bool SendConfirmation(SOCKET sock, const sockaddr_in &sender, int senderLen);

void CallNullaryFunction(char *buffer);

void CallParameterizedFunction(char *buffer);

void LoadLibrary(char *buffer);
