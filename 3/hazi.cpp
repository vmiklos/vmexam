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

float zoom = 3;
void onInitialization( ) {
	//float al[] = {0.1, 0.1, 0.1, 1.0};
	//float dl[] = {0.5, 0.2, 0.3, 1.0};
	glMatrixMode(GL_PROJECTION);
	gluPerspective(90, -1.0, 1.0 ,100.0);
	glMatrixMode(GL_MODELVIEW);
	gluLookAt(0, 2, 5,0,0,-10,0,1,0);
	glEnable(GL_LIGHTING);
	//glLightfv(GL_LIGHT0,GL_AMBIENT,al);
	//glLightfv(GL_LIGHT0,GL_DIFFUSE,dl);
	// glLightfv(GL_LIGHT0,GL_SPECULAR,sl);
	// glLightfv(GL_LIGHT0,GL_POSITION,pos);
	glEnable(GL_LIGHT0);
	glEnable(GL_DEPTH_TEST);
	glShadeModel(GL_SMOOTH);
}

void onDisplay( ) {
	glClearColor(0.5f, 0.5f, 1.0f, 1.0f);
	//glClearColor(0.1f, 0.2f, 0.3f, 1.0f);
	glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

	/*glColor3f(1.0f, 1.0f, 1.0f);
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
	glEnd();*/
	// terep
	float green[] = {0.0, 1.0, 0.0, 1.0};
	glBegin(GL_QUADS);
	glMaterialfv(GL_FRONT, GL_DIFFUSE, green);
	glVertex3f(-3*zoom, 0, -1*zoom);
	glVertex3f(-3*zoom, 0, 2*zoom);
	glVertex3f(3*zoom, 0, 2*zoom);
	glVertex3f(3*zoom, 0, -1*zoom);
	glEnd();
	// ut
	glBegin(GL_QUADS);
	float gray[] = {0.5, 0.5, 0.5, 1.0};
	glMaterialfv(GL_FRONT, GL_DIFFUSE, gray);
	glVertex3f(-3*zoom, 0, 0);
	glVertex3f(-3*zoom, 0, 1*zoom);
	glVertex3f(3*zoom, 0, 1*zoom);
	glVertex3f(3*zoom, 0, 0);
	glEnd();

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
