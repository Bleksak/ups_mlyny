#pragma once
#include <iostream>
#include <string>

namespace Validator {
    inline bool username(const std::string& username) {
        if(username.find('\n') != std::string::npos) {
            std::cout << "invalid username (contains newline)";
            return false;
        }
        
        if(username.empty()) {
            std::cout << "invalid username (empty)";
            return false;
        }
        
        return true;
    }
}
