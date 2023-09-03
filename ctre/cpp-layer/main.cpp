#include "ctre/phoenix/motorcontrol/can/VictorSPX.h"
#include <ctre/phoenix/motorcontrol/ControlMode.h>
#include <memory>

int main() {
  ctre::phoenix::motorcontrol::can::VictorSPX *motor =
      new ctre::phoenix::motorcontrol::can::VictorSPX(1);
  return 0;
}

std::unique_ptr<ctre::phoenix::motorcontrol::can::VictorSPX>
new_VictorSPX(int id) {
  return std::make_unique<ctre::phoenix::motorcontrol::can::VictorSPX>(id);
}
