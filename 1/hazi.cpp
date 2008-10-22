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
#include <stdio.h>

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Innentol modosithatod...

//--------------------------------------------------------
// Nev:		 
// Neptun:
//--------------------------------------------------------

#define ARRAY_SIZE(x) (sizeof(x)/sizeof(x[0]))

class Vector {
public:
	float x, y;

	Vector(float x0, float y0) {
		x = x0;
		y = y0;
	}

	Vector operator+(const Vector& v) {
		return Vector(x + v.x, y + v.y);
	}

	Vector operator*(float f) {
		return Vector(x * f, y * f);
	}
};

class Matrix {
public:
	float m[3][3];

	Vector operator*(const Vector& v) {
		float Xh = m[0][0] * v.x + m[0][1] * v.y + m[0][2];
		float Yh = m[1][0] * v.x + m[1][1] * v.y + m[1][2];
		float  h = m[2][0] * v.x + m[2][1] * v.y + m[2][2];

		return Vector(Xh/h, Yh/h);
	}
};

Vector* points[2][7];

void onInitialization( ) {
	points[0][0] = new Vector(10, 20);
	points[0][1] = new Vector(100, 80);
	points[0][2] = new Vector(120, 20);
	points[0][3] = new Vector(210, 80);
	points[0][4] = new Vector(230, 20);
	points[0][5] = new Vector(320, 80);
	points[0][6] = new Vector(340, 20);

	points[1][0] = new Vector(10, 120);
	points[1][1] = new Vector(100, 180);
	points[1][2] = new Vector(120, 120);
	points[1][3] = new Vector(210, 180);
	points[1][4] = new Vector(230, 120);
	points[1][5] = new Vector(320, 180);
	points[1][6] = new Vector(340, 120);

	gluOrtho2D(0., 500., 0., 500.);
}

void onDisplay( ) {
	glClearColor(0.1f, 0.2f, 0.3f, 1.0f);
	glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
	glColor4d(0.9f, 0.8f, 0.7f, 1.0f);

	for (int i = 0; i < ARRAY_SIZE(points); i++) {
		glBegin(GL_LINE_STRIP);
		for (int j = 0; j < ARRAY_SIZE(points[i]); j++)
			glVertex2d(points[i][j]->x, points[i][j]->y);
		glEnd();
	}

	// Buffercsere: rajzolas vege
	glFinish();
	glutSwapBuffers();
}

void onMouse(int button, int state, int x, int y) {
	// A GLUT_LEFT_BUTTON / GLUT_RIGHT_BUTTON
	// ill. a GLUT_DOWN / GLUT_UP makrokat hasznald.
}

void onIdle( ) {
}

void onKeyboard(unsigned char key, int x, int y) {
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
