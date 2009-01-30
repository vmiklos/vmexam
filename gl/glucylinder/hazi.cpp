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
void onInitialization( ) {
	glMatrixMode(GL_PROJECTION);
	gluPerspective(90, 1.0, 1.0 ,100.0);
	glMatrixMode(GL_MODELVIEW);
	gluLookAt(0, 2, 5,0,0,-10,0,1,0);
	glEnable(GL_LIGHTING);
	glEnable(GL_LIGHT0);
	glEnable(GL_DEPTH_TEST);
	glEnable(GL_NORMALIZE);
	glShadeModel(GL_SMOOTH);
}

class mygluCylinder {
	public:
		float base, top, height, sx, sy; // hogy mekkora legyen a felbontas finomsaga
		mygluCylinder(float base0, float top0, float height0, float sx0, float sy0) {
			base = base0;
			top = top0;
			height = height0;
			sx = sx0;
			sy = sy0;
			draw();
		}

		float getx(float u, float v) {
			return ((top-base)*v+base) * cosf(2 * M_PI * u);
		}

		float gety(float u, float v) {
			return ((top-base)*v+base) * sinf(2 * M_PI * u);
		}

		float getz(float u, float v) {
			return v * height;
		}

		void draw() {
			for (float i = 0; i < sx; i++) {
				for (float j = 0; j < sy; j++) {
					glBegin(GL_QUADS);
					glNormal3f(getx((i+0.5)/sx,(j+0.5)/sy),
							gety((i+0.5)/sx, (j+0.5)/sy),
							getz((i+0.5)/sx,(j+0.5)/sy));
					glVertex3f(getx(i/sx, j/sy),
							gety(i/sx, j/sy),
							getz(i/sx, j/sy));
					glVertex3f(getx((i+1)/sx, j/sy),
							gety((i+1)/sx, j/sy),
							getz((i+1)/sx, j/sy));
					glVertex3f(getx((i+1)/sx, (j+1)/sy),
							gety((i+1)/sx, (j+1)/sy),
							getz((i+1)/sx, (j+1)/sy));
					glVertex3f(getx(i/sx, (j+1)/sy),
							gety(i/sx, (j+1)/sy),
							getz(i/sx, (j+1)/sy));
					glEnd();
				}
			}
		}
};

void onDisplay( ) {
	glClearColor(0.1f, 0.2f, 0.3f, 1.0f);
	glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

	glColor3f(1.0f, 1.0f, 1.0f);
	glBegin(GL_LINES);
	for (int i = -3; i < 3; i++) {
		for (int j = -3; j <= 3; j++) {
			glVertex3f(i*zoom, 0, j*zoom);
			glVertex3f((i+1)*zoom, 0, j*zoom);
		}
	}
	for (int i = -3; i < 3; i++) {
		for (int j = -3; j <= 3; j++) {
			glVertex3f(j*zoom, 0, i*zoom);
			glVertex3f(j*zoom, 0, (i+1)*zoom);
		}
	}
	glEnd();

	mygluCylinder mgc(0.5, 1., 3., 100., 100.);
	//GLUquadric *quad = gluNewQuadric();
	//gluCylinder(quad, 0.5, 1, 3, 100., 100.);

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
