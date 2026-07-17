#include <iostream>
#include <vector>
#include <string>

struct Packet {
    std::string source;
    std::string destination;
    std::vector<uint8_t> payload;
};

extern "C" void analyze_packet(const Packet& packet) {
    std::cout << "[NetworkTools] Packet from " 
              << packet.source << " to " 
              << packet.destination
              << " (" << packet.payload.size() << " bytes)" << std::endl;
}

extern "C" int calculate_latency(int ping_ms, int jitter_ms) {
    return ping_ms + jitter_ms / 2;
}