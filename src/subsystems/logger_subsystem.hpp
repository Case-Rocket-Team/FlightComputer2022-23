#pragma once
#include "logger_subsystem.cpp"
#include "../subsystem.hpp"
class LoggerSubsystem: public Subsystem {
    public:
        bool success();
};