#pragma once

#include "subsystem.hpp"
#include "subsystems/pyro_subsystem.hpp"
#include "state.hpp"

class SubsystemManager {
    public:
        Subsystem motors;
        PyroSubsystem pyros;
        Subsystem logger;
        bool success() {
            return motors.success()
                    & pyros.success()
                    & logger.success();
        }

        /// @brief Drives the motors, Drives the pyros, & Communicates with ground
        /// @param state the derived state
        /// @return true if state was sucessfully applied, otherwise false
        void apply(DerivedState &state) {
            motors.apply_state(state);
            pyros.apply_state(state);
            logger.apply_state(state);
        }
};