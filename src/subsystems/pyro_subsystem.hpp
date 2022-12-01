#pragma once
#include "../subsystem.hpp"
#include "../state.hpp"
class PyroSubsystem: public Subsystem {
    public:
        bool success();
        void apply_state(DerivedState &state);
};