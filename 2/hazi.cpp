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

// FIXME get rid of includes
#include <iostream>

//--------------------------------------------------------
// Nev: Vajna Miklos
// Neptun: AYU9RZ
//--------------------------------------------------------

enum DominantAxis {
	X_DOMINANT,
	Y_DOMINANT,
	Z_DOMINANT
};

//===============================================================
class Vector {
//===============================================================
public:
	float x, y, z;

	Vector() {}
	Vector(float xx, float yy, float zz) {
		x = xx; y = yy; z = zz;
	}
	
	void Set(float xx, float yy, float zz) {
		x = xx; y = yy; z = zz;
	}

	// binary operators
	Vector operator+(const Vector& c) const {
		return Vector(x + c.x, y + c.y, z + c.z);
	}

	Vector operator-(const Vector& c) const {
		return Vector(x - c.x, y - c.y, z - c.z);
	}

	Vector operator*(float f) const {
		return Vector(x * f, y * f, z * f);
	}

	Vector operator/(float f) const {
		return Vector(x / f, y / f, z / f);
	}

	// unary operators
	void operator+=(float f) { 
		x += f; y += f; z += f; 
	}

	void operator+=(const Vector& c) { 
		x += c.x; y += c.y; z += c.z; 
	}

	void operator*=(float f) { 
		x *= f; y *= f; z *= f; 
	}

	Vector operator-(void) const { 
		return Vector(-x, -y, -z);
	}

	// other methods
	float operator*(const Vector& v) const {		// DotProduct
		return x * v.x + y * v.y + z * v.z; 
	}

	Vector operator%(const Vector& v) const {		// CrossProduct
		return Vector(y * v.z - z * v.y, z * v.x - x * v.z, x * v.y - y * v.x);
	}    

	void operator<=(const Vector& smallVec) {
		if (x > smallVec.x) x = smallVec.x;
		if (y > smallVec.y) y = smallVec.y;
		if (z > smallVec.z) z = smallVec.z;
	}

	void operator>=(const Vector& largeVec) {
		if (x < largeVec.x) x = largeVec.x;
		if (y < largeVec.y) y = largeVec.y;
		if (z < largeVec.z) z = largeVec.z;
	}

	float Norm(void) const {
		return sqrt(x*x + y*y + z*z);
	}

	void Normalize() {
		float norm = Norm();
		x /= norm;
		y /= norm;
		z /= norm;
	}

	DominantAxis GetDominantAxis (void) const {
		float dx = fabs(x);
		float dy = fabs(y);
		float dz = fabs(z);
		return (dx > dy) ? ((dx > dz) ? X_DOMINANT : Z_DOMINANT) : ((dy > dz) ? Y_DOMINANT : Z_DOMINANT);
	}

	// other methods
	friend Vector operator*(float f, const Vector& v);
};

inline Vector operator*(float f, const Vector& v) {
	return Vector(f * v.x, f * v.y, f * v.z);
}

const int DefaultScreenWidth	= 200;
const int DefaultScreenHeight	= 200;

const float DefaultCameraNearClip	= 0.05;
const float DefaultCameraFarClip	= 500.0;

#define EPSILON                         1e-5F
#define EPSILON9                        1e-9F
#define EPSILON5                        1e-5F
#define EPSILON4                        1e-4F
#define EPSILON3                        1e-3F

//===============================================================
class Camera {
//===============================================================
public:
	Vector	eyep;			//! virtual camera position in 3D space
	Vector	lookp;			//! focus point of camera
	Vector	updir;			//! direction pointing upward

	float	viewdist;		//! distance from eyepoint to focus point	
	float	fov, hfov, vfov;	//! half of the field of view, horizontal and vertical, in degrees.

	float	nearClip, farClip;		//! near and far clipping plane distance
	long	hres, vres;			//! horizontal and vertical resolution
		
	Vector	X, Y, Z;			//! eye coordinate system (right-hand-orientation): X=right, Y=down, Z=viewing direction
	float	pixh, pixv;		//! Width and height of a pixel

	inline Camera();
	inline void CompleteCamera();
	inline void MoveUpDown(float step);
	inline void Strafe(float step);
};

//-----------------------------------------------------------------
Camera::Camera() {
//-----------------------------------------------------------------
	eyep.Set(0.0, 0.0, 10.0);
	lookp.Set(0.0, 0.0, 0.0);
	updir.Set(0., 1.0, 0.0);
	fov			= 22.5;

	nearClip	= DefaultCameraNearClip; //EPSILON;
	farClip		= DefaultCameraFarClip;

	hres		= DefaultScreenWidth;
	vres		= DefaultScreenHeight;

	CompleteCamera();
}

//-----------------------------------------------------------------
void Camera::CompleteCamera() {
//-----------------------------------------------------------------
	// set up Z
	Z = lookp - eyep;
	/* distance from virtual camera position to focus point */
	viewdist = Z.Norm();
	if (viewdist < EPSILON) {
		std::cout << "Camera eyepoint and look-point coincide" << std::endl;
		return;
	}
	Z *= 1.0 / viewdist;

	// set up X   Camera->X is a direction pointing to the right in the window 
	X = Z % updir;
	float lengthX = X.Norm();
	if (lengthX < EPSILON) {
		std::cout << "Camera up-direction and viewing direction coincide" << std::endl;
		return;
	}
	X *= 1.0 / lengthX;

	// set up Y
	Y = Z % X;
	Y.Normalize();

	// compute horizontal and vertical field of view angle from the specified one
	// if the vertical resolution is smaller, it is the specified fov = 45, the other is greater than 45
	if (hres < vres) {
		hfov = fov; 
		vfov = atan(tan(fov * M_PI/180.0) * (float)vres/(float)hres) * 180.0/M_PI;
	} else {
		vfov = fov; 
		hfov = atan(tan(fov * M_PI/180.0) * (float)hres/(float)vres) * 180.0/M_PI;
	}

	float tanFovH = tan(hfov * M_PI / 180.0);
	float tanFovV = tan(vfov * M_PI / 180.0);
	pixh = 2.0 * tanFovH / (float)(hres);
	pixv = 2.0 * tanFovV / (float)(vres);

}

//-----------------------------------------------------------------
void Camera::Strafe(float step) {
//-----------------------------------------------------------------
	eyep	+= step * X;
	lookp	+= step * X;
	CompleteCamera();
}

//-----------------------------------------------------------------
void Camera::MoveUpDown(float step) {
//-----------------------------------------------------------------
	eyep	+= step * Y;
	lookp	+= step * Y;
	CompleteCamera();
}

//===============================================================
class Color {
//===============================================================
public:
	float r, g, b;		// color coefficients on the representative wavelengths

	Color() {}
	Color(float rr, float gg, float bb) { 
		r = rr; g = gg; b = bb;
	} 

	void Set(float rr, float gg, float bb) {
		r = rr; g = gg; b = bb;
	}

	// binary operators
	Color operator+(const Color& c) const {
		return Color(r + c.r, g + c.g, b + c.b);
	}

	Color operator-(const Color& c) const {
		return Color(r - c.r, g - c.g, b - c.b);
	}

	Color operator*(float f) const {
		return Color(r * f, g * f, b * f);
	}

	Color operator*(const Color& c) const {
		return Color(r * c.r, g * c.g, b * c.b);
	}

	Color operator/(float f) const {
		return Color(r / f, g / f, b / f);
	}

	// unary operators
	void operator+=(const Color& c) {
		r += c.r; g += c.g; b += c.b;
	}

	void operator*=(const Color& c) {
		r *= c.r; g *= c.g; b *= c.b;
	}

	void operator*=(float c) {
		r *= c; g *= c; b *= c;
	}

	Color operator-(void) {
		return Color(-r, -g, -b);
	}

	// other methods
	float Lum() const {
		return (r + g + b) / 3.0;
	}
	friend Color operator*(float f, const Color& c);
};

inline Color operator*(float f, const Color& c) {
	return Color(f * c.r, f * c.g, f * c.b);
}

extern const Color	gColorBlack;
extern const Color	gColorWhite;
extern const Color	gColorAmbient;	// global ambient

//===============================================================
class Material {
//===============================================================
public:
	char name[256];	// anyag neve
	Color Ka;			// ambiens albedo (ka*pi)	
	Color Kd;			// diffúz albedo (kd*pi)
	Color Ks;			// spekuláris albedó 
	float shine;	// fényesség

	// eloreszámított értékek
	Color ka;			// a BRDF ambines tagja
	Color kd;			// a BRDF diffúz tagja

	Color kr;			// tökéletes tükör hányados
	Color kt;			// tökéletes 
	float n;		// toresmutato

	Material();
	void	FinishMaterial (void);
	Color	Brdf(const Vector& inDir, const Vector& outDir, const Vector& normal);
	bool	RefractionDir(const Vector& inDir, const Vector& normal, Vector* outDir);
};

//-----------------------------------------------------------------
inline Material::Material() {
//-----------------------------------------------------------------
	name[0] = '\0';
	Ka = Kd = Ks = ka = kd = kr = kt = gColorBlack;
	shine = 0;
}

//-----------------------------------------------------------------
inline void Material::FinishMaterial(void) {
//-----------------------------------------------------------------
	ka = Ka / M_PI;			// a BRDF ambines tagja
	kd = Kd / M_PI;			// a BRDF diffúz tagja
	
	if (shine >= 100.0) {	// 100-as shine esetén tükörnek tekintjük
		kr	= Ks;
		Ks	= gColorBlack;
	}

	n = 1.2;				// törésmutatót VRML-ben nem lehet megadni
}

//-----------------------------------------------------------------
inline Color Material::Brdf(const Vector& inDir, const Vector& outDir, const Vector& normal) {
//-----------------------------------------------------------------
	double cosIn = -1.0 * (inDir * normal);
	if (cosIn <= EPSILON)		// ha az anyag belsejébol jövünk
		return gColorBlack;

	Color ks = gColorBlack;
	Vector reflDir = normal * (2.0 * cosIn) + inDir;
	double cos_refl_out = reflDir * outDir;
	if (cos_refl_out > EPSILON) {
		Color ref = Ks * (shine + 2) / M_PI / 2.0;
		ks = ref * pow(cos_refl_out, shine);
	}
	return kd + ks;		// diffúz + spekuláris BRDF
}

//-----------------------------------------------------------------
inline bool Material::RefractionDir(const Vector& inDir, const Vector& normal, Vector* outDir)  {
//-----------------------------------------------------------------
	double cosIn = -1.0 * (inDir * normal);
	if (fabs(cosIn) <= EPSILON4)
		return false;

	float cn = n;
	Vector useNormal = normal;
	if (cosIn < 0) {				// ha az anyag belsejebol jovunk
		cn			= 1.0 / n;	 
		useNormal	= -normal;		// a toresmutato reciprokat kell hasznalni
		cosIn		= -cosIn;
	}															
	
	float disc = 1 - (1 - cosIn * cosIn) / cn / cn;	 // Snellius-Descartes torveny
	if (disc < 0) 
		return false;

	*outDir = useNormal * (cosIn / cn - sqrt(disc)) + inDir / cn;
	return true;
}

class Ray;
class HitRec;

//===============================================================
class Triangle {
//===============================================================
public:
	Vector			*a, *b, *c;		//! defines the 3 vertices
	long			ai, bi, ci;		// indexes

	Vector			normal;	
	Vector			*Na, *Nb, *Nc;	//! normal for vertex a,b,c

	Material*		material;
	long			materialInd;

	DominantAxis	dominantAxis;
	float			hiperPlaneShiftOffset;
	bool			wasSwap;
	float			u1, u2, v1, v2;				// for Intersect2D()
	float			d1, d2, d3, d4, d5, d6;		// pre-computation for IntersectFast() speed-up
	float			abV1, abV2, abC, bcV1, bcV2, bcC, caV1, caV2, caC;	// for IntersectGreen()
protected:
	bool	Intersect3D(const Ray& ray, HitRec* hitRec);
	bool	Intersect2D(const Ray& ray, HitRec* hitRec);
	bool	IntersectGreen(const Ray& ray, HitRec* hitRec);
public:
	bool	FinishTriangle(void);
	bool	Intersect(const Ray& ray, HitRec* hitRec);
};



void onInitialization( ) {
}

void onDisplay( ) {
    glClearColor(0.1f, 0.2f, 0.3f, 1.0f);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

    // ...

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
