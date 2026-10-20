#pragma once

#include <source_location>
#include <stdexcept>
#include <string>

// Fails the fixture test with the client's file and line when `condition` is false.
inline void check(bool condition, std::source_location const location = std::source_location::current())
{
    if (!condition)
    {
        throw std::runtime_error(std::string(location.file_name()) + "(" + std::to_string(location.line()) + "): check failed");
    }
}
