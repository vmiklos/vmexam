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
// Nev: Vajna Miklos
// Neptun: AYU9RZ
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
	float m[16];

	void Clear() {
		memset(&m[0], 0, sizeof(m));
	}

	void LoadIdentify() {
		Clear();
		m[0] = m[5] = m[10] = m[15] = 1;
	}

	Vector operator*(const Vector& v) {
		float Xh =  m[0] * v.x +  m[1] * v.y +  m[2] * v.z +  m[3];
		float Yh =  m[4] * v.x +  m[5] * v.y +  m[6] * v.z +  m[7];
		float Zh =  m[8] * v.x +  m[9] * v.y + m[10] * v.z + m[11];
		float  h = m[12] * v.x + m[13] * v.y + m[14] * v.z + m[15];

		return Vector(Xh/h, Yh/h, Zh/h);
	}

	float *GetArray() {
		return &m[0];
	}

	Matrix Transpose() {
		Matrix result;
		for (int i = 0; i < 4; i++)
			for (int j = 0; j < 4; j++)
				result.m[j * 4 + i] = m[i * 4 + j];
		return result;
	}
};

enum {
	NOOP = 0,
	SCALE,
	ROTATE,
	SHIFT
};

int trans_state = NOOP;

enum {
	OPENGL = 0,
	MANUAL
};

int matrix_state = MANUAL;

// csak mert math.ht nemszabad ;-/
# define M_PI           3.14159265358979323846

const Vector* points[2][13];

Matrix* transs[4];

void CatmullClark(const Vector** p, int len) {
	for (int i = 1; i < len; i += 2) {
		p[i] = new Vector(((Vector)*p[i-1]) * 0.5 + ((Vector)*p[i+1]) * 0.5);
	}
	for (int i = 2; i + 2 < len; i += 2) {
		p[i] = new Vector(((Vector)*p[i-1]) * 0.25 + ((Vector)*p[i+1]) * 0.25 + ((Vector)*p[i]) * 0.5);
	}
}

void onInitialization( ) {
	points[0][0] = new Vector(160, 20, 0);
	points[0][2] = new Vector(250, 80, 0);
	points[0][4] = new Vector(270, 20, 0);
	points[0][6] = new Vector(360, 80, 0);
	points[0][8] = new Vector(390, 20, 0);
	points[0][10] = new Vector(470, 80, 0);
	points[0][12] = new Vector(490, 20, 0);
	CatmullClark(points[0], ARRAY_SIZE(points[0]));

	points[1][0] = new Vector(160, 120, 0);
	points[1][2] = new Vector(250, 180, 0);
	points[1][4] = new Vector(270, 120, 0);
	points[1][6] = new Vector(360, 180, 0);
	points[1][8] = new Vector(390, 120, 0);
	points[1][10] = new Vector(470, 180, 0);
	points[1][12] = new Vector(490, 120, 0);
	CatmullClark(points[1], ARRAY_SIZE(points[1]));

	/*
	 * 1 0 0 0
	 * 0 1 0 0
	 * 0 0 1 0
	 * 0 0 0 1
	 */
	transs[NOOP] = new Matrix();
	transs[NOOP]->LoadIdentify();

	/*
	 * 0.5   0   0   0
	 *   0 0.5   0   0
	 *   0   0 0.5   0
	 *   0   0   0   1
	 */
	transs[SCALE] = new Matrix();
	transs[SCALE]->LoadIdentify();
	transs[SCALE]->m[0] = 0.5;
	transs[SCALE]->m[5] = 0.5;
	transs[SCALE]->m[10] = 0.5;

	/*
	 *  cos  sin    0    0
	 * -sin  cos    0    0
	 *    0    0    1    0
	 *    0    0    0    1
	 */
	float angle = M_PI/4;
	transs[ROTATE] = new Matrix();
	transs[ROTATE]->LoadIdentify();
	transs[ROTATE]->m[0] = cosf(angle);
	transs[ROTATE]->m[1] = -sinf(angle);
	transs[ROTATE]->m[4] = sinf(angle);
	transs[ROTATE]->m[5] = cosf(angle);

	/*
	 *  1  0  0  0
	 *  0  1  0  0
	 *  0  0  1  0
	 * px py pz  1
	 */
	transs[SHIFT] = new Matrix();
	transs[SHIFT]->LoadIdentify();
	transs[SHIFT]->m[7] = 1.2f;
}

void onDisplay( ) {
	glClearColor(0.1f, 0.2f, 0.3f, 1.0f);
	glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
	glColor4d(0.9f, 0.8f, 0.7f, 1.0f);

	glMatrixMode(GL_MODELVIEW);
	if (matrix_state == MANUAL)
		glLoadIdentity();
	else
		glLoadMatrixf(transs[trans_state]->Transpose().GetArray());
	gluOrtho2D(0., 600., 0., 600.);
	for (int i = 0; i < ARRAY_SIZE(points); i++) {
		glBegin(GL_LINE_STRIP);
		for (int j = 0; j < ARRAY_SIZE(points[i]); j++) {
			if (matrix_state == MANUAL) {
				Vector v = *transs[trans_state] * *points[i][j];
				glVertex2d(v.x, v.y);
			} else {
				Vector v = *points[i][j];
				glVertex2d(v.x, v.y);
			}
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
	if (key != 's' && key != 'S')
		return;
	if (key == 's')
		matrix_state = MANUAL;
	else
		matrix_state = OPENGL;
	trans_state = (trans_state + 1) % 4;
	onDisplay();
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
