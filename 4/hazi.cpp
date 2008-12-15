//========================================================
// Hazi feladat keret.
// A //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// sorokon beluli reszben celszeru garazdalkodni, mert
// a tobbit ugyis toroljuk.
// A Hazi feladat csak ebben a fajlban lehet
// Tilos:
// - mast "beincludolni", illetve mas konyvtarat hasznalni
// - faljmuveleteket vegezni
//========================================================

#include <math.h>
#include <stdlib.h>

#if defined(WIN32) || defined(_WIN32) || defined(__WIN32__)
// MsWindows-on ez is kell
#include <windows.h>
#endif // Win32 platform

#include <GL/gl.h>
#include <GL/glu.h>
// A GLUT-ot le kell tolteni: http://www.opengl.org/resources/libraries/glut/
#include <GL/glut.h>

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Innentol modosithatod...

//--------------------------------------------------------
// Nev: Vajna Miklos
// Neptun: AYU9RZ
//--------------------------------------------------------

#include <stdio.h>

float zoom = 3;
// 0 milyen sugaru kornyezeteben valtozzon a state
int barrier = 2;

int prevtime = 0;
int difftime = 0;

// gimpbol exportalva
unsigned char	 pixel_data[] = {
  "\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0"
  "\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0"
  "\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0"
  "\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\214\0\0\205\0"
  "\0z\0\0w\0\0w\0\0\202\0\0\211\0\0\220\0\0\221\0\0\221\0\0\221\0\0\221\0\0"
  "\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0"
  "\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0"
  "\221\0\0\214\0\0u\0\0k\0\0_\0\0]\0\0]\0\0g\0\0o\0\0\205\0\0\220\0\0\221\0"
  "\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0"
  "\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0"
  "\0\221\0\0\221\0\0\214\0\0u\0\0a\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0m\0\0\205"
  "\0\0\220\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221"
  "\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221"
  "\0\0\221\0\0\221\0\0\221\0\0\201\0\0c\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]"
  "\0\0]\0\0w\0\0\215\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221"
  "\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221"
  "\0\0\221\0\0\221\0\0\221\0\0\221\0\0r\0\0_\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]"
  "\0\0]\0\0]\0\0g\0\0\211\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0"
  "\0\221\0\0\214\0\0\205\0\0z\0\0w\0\0w\0\0\202\0\0\211\0\0\220\0\0\221\0\0"
  "\221\0\0\221\0\0\221\0\0\221\0\0g\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0"
  "]\0\0]\0\0]\0\0\207\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\214"
  "\0\0u\0\0k\0\0_\0\0]\0\0]\0\0g\0\0o\0\0\205\0\0\220\0\0\221\0\0\221\0\0\221"
  "\0\0\221\0\0g\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0\207\0"
  "\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\214\0\0u\0\0a\0\0]\0\0]\0\0]\0"
  "\0]\0\0]\0\0]\0\0m\0\0\205\0\0\220\0\0\221\0\0\221\0\0\221\0\0r\0\0_\0\0"
  "]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0g\0\0\211\0\0\221\0\0\221\0\0\221"
  "\0\0\221\0\0\221\0\0\201\0\0c\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0"
  "w\0\0\215\0\0\221\0\0\221\0\0\221\0\0\201\0\0c\0\0]\0\0]\0\0]\0\0]\0\0]\0"
  "\0]\0\0]\0\0]\0\0w\0\0\215\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0r\0"
  "\0_\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0g\0\0\211\0\0\221\0\0\221"
  "\0\0\221\0\0\214\0\0u\0\0a\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0m\0\0\205\0\0"
  "\220\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0g\0\0]\0\0]\0\0]\0\0]\0\0"
  "]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0\207\0\0\221\0\0\221\0\0\221\0\0\221\0\0\214"
  "\0\0u\0\0k\0\0_\0\0]\0\0]\0\0g\0\0o\0\0\205\0\0\220\0\0\221\0\0\221\0\0\221"
  "\0\0\221\0\0\221\0\0\221\0\0g\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0"
  "]\0\0]\0\0\207\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\214\0\0\205\0"
  "\0z\0\0w\0\0w\0\0\202\0\0\211\0\0\220\0\0\221\0\0\221\0\0\221\0\0\221\0\0"
  "\221\0\0\221\0\0\221\0\0r\0\0_\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0"
  "\0g\0\0\211\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221"
  "\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221"
  "\0\0\221\0\0\221\0\0\221\0\0\201\0\0c\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]"
  "\0\0]\0\0w\0\0\215\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221"
  "\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221"
  "\0\0\221\0\0\221\0\0\221\0\0\221\0\0\214\0\0u\0\0a\0\0]\0\0]\0\0]\0\0]\0"
  "\0]\0\0]\0\0m\0\0\205\0\0\220\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0"
  "\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0"
  "\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\214\0\0u\0\0k\0"
  "\0_\0\0]\0\0]\0\0g\0\0o\0\0\205\0\0\220\0\0\221\0\0\221\0\0\221\0\0\221\0"
  "\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0"
  "\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0"
  "\0\221\0\0\214\0\0\205\0\0z\0\0w\0\0w\0\0\202\0\0\211\0\0\220\0\0\221\0\0"
  "\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0"
  "\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0"
  "\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0"
  "\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0"
  "\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\217\0\0\202\0\0r\0\0]\0\0]\0\0b"
  "\0\0z\0\0\207\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0"
  "\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0"
  "\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\217\0\0|\0\0]\0"
  "\0]\0\0]\0\0]\0\0]\0\0]\0\0e\0\0\207\0\0\221\0\0\221\0\0\221\0\0\221\0\0"
  "\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0"
  "\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\217\0\0"
  "|\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0e\0\0\207\0\0\221\0\0\221\0"
  "\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0"
  "\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0"
  "\0\214\0\0r\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0b\0\0|\0\0\221\0\0"
  "\221\0\0\221\0\0\221\0\0\221\0\0\212\0\0}\0\0j\0\0]\0\0]\0\0j\0\0}\0\0\212"
  "\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\207\0\0]\0\0"
  "]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0g\0\0\221\0\0\221\0\0\221\0"
  "\0\221\0\0\212\0\0q\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0q\0\0\212\0\0\221\0"
  "\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\207\0\0]\0\0]\0\0]\0\0]\0\0]\0"
  "\0]\0\0]\0\0]\0\0]\0\0]\0\0g\0\0\221\0\0\221\0\0\221\0\0\212\0\0q\0\0]\0"
  "\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0q\0\0\212\0\0\221\0\0\221\0\0\221\0"
  "\0\221\0\0\221\0\0\207\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]"
  "\0\0g\0\0\221\0\0\221\0\0\221\0\0\204\0\0j\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]"
  "\0\0]\0\0]\0\0j\0\0\204\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\214\0"
  "\0r\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0b\0\0|\0\0\221\0\0\221\0\0"
  "\221\0\0w\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0w\0\0\221"
  "\0\0\221\0\0\221\0\0\221\0\0\221\0\0\217\0\0|\0\0]\0\0]\0\0]\0\0]\0\0]\0"
  "\0]\0\0]\0\0]\0\0e\0\0\207\0\0\221\0\0\221\0\0\221\0\0w\0\0]\0\0]\0\0]\0"
  "\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0w\0\0\221\0\0\221\0\0\221\0\0\221\0"
  "\0\221\0\0\221\0\0\217\0\0|\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0e\0\0\207\0"
  "\0\221\0\0\221\0\0\221\0\0\221\0\0w\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0"
  "\0]\0\0]\0\0]\0\0w\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221"
  "\0\0\217\0\0\202\0\0r\0\0]\0\0]\0\0b\0\0z\0\0\207\0\0\221\0\0\221\0\0\221"
  "\0\0\221\0\0\221\0\0\204\0\0j\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0"
  "j\0\0\204\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221"
  "\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221"
  "\0\0\221\0\0\221\0\0\221\0\0\212\0\0q\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]\0\0]"
  "\0\0]\0\0q\0\0\212\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221"
  "\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221"
  "\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\212\0\0q\0\0]\0\0]\0\0]\0\0"
  "]\0\0]\0\0]\0\0q\0\0\212\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221"
  "\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221"
  "\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\221\0\0\212"
  "\0\0}\0\0j\0\0]\0\0]\0\0j\0\0}\0\0\212\0\0\221\0\0\221\0\0\221\0\0\221\0",
};

unsigned int texture;
void onInitialization( ) {
	float al[] = {1.0, 1.0, 1.0, 1.0};
	float dl[] = {1.0, 1.0, 1.0, 1.0};
	float sl[] = {1.0, 1.0, 1.0, 1.0};
	float pos[] = {0.0, 10.0, 0.0, 0.0};
	glMatrixMode(GL_PROJECTION);
	gluPerspective(90, -1.0, 1.0 ,100.0);
	glMatrixMode(GL_MODELVIEW);
	gluLookAt(0, 2, 5,0,0,-10,0,1,0);
	glEnable(GL_LIGHTING);
	glLightfv(GL_LIGHT0,GL_AMBIENT,al);
	glLightfv(GL_LIGHT0,GL_DIFFUSE,dl);
	glLightfv(GL_LIGHT0,GL_SPECULAR,sl);
	glLightfv(GL_LIGHT0,GL_POSITION,pos);
	glEnable(GL_LIGHT0);
	glEnable(GL_DEPTH_TEST);
	glShadeModel(GL_SMOOTH);
	glGenTextures(1, &texture);
	glBindTexture (GL_TEXTURE_2D, texture);
	glPixelStorei (GL_UNPACK_ALIGNMENT, 1);
	glTexParameteri (GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);
	glTexParameteri (GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);
	glTexParameteri (GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
	glTexParameteri (GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
	glTexEnvf (GL_TEXTURE_ENV, GL_TEXTURE_ENV_MODE, GL_MODULATE);
	glTexImage2D (GL_TEXTURE_2D, 0, GL_RGB, 32, 32, 0, GL_RGB, GL_UNSIGNED_BYTE, pixel_data);
}

float labState() {
	static int swap = 1;
	static float state = -1*barrier;
	state += (float)difftime / 200;
	if (state > barrier){
		state -= 2*barrier;
		swap = swap == 1 ? -1 : 1;
	}
	//printf("debug, labState(), returning %f\n", state*swap);
	return state * swap;
}

float fejState() {
	static float state = -1*barrier;
	state += (float)difftime / 1000;
	if (state > barrier){
		state -= 2*barrier;
	}
	return -1*state;
}

void drawCsirke() {
	// csirke feje
	glPushMatrix();
	float yellow[] = {1.0, 1.0, 0.0, 1.0};
	glMaterialfv(GL_FRONT, GL_DIFFUSE, yellow);
	glTranslatef(-0.4*zoom, 0.75*zoom, 0.3*zoom);
	GLUquadric *quad = gluNewQuadric();
	gluSphere(quad, 0.1*zoom, 100, 100);
	glPopMatrix();
	// teste
	glPushMatrix();
	glTranslatef(-0.4*zoom, 0.2*zoom, 0.3*zoom);
	glScalef(0.5, 0.5, 1);
	glBegin(GL_TRIANGLES);
	glVertex3f(0, 1*zoom, 0);
	glVertex3f(1*zoom, 1*zoom, 0);
	glVertex3f(0.5*zoom, 0, 0);
	glEnd();
	glPopMatrix();
	// bal laba
	float labstate = labState();
	glPushMatrix();
	// az utolso parameter a lab melysege, a ket lab gyak ebben
	// kulonbozik (meg a szogben)
	glTranslatef(-0.15*zoom, 0.3*zoom, 0.2*zoom);
	glRotatef(45*labstate, 0, 1, 0);
	glRotatef(135, 1, 0, 0);
	GLUquadric *lab1 = gluNewQuadric();
	float red[] = {1.0, 0.0, 0.0, 1.0};
	float white[] = {1.0, 1.0, 1.0, 1.0};
	glMaterialf(GL_FRONT, GL_SHININESS, 20.0);
	glMaterialfv(GL_FRONT, GL_DIFFUSE, red);
	glMaterialfv(GL_FRONT, GL_SPECULAR, white);
	gluCylinder(lab1, 0.1, 0.2, 1, 100, 100);
	glPopMatrix();
	// jobb laba
	glPushMatrix();
	glTranslatef(-0.15*zoom, 0.3*zoom, 0.3*zoom);
	glRotatef(45*labstate, 0, 1, 0);
	glRotatef(45, 1, 0, 0);
	GLUquadric *lab2 = gluNewQuadric();
	gluCylinder(lab2, 0.1, 0.2, 1, 100, 100);
	glPopMatrix();
	// csor
	glPushMatrix();
	glTranslatef(-0.45*zoom, 0.75*zoom, 0.25*zoom);
	glRotatef(45, 1, 0, 0);
	glRotatef(-90, 0, 1, 0);
	GLUquadric *csor = gluNewQuadric();
	gluCylinder(csor, 0.2, 0.0, 0.5, 100, 100);
	glPopMatrix();

}

void onDisplay( ) {
	glClearColor(0.5f, 0.5f, 1.0f, 1.0f);
	glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

	// defaults
	float dd[] = {0.8, 0.8, 0.8, 1.0};
	glMaterialfv(GL_FRONT, GL_DIFFUSE, dd);
	float sd[] = {0.0, 0.0, 0.0, 1.0};
	glMaterialfv(GL_FRONT, GL_SPECULAR, sd);
	// terep
	glNormal3f(1.0, 1.0, 1.0);
	glEnable (GL_TEXTURE_2D);
	glBindTexture (GL_TEXTURE_2D, texture);
	glBegin(GL_QUADS);
	glTexCoord2f (0.0f,0.0f);
	glVertex3f(-3*zoom, 0, -1*zoom);
	glTexCoord2f (1.0f, 0.0f);
	glVertex3f(-3*zoom, 0, 0*zoom);
	glTexCoord2f (1.0f, 1.0f);
	glVertex3f(3*zoom, 0, 0*zoom);
	glTexCoord2f (0.0f, 1.0f);
	glVertex3f(3*zoom, 0, -1*zoom);

	glTexCoord2f (0.0f,0.0f);
	glVertex3f(-3*zoom, 0, 1*zoom);
	glTexCoord2f (1.0f, 0.0f);
	glVertex3f(-3*zoom, 0, 2*zoom);
	glTexCoord2f (1.0f, 1.0f);
	glVertex3f(3*zoom, 0, 2*zoom);
	glTexCoord2f (0.0f, 1.0f);
	glVertex3f(3*zoom, 0, 1*zoom);
	glEnd();
	glDisable (GL_TEXTURE_2D);
	// ut
	glBegin(GL_QUADS);
	float gray[] = {0.5, 0.5, 0.5, 1.0};
	glMaterialfv(GL_FRONT, GL_DIFFUSE, gray);
	glVertex3f(-3*zoom, 0, 0);
	glVertex3f(-3*zoom, 0, 1*zoom);
	glVertex3f(3*zoom, 0, 1*zoom);
	glVertex3f(3*zoom, 0, 0);
	glEnd();

	glPushMatrix();
	glTranslatef(fejState()*zoom, 0, 0);
	drawCsirke();
	glPopMatrix();

	// Buffercsere: rajzolas vege
	glFinish();
	glutSwapBuffers();
}

void onMouse(int button, int state, int x, int y) {
	// A GLUT_LEFT_BUTTON / GLUT_RIGHT_BUTTON
	// ill. a GLUT_DOWN / GLUT_UP makrokat hasznald.
}

void onIdle( ) {
	int curr = glutGet(GLUT_ELAPSED_TIME);
	difftime = curr - prevtime;
	glutPostRedisplay();
	prevtime = curr;
}

void onKeyboard(unsigned char key, int x, int y) {
	if (key == ' ') {
	}
}

// ...Idaig modosithatod
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

int main(int argc, char **argv) {
	glutInit(&argc, argv);
	glutInitWindowSize(600, 600);
	glutInitWindowPosition(100, 100);
	glutInitDisplayMode(GLUT_RGB | GLUT_DOUBLE | GLUT_DEPTH);

	glutCreateWindow("Grafika hazi feladat");

	glMatrixMode(GL_MODELVIEW);
	glLoadIdentity();
	glMatrixMode(GL_PROJECTION);
	glLoadIdentity();

	onInitialization();

	glutDisplayFunc(onDisplay);
	glutMouseFunc(onMouse);
	glutIdleFunc(onIdle);
	glutKeyboardFunc(onKeyboard);

	glutMainLoop();

	return 0;
}
