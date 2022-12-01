#pragma once

#include "state.hpp"

class Subsystem {
    public:
        // whether the subsytem applied the state
        virtual bool success() { };
        //acts based on the derived state
        virtual void apply_state(DerivedState &state) { };
};