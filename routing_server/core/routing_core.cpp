#include <iostream>
#include <string>
#include <vector>
#include <openssl/evp.h>

extern "C" std::string encrypt_data(const std::string& data) {
    EVP_MD_CTX* ctx = EVP_MD_CTX_new();
    unsigned char hash[32];
    EVP_DigestInit_ex(ctx, EVP_sha256(), NULL);
    EVP_DigestUpdate(ctx, data.c_str(), data.size());
    EVP_DigestFinal_ex(ctx, hash, NULL);
    EVP_MD_CTX_free(ctx);

    return std::string((char*)hash, 32);
}

extern "C" void process_packet(const std::string& packet) {
    std::cout << "[Routing Core] Packet processed: " << packet << std::endl;
}