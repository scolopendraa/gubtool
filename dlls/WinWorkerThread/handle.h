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

bool HandleRequest(char *buffer, SOCKET sock, const sockaddr_in &sender, int senderLen);

bool SendConfirmation(SOCKET sock, const sockaddr_in &sender, int senderLen);

void CallNullaryFunction(char *buffer);

void CallParameterizedFunction(char *buffer);

void LoadLibrary(char *buffer);
