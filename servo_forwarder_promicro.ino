// packet format [0xFE, unsigned(8bit)int, unsigned(8bit)int, 0xFF]
#include <Servo.h>

//#define DEBUG // turns on serial debug messages

Servo servoX;
Servo servoY;

int servoYPos = 90; // safe y pos is not zero
int servoXPos = 0;

int servoYPos_min = 50;
int servoXPos_min = 0;

int servoYPos_max = 120;
int servoXPos_max = 180; // confirmed with servo range test

unsigned long servoTimeout = millis();

int recievedServoPos[2];
  
void setup() { 
  Serial.begin(9600);
} 

void receivePos () {
	byte data[2]; // was char
	delay(1);
	Serial.readBytesUntil(0xFF, data, 2); // TODO: check this assumption is true before reading to array
	int tmp_servoYPos = (int)data[0];
	int tmp_servoXPos = (int)data[1];
	// do some input validation
	if (tmp_servoYPos >= servoYPos_min && tmp_servoYPos <= servoYPos_max) {
		#ifdef DEBUG 
			Serial.println("Y value passed validation"); 
		#endif
		servoYPos = tmp_servoYPos;
	} else {
		#ifdef DEBUG 
			Serial.print("Y value failed validation, was: ");
			Serial.println(tmp_servoYPos);
		#endif
	}
	
	if (tmp_servoXPos >= servoXPos_min && tmp_servoXPos <= servoXPos_max) {
		#ifdef DEBUG 
			Serial.println("X value passed validation");
		#endif
		servoXPos = tmp_servoXPos;
	} else {
		#ifdef DEBUG 
			Serial.print("X value failed validation, was: ");
			Serial.println(tmp_servoXPos);
		#endif
	}
	servoSet();
	#ifdef DEBUG 
		servoDebug();
	#endif
}

void servoSet() {
    servoX.attach(9);
    servoY.attach(10);
  
    servoX.write(servoXPos);
    servoY.write(servoYPos);
}

void servoDebug() {
    Serial.print("Servo y rotation: ");
    Serial.print(" Got: ");
    Serial.println(servoYPos);
    Serial.print("Servo x rotation: ");
    Serial.print(" Got: ");
    Serial.println(servoXPos);
}

void loop() {
  if ( Serial.available() > 0 ) { // if there is data waiting in buffer
    byte firstbyte = Serial.read(); //read a byte
    if (firstbyte == 0xFE) { // if initial expected character found
	#ifdef DEBUG 
		Serial.println("Found Start Char");
	#endif
    delay(1);
    receivePos();
    servoTimeout = millis(); // reset servo timeout
    }
  }

  if ( (millis()-servoTimeout) > 1000) { // gives dodgey servos that twitch a break
    servoX.detach();
    servoY.detach();
  }
} 
