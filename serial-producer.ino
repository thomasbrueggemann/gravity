#include <ResponsiveAnalogRead.h>

ResponsiveAnalogRead analogOne(A0, true);
ResponsiveAnalogRead analogTwo(A1, true);

float activityThreshold = 6.0;

void setup() {
    Serial.begin(9600);

    analogOne.setActivityThreshold(activityThreshold);
    analogTwo.setActivityThreshold(activityThreshold);
}

void loop() {
  analogOne.update();
  analogTwo.update();

  if(analogOne.hasChanged() || analogTwo.hasChanged()) 
  {
    sendSerialMessage(
      analogOne.getValue(),
      analogTwo.getValue()
    );
  }
  
  delay(50);
}

void sendSerialMessage(int pot1, int pot2) 
{
  int msgLen = 8;
  byte msg[msgLen];
  msg[0] = 71;
  msg[1] = 86;
  msg[2] = 84;
  msg[3] = 89;

  // pot 1
  msg[4] = (byte) (pot1 & 0xff);
  msg[5] = (byte) ((pot1 >> 8) & 0xff);

  // pot 2
  msg[6] = (byte) (pot2 & 0xff);
  msg[7] = (byte) ((pot2 >> 8) & 0xff);

  Serial.write(msg, msgLen);
}
