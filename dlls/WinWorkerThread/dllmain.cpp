#include "handle.h"
#include "port.h"
#include <Windows.h>
#include <basetsd.h>
#include <winnt.h>
#include <winsock.h>
#pragma comment(lib, "WS2_32.lib")

DWORD WINAPI WorkerThread(LPVOID lpParameter)
{
    WSADATA wsa;

    if (WSAStartup(MAKEWORD(2, 2), &wsa) != 0)
    {
        return 1;
    }

    SOCKET sock = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);

    if (sock == INVALID_SOCKET)
    {
        WSACleanup();
        return 1;
    }

    sockaddr_in addr;
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    addr.sin_port = 0;

    if (bind(sock, reinterpret_cast<sockaddr *>(&addr), sizeof(addr)) == SOCKET_ERROR)
    {
        closesocket(sock);
        WSACleanup();
        return 1;
    }

    sockaddr_in boundAddr;
    int size = sizeof(boundAddr);

    if (getsockname(sock, reinterpret_cast<sockaddr *>(&boundAddr), &size) == SOCKET_ERROR)
    {
        closesocket(sock);
        return 1;
    }

    u_short port = ntohs(boundAddr.sin_port);

    WritePortAddress(port);

    char buffer[0x50];

    while (true)
    {
        sockaddr_in sender;
        int senderLen = sizeof(sender);

        int bytesReceived =
            recvfrom(sock, buffer, sizeof(buffer), 0, reinterpret_cast<sockaddr *>(&sender), &senderLen);

        if (bytesReceived == SOCKET_ERROR)
        {
            break;
        }

        if (HandleRequest(buffer, sock, sender, senderLen) == 0)
        {
            continue;
        }
    }

    return 1;
}

BOOL APIENTRY DllMain(HMODULE hModule, DWORD reason, LPVOID lpReserved)
{
    if (reason == DLL_PROCESS_ATTACH)
    {
        CreateThread(nullptr, 0, WorkerThread, nullptr, 0, nullptr);
    }
    return TRUE;
}
