#pragma once

#include "state.hpp"

class InstrumentManager {
    public:
        bool success(){
            // TODO
        }

        /// @brief polls data and saves to internal state
        /// by reading from instruments
        void poll() {
            // TODO
        }

        /// @brief WRITES data into derivedState from internal
        /// instruments
        /// @param derived the derived state
        void derive_state(DerivedState& state) {
            // TODO
        }
};