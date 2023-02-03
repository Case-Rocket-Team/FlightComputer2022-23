#pragma once

#include "prelude.hpp"

using namespace Eigen;

const f32 baroErr = 20;
const f32 accelErr = 0.1;

const int stateLength = 2;
typedef Vector<f32, stateLength> StateVector;

const int outputLength = 2;
typedef Vector<f32, outputLength> OutputVector;

typedef Matrix<f32, stateLength, stateLength> StateSquareMatrix;

const int inputLength = 2;
typedef Vector<f32, inputLength> InputVector;

void iterate() {

}