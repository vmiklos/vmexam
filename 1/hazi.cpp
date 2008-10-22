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
#else // g++ nem fordit a stanard include-ok nelkul :-/
#include <string.h>
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
	float x, y, z;

	Vector(float x0, float y0, float z0) {
		x = x0;
		y = y0;
		z = z0;
	}

	Vector operator+(const Vector& v) {
		return Vector(x + v.x, y + v.y, z + v.z);
	}

	Vector operator*(float f) {
		return Vector(x * f, y * f, z * f);
	}
};

class Matrix {
public:
	float m[4][4];

	void Clear() {
		memset(&m[0][0], 0, sizeof(m));
	}

	void LoadIdentify() {
		Clear();
		m[0][0] = m[1][1] = m[2][2] = m[3][3] = 1;
	}

	Vector operator*(const Vector& v) {
		float Xh = m[0][0] * v.x + m[0][1] * v.y + m[0][2] * v.z + m[0][3];
		float Yh = m[1][0] * v.x + m[1][1] * v.y + m[1][2] * v.z + m[1][3];
		float Zh = m[2][0] * v.x + m[2][1] * v.y + m[2][2] * v.z + m[2][3];
		float  h = m[3][0] * v.x + m[3][1] * v.y + m[3][2] * v.z + m[3][3];

		return Vector(Xh/h, Yh/h, Zh/h);
	}
};

enum {
	NOOP = 0,
	SCALE,
	ROTATE,
	SHIFT
};

// csak mert math.ht nemszabad ;-/
# define M_PI           3.14159265358979323846

const Vector* points[2][7];

Matrix* transs[3];

int trans_state = ROTATE;

void onInitialization( ) {
	points[0][0] = new Vector(10, 20, 0);
	points[0][1] = new Vector(100, 80, 0);
	points[0][2] = new Vector(120, 20, 0);
	points[0][3] = new Vector(210, 80, 0);
	points[0][4] = new Vector(230, 20, 0);
	points[0][5] = new Vector(320, 80, 0);
	points[0][6] = new Vector(340, 20, 0);

	points[1][0] = new Vector(10, 120, 0);
	points[1][1] = new Vector(100, 180, 0);
	points[1][2] = new Vector(120, 120, 0);
	points[1][3] = new Vector(210, 180, 0);
	points[1][4] = new Vector(230, 120, 0);
	points[1][5] = new Vector(320, 180, 0);
	points[1][6] = new Vector(340, 120, 0);

	transs[NOOP] = new Matrix();
	transs[NOOP]->LoadIdentify();

	transs[SCALE] = new Matrix();
	transs[SCALE]->LoadIdentify();
	transs[SCALE]->m[0][0] = 0.5;
	transs[SCALE]->m[1][1] = 0.5;
	transs[SCALE]->m[2][2] = 0.5;

	float angle = M_PI/4;
	transs[ROTATE] = new Matrix();
	transs[ROTATE]->LoadIdentify();
	transs[ROTATE]->m[0][0] = cosf(angle);
	transs[ROTATE]->m[0][1] = -sinf(angle);
	transs[ROTATE]->m[1][0] = sinf(angle);
	transs[ROTATE]->m[1][1] = cosf(angle);

	gluOrtho2D(0., 500., 0., 500.);
}

void onDisplay( ) {
	glClearColor(0.1f, 0.2f, 0.3f, 1.0f);
	glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
	glColor4d(0.9f, 0.8f, 0.7f, 1.0f);

	for (int i = 0; i < ARRAY_SIZE(points); i++) {
		glBegin(GL_LINE_STRIP);
		for (int j = 0; j < ARRAY_SIZE(points[i]); j++) {
			Vector v = *transs[trans_state] * *points[i][j];
			glVertex2d(v.x, v.y);
		}
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
