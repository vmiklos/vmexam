/*
 *  kisfeladat1.cpp
 * 
 *  Copyright (c) 2007 by Miklos Vajna <vmiklos@frugalware.org>
 */

#include <GL/gl.h>
#include <GL/glu.h>
#include <GL/glut.h>
#include <stdio.h>
#include <math.h>
#include <limits.h>

#include <vector>
#include <iostream>
#include <fstream>
#include <string>

using namespace std;

struct Vector
{
	float x, y;

	Vector(float x0, float y0)
	{
		x = x0;
		y = y0;
	}
};

Vector** vectors[4];
int vsizes[4];

int dlevel = 0;

float divide(float a, float b)
{
	float ret = a/b;
	if(fpclassify(ret) == FP_INFINITE)
		ret = INT_MAX*isinf(ret);
	return ret;
}

int intersect(float x1, float y1, float x2, float y2, float u1, float v1, float u2, float v2, float *xi, float*yi)
{
	float b1 = divide((y2-y1),(x2-x1));
	float b2 = divide((v2-v1),(u2-u1));
	float a1 = (y1-b1*x1);
	float a2 = (v1-b2*u1);
	*xi = - divide((a1-a2),(b1-b2));
	*yi = (a1+b1 * *xi);

	// ez lenne a teljes ellenorzes ket szakasz eseten, viszont itt mi csak
	// a masodik szakaszra akarunk ellenorizni
	//if (((x1-*xi)*(*xi-x2))>=0 && ((u1-*xi)*(*xi-u2))>=0 && ((y1-*yi)*(*yi-y2))>=0 && ((v1-*yi)*(*yi-v2))>=0)
	if (((u1-*xi)*(*xi-u2))>=0 && ((v1-*yi)*(*yi-v2))>=0)
		return(0);
	else
		return(-1);
}

bool is_inside(float qx, float qy)
{
	int count = 0;
	// fektetunk egy vizszintes egyenest a kerdeses pontra
	float qx2 = qx + 1;
	float qy2 = qy;

	for(int i=0;i<vsizes[dlevel];i++)
	{
		int n;
		if(i+1 < vsizes[dlevel])
			n = i+1;
		else
			n = 0;
		float mx, my;
		if(intersect(qx, qy, qx2, qy2, (*vectors[dlevel][i]).x, (*vectors[dlevel][i]).y,
			(*vectors[dlevel][n]).x, (*vectors[dlevel][n]).y, &mx, &my) == 0 && mx > qx)
			count++;
	}

	return (count%2);
}

void ReDraw( ) { 
	glClearColor(0, 0, 1, 0);
	glClear(GL_COLOR_BUFFER_BIT);
	glColor3d( 1.0, 1.0, 1.0 );
	glBegin(GL_LINE_LOOP);
	// a dlevel mondja megy, hogy milyen szintu simitas kell, es a vsizes
	// tomb mondja meg, hogy hany pontot rajzolunk ezen a szinten
	for(int i=0;i<vsizes[dlevel];i++)
		// a pontok koordinataihoz pedig kozvetlenul hozzaferhetunk
		glVertex2d((*vectors[dlevel][i]).x, (*vectors[dlevel][i]).y);
	glEnd( );
	glFlush( );
}

void Mouse(int button, int state, int x, int y)
{
	if(state != GLUT_DOWN)
		return;

	// szamoljuk at a virtualis vilag koordinataiba
	float vx = float(x)/2;
	float vy = float(200-y)/2;

	if(!is_inside(vx, vy))
		// kivul kattintottak
		return;

	if(button == GLUT_LEFT_BUTTON)
	{
		if(dlevel == 3)
			// mar a maximumon vagyunk
			return;
		dlevel++;
		// kiszamoljuk, hogy az uj levelen hany pont lesz
		int size = (vsizes[dlevel-1]*2);
		if(vectors[dlevel] == NULL)
		{
			// ezen a levelen meg nem jartunk, ki kell szamolni a pontokat
			vectors[dlevel] = new Vector* [size];
			vsizes[dlevel] = size;
			for(int i=1;i<=size;i++)
			{
				int p = i/2-1;
				int n = i/2;
				if(i==size)
					n = 0;
				if(i%2==0)
				{
					// a paros (zold) pontokat szamoljuk ki
					// eloszor. atlagoljuk az elozo korben
					// felvett ket pont koordinatait
					float x = ((*vectors[dlevel-1][p]).x+(*vectors[dlevel-1][n]).x)/2;
					float y = ((*vectors[dlevel-1][p]).y+(*vectors[dlevel-1][n]).y)/2;
					vectors[dlevel][i-1] = new Vector(x,y);
				}
			}
			for(int i=1;i<=size;i++)
			{
				// ebben a masodik korben szamoljuk a paratlan
				// pontokat, mivel mostmar megvan a kovetkezo
				// paros pont erteke is minden paratlan ponthoz
				int pp = i/2;
				int p = i-2;
				int n = i;
				if(i==1)
				{
					// az elso vagy utolso pont spec eset
					p = vsizes[dlevel]-1;
				}
				else if(i==size)
					n = 0;
				if(i%2==1)
				{
					float x = 0.5*(*vectors[dlevel-1][pp]).x + 0.25*((*vectors[dlevel][p]).x + (*vectors[dlevel][n]).x);
					float y = 0.5*(*vectors[dlevel-1][pp]).y + 0.25*((*vectors[dlevel][p]).y + (*vectors[dlevel][n]).y);
					vectors[dlevel][i-1] = new Vector(x,y);
				}
			}
			// elkeszultunk a pontok kiszamitasaval
		}
	}
	else if(button == GLUT_RIGHT_BUTTON)
	{
		if(dlevel == 0)
			// ennel tovabb nem lehet csokkenteni a simitast
			return;
		dlevel--;
	}
	// vegul rajzoljuk ki azt amit alkottunk
	ReDraw();
}

main(int argc, char **argv)
{
	ifstream myfile ("pontok.txt");
	string line;

	// oprendszer ablak init glut modra
	glutInitWindowSize(200, 200);
	glutInitWindowPosition(100, 100);
	glutInit(&argc, argv);
	glutInitDisplayMode( GLUT_RGB );
	glutCreateWindow("kisfeladat1");
	// meret a kepernyon
	glViewport(0, 0, 200, 200);
	glMatrixMode(GL_MODELVIEW);    
	glLoadIdentity( );
	glMatrixMode(GL_PROJECTION);   
	glLoadIdentity( );
	// ablak merete a virtualis vilagra
	gluOrtho2D(0., 100., 0., 100.);
	// callbackek
	glutMouseFunc(Mouse);
	glutDisplayFunc( ReDraw );

	// vektorok feltoltese
	if (myfile.is_open())
	{
		getline (myfile,line);
		// ennyi pontunk lesz
		int count = (int)atoi(line.c_str());
		vectors[dlevel] = new Vector* [count];
		vsizes[dlevel] = count;
		int i=0;
		// mostpedig beolvassuk a koordinatakat
		while (! myfile.eof() )
		{
			getline (myfile,line);
			if(line.length())
			{
				size_t pos = line.find(',');
				string buf = line.substr(0,pos);
				float x = (float)atof(buf.c_str());
				buf = line.substr(pos+1);
				float y = (float)atof(buf.c_str());
				vectors[dlevel][i++] = new Vector(x, y);
			}
		}
		myfile.close();
	}
	else
		cout << "nem lehet megnyitni a file-t";

	// rajzoljuk a kezdeti koordinatakat
	ReDraw();
	// fo hurok
	glutMainLoop();
}
