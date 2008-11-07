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
#include <vector>
#include <float.h>

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

const Color             gColorBlack(0.0, 0.0, 0.0);
const Color             gColorWhite(0.9, 0.9, 0.9);
const Color             gColorAmbient(1.5, 1.5, 1.5);   // global ambient
const Vector    gVectorNull(0.0, 0.0, 0.0);

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


//===============================================================
class Ray {
//===============================================================
public:
	Vector	origin;
	Vector dir;
	Ray(const Vector& newOrigin,  const Vector& newDir) { origin = newOrigin; dir = newDir; };

};

//===============================================================
class HitRec {
//===============================================================
public:
	int		objectInd;		// objektum index
	int		primitiveInd;	// primitív index
	Vector	point;			// metszéspont
	Vector	normal;			// normálvektor az adott pontban
	float	t;				// sugárparaméter

	HitRec() { objectInd = primitiveInd = -1; }
};


//===============================================================
class Object {
//===============================================================
public:
	virtual bool		Intersect(const Ray& ray, HitRec* hitRec) { return false; };
	virtual Material*	GetMaterial(const HitRec& hitRec) {return NULL; };
};

//===============================================================
class Mesh : public Object {
//===============================================================
public:
	std::vector <Vector>	vertices;	// csúcspontok
	std::vector <Triangle>	triangles;	// háromszögek

	bool		Intersect(const Ray& ray, HitRec* hitRec);
	Material*	GetMaterial(const HitRec& hitRec) { return triangles[hitRec.primitiveInd].material; }
};

//===============================================================
class Light {
//===============================================================
public:
	Color emission;

	virtual Color	GetEmission() { return emission; };
};

//===============================================================
class PointLight : public Light {
//===============================================================
public:
	Vector location;
};

//===============================================================
class DirectionalLight : public Light {
//===============================================================
public:
	Vector direction; 
};

//===============================================================
class Scene {
//===============================================================
public:
	Camera					camera;
	std::vector <Material>	materials;
	std::vector <Object*>	objects;
	std::vector <Light*>	lights;
	bool					isLoaded;

	Scene() { isLoaded = false;}
	bool	Read				();
	bool	Intersect			(const Ray& ray, HitRec* hitRec);
	Color	Trace				(const Ray& ray, short depth);
	Color	DirectLightsource	(const Vector& inDir, const HitRec& hitRec);
};


//===============================================================
class VrmlReader {
//===============================================================
	Scene*					scene;

	void	HandleCamera			();
	void	HandleMaterial			();
	void	HandleIFaceSet			();
	void	HandlePointLight		();
	void	ComputeView				(const float position[3], float orientation[4], float distance, float target[3], float up[3]);

public:
	VrmlReader(Scene* pScene) { scene = pScene; }
	bool ReadFile();
};

const short MaxDepth = 5;

//-----------------------------------------------------------------
bool Scene::Read() {
//-----------------------------------------------------------------
	VrmlReader vrmlRaeder(this);
	return vrmlRaeder.ReadFile();
}

//-----------------------------------------------------------------
bool Scene::Intersect(const Ray& ray, HitRec* hitRec) {
//-----------------------------------------------------------------
	hitRec->objectInd = -1;
	float mint = FLT_MAX;
	HitRec hitRecLocal;
	for (long i = 0; i < objects.size(); i++) {	// min. kereses
		if (!objects[i]->Intersect(ray, &hitRecLocal))
			continue;
		if (hitRecLocal.t < mint) {
			mint = hitRecLocal.t; 
			*hitRec = hitRecLocal;
			hitRec->objectInd	= i;
		}
	}
	return hitRec->objectInd != -1;
}

//-----------------------------------------------------------------
Color Scene::DirectLightsource(const Vector& inDir, const HitRec& hitRec) {
//-----------------------------------------------------------------
	Color sumColor = gColorBlack; // akkumulált radiancia
	for (short i = 0; i < lights.size(); i++) {
		//		if (dynamic_cast<DirectionalLight*>(lights[i]) != NULL) {
		//			// 1. handle directional lights
		//			DirectionalLight* dLight = dynamic_cast<DirectionalLight*>(lights[i]);
		//			continue;
		//		}

		// 2. pontszeru fényforrások kezelése
		PointLight* pLight = dynamic_cast<PointLight*>(lights[i]);
		// sugár a felületi pontból a fényforrásig
		Ray		rayToLight(hitRec.point, pLight->location - hitRec.point);
		float	lightDist	= rayToLight.dir.Norm();
		rayToLight.dir.Normalize();

		// az árnyalási normális az adott pontban
		float	cost = rayToLight.dir * hitRec.normal;
		if (cost <= 0)	// a test belsejébol jövünk
			continue;

		HitRec	hitRecToLight;
		bool isIntersect = Intersect(rayToLight, &hitRecToLight);
		bool meetLight = !isIntersect;
		if (isIntersect) {//a metszéspont távolabb van, mint a fényforrás
			Vector distIntersect = pLight->location - hitRecToLight.point;
			if (distIntersect.Norm() > lightDist)
				meetLight = true; 	
		}
		if (!meetLight)
			continue;	// árnyékban vagyunk

		Color brdf = objects[hitRec.objectInd]->GetMaterial(hitRec)->Brdf(inDir, rayToLight.dir, hitRec.normal);
		sumColor += brdf * lights[i]->emission * cost;
	}
	return sumColor;
}

//-----------------------------------------------------------------
Color Scene::Trace(const Ray& ray, short depth) {
//-----------------------------------------------------------------
	if (depth > MaxDepth)	   // rekurzio korlatozasa
		return gColorBlack;     

	HitRec hitRec;
	if (!Intersect(ray, &hitRec))
		return gColorBlack;

	// 1. ambiens resz
	Color ambientColor = objects[hitRec.objectInd]->
		GetMaterial(hitRec)->ka * gColorAmbient;
	// 2. fényforrások közvetlen hatása
	Color directLightColor = DirectLightsource(ray.dir, hitRec);

	Material* pMaterial = objects[hitRec.objectInd]->GetMaterial(hitRec);
	// 3. idealis tukor resz
	Color idealReflector = gColorBlack;
	Color kr = pMaterial->kr;
	if (kr.Lum() > EPSILON) {
		Vector reflDir = hitRec.normal * (-2.0 * (ray.dir * hitRec.normal)) 
			+ ray.dir;
		idealReflector = kr * Trace(Ray(hitRec.point, reflDir), depth + 1);
	}
	// 4. idealis feny ateresztes resz
	Color idealRefractor = gColorBlack;
	Color kt = pMaterial->kt;
	if (kt.Lum() > EPSILON) {
		Vector refrDir; //toresmutato függo
		if (pMaterial->RefractionDir(ray.dir, hitRec.normal, &refrDir))
			idealRefractor = kt * Trace(Ray(hitRec.point, refrDir), depth + 1);
	}
	return ambientColor + directLightColor + idealReflector + idealRefractor;
}

//-----------------------------------------------------------------
bool Mesh::Intersect(const Ray& ray, HitRec* hitRec) {
//-----------------------------------------------------------------
	hitRec->primitiveInd = -1;

	float mint = FLT_MAX;
	HitRec hitRecLocal;
	for (long i = 0; i < triangles.size(); i++) {
		if (!triangles[i].Intersect(ray, &hitRecLocal))
			continue;

		if (hitRecLocal.t < mint) {
			mint = hitRecLocal.t; 
			hitRec->primitiveInd = i;
			hitRec->t		= hitRecLocal.t;
			hitRec->point	= hitRecLocal.point;
			hitRec->normal	= triangles[i].normal;
		}
	}
	return hitRec->primitiveInd != -1;
}

enum IntersectMethodType {
	IntersectType3D,
	IntersectType2D,
	IntersectTypeGreen
};

IntersectMethodType IntersectMethod = IntersectTypeGreen;

#define MAX_DISTANCE	1e+7

//-----------------------------------------------------------------
bool Triangle::Intersect(const Ray& ray, HitRec* hitRec) {
//-----------------------------------------------------------------
	if (IntersectMethod == 0)
		return Intersect3D(ray, hitRec);
	if (IntersectMethod == 1)
		return Intersect2D(ray, hitRec);
	if (IntersectMethod == 2)
		return IntersectGreen(ray, hitRec);
}

//-----------------------------------------------------------------
bool Triangle::FinishTriangle(void)  {
//-----------------------------------------------------------------
	Vector va, vb;
	va = *b - *a;
	vb = *c - *b;
	normal= va % vb;
	normal.Normalize();		
	// if 3 vertices in the same line, this result normal= (NAN,NAN,NAN), which is OK.
	if (IntersectMethod == IntersectType2D) {
		dominantAxis = normal.GetDominantAxis();
		// it doens't matter which point to choose for hiperPlaneShiftOffset
		hiperPlaneShiftOffset = -1.0 * normal * *a;

		wasSwap = false;
		switch (dominantAxis) {
		case X_DOMINANT:
			if ((b->y - c->y) * (b->z - a->z) < (b->z - c->z) * (b->y - a->y))
				wasSwap = true;
			break;
		case Y_DOMINANT:
			if ((b->x - c->x) * (b->z - a->z) < (b->z - c->z) * (b->x - a->x))
				wasSwap = true;
			break;
		case Z_DOMINANT:
			if ((b->x - c->x) * (b->y - a->y) < (b->y - c->y) * (b->x - a->x))
				wasSwap = true;
			break;
		}

		if (wasSwap) {	// change pointers pf vertices
			Vector* temp = b;
			b = a;
			a = temp;
			// change pointer of normals
		}
		switch (dominantAxis) {
		case X_DOMINANT:
			d1 = b->z - a->z;
			d2 = b->y - a->y;
			d3 = c->z - b->z;
			d4 = c->y - b->y;
			d5 = a->z - c->z;
			d6 = a->y - c->y;
			break;
		case Y_DOMINANT:
			d1 = b->z - a->z;
			d2 = b->x - a->x;
			d3 = c->z - b->z;
			d4 = c->x - b->x;
			d5 = a->z - c->z;
			d6 = a->x - c->x;		
			break;
		case Z_DOMINANT:
			d1 = b->y - a->y;
			d2 = b->x - a->x;
			d3 = c->y - b->y;
			d4 = c->x - b->x;
			d5 = a->y - c->y;
			d6 = a->x - c->x;			
			break;
		}
	}	// IntersectMethod == Intersect2D

	if (IntersectMethod == IntersectTypeGreen) {
		dominantAxis = normal.GetDominantAxis();
		// it doens't matter which point to choose for hiperPlaneShiftOffset
		hiperPlaneShiftOffset = -1.0 * normal * *a;

		switch (dominantAxis) {
	case X_DOMINANT:
		abV1	= b->z - a->z;
		abV2	= a->y - b->y;
		abC		= abV1 * b->y + abV2 * b->z;
		if (c->y * abV1 + c->z * abV2 > abC) {
			abC		*= -1.0;
			abV1	*= -1.0;
			abV2	*= -1.0;
		}

		bcV1	= c->z - b->z;
		bcV2	= b->y - c->y;
		bcC		= bcV1 * c->y + bcV2 * c->z;
		if (a->y * bcV1 + a->z * bcV2 > bcC) {
			bcC		*= -1.0;
			bcV1	*= -1.0;
			bcV2	*= -1.0;
		}

		caV1	= a->z - c->z;
		caV2	= c->y - a->y;
		caC		= caV1 * a->y + caV2 * a->z;
		if (b->y * caV1 + b->z * caV2 > caC) {
			caC		*= -1.0;
			caV1	*= -1.0;
			caV2	*= -1.0;
		}

		break;
	case Y_DOMINANT:
		abV1	= b->z - a->z;
		abV2	= a->x - b->x;
		abC		= abV1 * b->x + abV2 * b->z;
		if (c->x * abV1 + c->z * abV2 > abC) {
			abC		*= -1.0;
			abV1	*= -1.0;
			abV2	*= -1.0;
		}

		bcV1	= c->z - b->z;
		bcV2	= b->x - c->x;
		bcC		= bcV1 * c->x + bcV2 * c->z;
		if (a->x * bcV1 + a->z * bcV2 > bcC){
			bcC		*= -1.0;
			bcV1	*= -1.0;
			bcV2	*= -1.0;
		}

		caV1	= a->z - c->z;
		caV2	= c->x - a->x;
		caC		= caV1 * a->x + caV2 * a->z;
		if (b->x * caV1 + b->z * caV2 > caC){
			caC		*= -1.0;
			caV1	*= -1.0;
			caV2	*= -1.0;
		}
		break;
	case Z_DOMINANT:
		abV1	= b->y - a->y;
		abV2	= a->x - b->x;
		abC		= abV1 * b->x + abV2 * b->y;
		if (c->x * abV1 + c->y * abV2 > abC) {
			abC		*= -1.0;
			abV1	*= -1.0;
			abV2	*= -1.0;
		}

		bcV1	= c->y - b->y;
		bcV2	= b->x - c->x;
		bcC		= bcV1 * c->x + bcV2 * c->y;
		if (a->x * bcV1 + a->y * bcV2 > bcC) {
			bcC		*= -1.0;
			bcV1	*= -1.0;
			bcV2	*= -1.0;
		}

		caV1	= a->y - c->y;
		caV2	= c->x - a->x;
		caC		= caV1 * a->x + caV2 * a->y;
		if (b->x * caV1 + b->y * caV2 > caC){
			caC		*= -1.0;
			caV1	*= -1.0;
			caV2	*= -1.0;
		}
		break;
		}
	}
	return !isnan(normal.x) && !isnan(normal.y) && !isnan(normal.z);		
}

//-----------------------------------------------------------------
bool Triangle::Intersect2D(const Ray& ray, HitRec* hitRec) {	
//-----------------------------------------------------------------
	float cosa = normal * ray.dir;
	//if (cosa > -EPSILON)	// back facing patch
	//	return false;

	float originDistOnNormal = normal * ray.origin;
	float t = -(hiperPlaneShiftOffset + originDistOnNormal) / cosa;
	if (t < EPSILON4 || t > MAX_DISTANCE)
		return false;

	float s, v;
	switch (dominantAxis)
	{
		case X_DOMINANT:
		// project to YZ plane than
		// test that tg(P2->P1) > tg(P2->Pintersect)   , so if Pintersect is on the wrong side, drop it
		s = ray.origin.y + t * ray.dir.y;
		v = ray.origin.z + t * ray.dir.z;

		if ((b->y - s) * (d1) < (b->z - v) * (d2))	// speed up: e->g store: (b->z - a->z)
			return false;
		if ((c->y - s) * (d3) < (c->z - v) * (d4))
			return false;
		if ((a->y - s) * (d5) < (a->z - v) * (d6))
			return false;
		break;
	case Y_DOMINANT:
		s = ray.origin.x + t * ray.dir.x;
		v = ray.origin.z + t * ray.dir.z;

		if ((b->x - s) * (d1) < (b->z - v) * (d2))
			return false;
		if ((c->x - s) * (d3) < (c->z - v) * (d4))
			return false;
		if ((a->x -s) * (d5) < (a->z - v) * (d6))
			return false;
		break;
	case Z_DOMINANT:
		s = ray.origin.x + t * ray.dir.x;
		v = ray.origin.y + t * ray.dir.y;
		
		if ((b->x - s) * (d1) < (b->y - v) * (d2))
			return false;
		if ((c->x - s) * (d3) < (c->y - v) * (d4))
			return false;
		if ((a->x - s) * (d5) < (a->y - v) * (d6))
			return false;
		break;

	}
	hitRec->point	= ray.origin + ray.dir * t;
	hitRec->t		= t;
	return true;
}

//-----------------------------------------------------------------
bool Triangle::IntersectGreen(const Ray& ray, HitRec* hitRec) {	
//-----------------------------------------------------------------
	float cosa = normal * ray.dir;
	//if (cosa > -EPSILON)	// back facing patch
	//	return false;

	float originDistOnNormal = normal * ray.origin;
	float t = -(hiperPlaneShiftOffset + originDistOnNormal) / cosa;
	if (t < EPSILON4 || t > MAX_DISTANCE)
		return false;

	float s, v;
	switch (dominantAxis) {
	case X_DOMINANT:
		s = ray.origin.y + t * ray.dir.y;
		v = ray.origin.z + t * ray.dir.z;
		break;
	case Y_DOMINANT:
		s = ray.origin.x + t * ray.dir.x;
		v = ray.origin.z + t * ray.dir.z;
		break;
	case Z_DOMINANT:
		s = ray.origin.x + t * ray.dir.x;
		v = ray.origin.y + t * ray.dir.y;
		break;
	}

	if (abV1 * s + abV2 * v > abC)
		return false;
	else if (bcV1 * s + bcV2 * v > bcC)
		return false;
	else if (caV1 * s + caV2 * v > caC)
		return false;

	hitRec->point	= ray.origin + ray.dir * t;
	hitRec->t		= t;
	return true;
}

//-----------------------------------------------------------------
bool  Triangle::Intersect3D(const Ray& ray, HitRec* hitRec) {
//-----------------------------------------------------------------
	double cost = ray.dir * normal;
	if (fabs(cost) <= EPSILON) 
		return false;

	double t = ((*a - ray.origin) * normal) / cost;
	if(t < EPSILON4) 
		return false;

	Vector ip = ray.origin + ray.dir * t;
	hitRec->point	= ip;
	hitRec->t		= t;

	double c1 = (((*b - *a) % (ip - *a)) * normal);
	double c2 = (((*c - *b) % (ip - *b)) * normal);
	double c3 = (((*a - *c) % (ip - *c)) * normal);
	if (c1 >= 0 && c2 >= 0 && c3 >= 0) 
		return true;
	if (c1 <= 0 && c2 <= 0 && c3 <= 0) 
		return true;
	return false;
}

#define VRML_REFERENCE_DISTANCE_FROM_EYE 20.0

//-----------------------------------------------------------------
bool VrmlReader::ReadFile() {
//-----------------------------------------------------------------
	HandleCamera();
	HandlePointLight();
	HandleMaterial();
	HandleIFaceSet();
	HandleMaterial();
	HandleIFaceSet();
	HandleMaterial();
	HandleIFaceSet();
	HandleMaterial();
	HandleIFaceSet();

	// finishScene
	for (long i = 0; i < scene->objects.size(); i++) {
		if (dynamic_cast<Mesh*>(scene->objects[i]) != NULL) {
			Mesh* pMesh = dynamic_cast<Mesh*>(scene->objects[i]);
			for (long j = 0; j < pMesh->triangles.size(); j++) {
				pMesh->triangles[j].a			= &pMesh->vertices[pMesh->triangles[j].ai];
				pMesh->triangles[j].b			= &pMesh->vertices[pMesh->triangles[j].bi];
				pMesh->triangles[j].c			= &pMesh->vertices[pMesh->triangles[j].ci];
				pMesh->triangles[j].material	= &scene->materials[pMesh->triangles[j].materialInd];
				pMesh->triangles[j].FinishTriangle();
			}
		}
	}

	// delete vrmlScene;
	return true;
}

// Compute a target and up vector from position/orientation/distance.
//-----------------------------------------------------------------
void VrmlReader::ComputeView(const float position[3], float orientation[4], float distance, float target[3], float up[3]) {
//-----------------------------------------------------------------
	// Graphics Gems, p 466. Convert between axis/angle and rotation matrix
	float len = sqrt( orientation[0]*orientation[0] + orientation[1]*orientation[1] + orientation[2]*orientation[2] );
	if (len > 0.0) {
		orientation[0] /= len;
		orientation[1] /= len;
		orientation[2] /= len;
	}

	float s = sin(orientation[3]);
	float c = cos(orientation[3]);
	float t = 1.0 - c;

	// Transform [0,0,1] by the orientation to determine sight line
	target[0] = t * orientation[0] * orientation[2] + s * orientation[1];
	target[1] = t * orientation[1] * orientation[2] - s * orientation[0];
	target[2] = t * orientation[2] * orientation[2] + c;

	// Move along that vector the specified distance away from position[]
	target[0] = -distance*target[0] + position[0];
	target[1] = -distance*target[1] + position[1];
	target[2] = -distance*target[2] + position[2];

	// Transform [0,1,0] by the orientation to determine up vector
	up[0] = t * orientation[0] * orientation[1] - s * orientation[2];
	up[1] = t * orientation[1] * orientation[1] + c;
	up[2] = t * orientation[1] * orientation[2] + s * orientation[0];
}

//-----------------------------------------------------------------
void VrmlReader::HandleCamera() {
//-----------------------------------------------------------------
	// orientation specifies a rotation relative to the default orientation (0 0 1  0); 
	// orientation field of the Viewpoint does not affect the definition of the down or up vectors
	// navigation types (see NavigationInfo) that require a definition of an up vector shall use the positive Y-axis of 
	// the coordinate system of the currently bound Viewpoint. 


	// Default viewpoint parameters
	float position[3] = { 1.184, 30.52, 61.69};
	float orientation[4] = { 0.9996, -0.02772, -0.006409, -0.4547 };
	float field = 0.6024;
	float avatarSize = 0.25;
	float visibilityLimit = 0.0;
	float target[3], up[3];

	float dist = VRML_REFERENCE_DISTANCE_FROM_EYE;
	ComputeView(position, orientation, dist, target, up);

	scene->camera.fov = (field / 3.14f) * 180.0f / 2.0;
	scene->camera.eyep.Set(position[0], position[1], position[2]);
	scene->camera.updir.Set(up[0], up[1], up[2]);
	scene->camera.lookp.Set(target[0], target[1], target[2]);

	scene->camera.nearClip	= DefaultCameraNearClip; //EPSILON;
	scene->camera.farClip	= DefaultCameraFarClip;
	scene->camera.CompleteCamera();
}

//-----------------------------------------------------------------
void VrmlReader::HandleMaterial() {
//-----------------------------------------------------------------
	static int cnum = 0;
	Material material;

	if (cnum == 0 ) {
		// diffuse color
		material.Kd.Set(0.08627, 0.08627, 0.08627);
		// specular color
		material.Ks.Set(0.9294, 0.9294, 0.9294);
		// ambient intensity
		material.Ka = material.Kd * 1.0;
		// shininess
		material.shine = 100;
		// transparency
		material.kt.Set(0, 0, 0);
	} else if (cnum == 1) {
		material.Kd.Set(0.102, 0.6941, 0.5804);
	} else if (cnum == 2) {
		material.Kd.Set(0.4235, 0.03137, 0.5333);
		material.kt.Set(0.8, 0.8, 0.8);
	} else if (cnum == 3) {
		material.Kd.Set(0.3451, 0.7804, 0.8824);
	}
	material.FinishMaterial();

	scene->materials.push_back(material);
	cnum++;
}

#define ARRAY_SIZE(x) (sizeof(x)/sizeof(x[0]))
//-----------------------------------------------------------------
void VrmlReader::HandleIFaceSet() {
//-----------------------------------------------------------------
	static int cnum = 0;

	Mesh* mesh = new Mesh;
	// 2. Handle "coord"
	Vector	newVector;

	if (cnum == 0) {
		float coords[] = {
			-24.995596, 0.368900, 25.000000, -12.495596, 0.368900, 25.000000, 0.004404, 0.368900, 25.000000, 12.504404, 0.368900, 25.000000, 25.004404, 0.368900, 25.000000, -24.995596, 0.368900, 12.500000, -12.495596, 0.368900, 12.500000, 0.004404, 0.368900, 12.500000, 12.504404, 0.368900, 12.500000, 25.004404, 0.368900, 12.500000, -24.995596, 0.368900, 0.000000, -12.495596, 0.368900, 0.000000, 0.004404, 0.368900, 0.000000, 12.504404, 0.368900, 0.000000, 25.004404, 0.368900, 0.000000, -24.995596, 0.368900, -12.500000, -12.495596, 0.368900, -12.500000, 0.004404, 0.368900, -12.500000, 12.504404, 0.368900, -12.500000, 25.004404, 0.368900, -12.500000, -24.995596, 0.368900, -25.000000, -12.495596, 0.368900, -25.000000, 0.004404, 0.368900, -25.000000, 12.504404, 0.368900, -25.000000, 25.004404, 0.368900, -25.000000
		};
		for (int i = 0; i < ARRAY_SIZE(coords); i+= 3) {
			newVector.Set(coords[i], coords[i+1], coords[i+2]);
			mesh->vertices.push_back(newVector);
		}
	} else if (cnum == 1) {
		float coords[] = {
			-9.510000, 17.138000, -15.300000, -9.510000, 7.138000, -15.300000, -19.510000, 7.138000, -15.300000, -19.510000, 17.138000, -15.300000, -9.510000, 17.137989, -5.300000, -9.510010, 7.137999, -5.300000, -19.510000, 7.138000, -5.300000, -19.510000, 17.138000, -5.300000
		};
		for (int i = 0; i < ARRAY_SIZE(coords); i+= 3) {
			newVector.Set(coords[i], coords[i+1], coords[i+2]);
			mesh->vertices.push_back(newVector);
		}
	} else if (cnum == 2) {
		float coords[] = {
			8.510000, 13.138000, -18.299999, 8.510000, 3.138000, -18.299999, -1.490000, 3.138000, -18.299999, -1.490000, 13.138000, -18.299999, 8.510000, 13.137989, -8.300000, 8.509990, 3.137999, -8.300000, -1.490000, 3.138000, -8.300000, -1.490000, 13.138000, -8.300000
		};
		for (int i = 0; i < ARRAY_SIZE(coords); i+= 3) {
			newVector.Set(coords[i], coords[i+1], coords[i+2]);
			mesh->vertices.push_back(newVector);
		}
	} else if (cnum == 3) {
		float coords[] = {
			24.778297, -0.005699, -25.468597, 24.781147, 12.494300, -25.471447, 24.783998, 24.994299, -25.474298, 24.786850, 37.494297, -25.477148, 24.789700, 49.994297, -25.480000, 12.278297, -0.002849, -25.471447, 12.281149, 12.497150, -25.474298, 12.283999, 24.997150, -25.477148, 12.286850, 37.497150, -25.480000, 12.289701, 49.997147, -25.482849, -0.221702, 0.000001, -25.474298, -0.218851, 12.500001, -25.477150, -0.216000, 25.000000, -25.480000, -0.213149, 37.500000, -25.482849, -0.210298, 50.000000, -25.485701, -12.721701, 0.002852, -25.477150, -12.718850, 12.502851, -25.480000, -12.716000, 25.002850, -25.482851, -12.713148, 37.502850, -25.485701, -12.710298, 50.002850, -25.488552, -25.221701, 0.005702, -25.480000, -25.218849, 12.505702, -25.482851, -25.215998, 25.005701, -25.485701, -25.213148, 37.505699, -25.488552, -25.210297, 50.005699, -25.491402
		};
		for (int i = 0; i < ARRAY_SIZE(coords); i+= 3) {
			newVector.Set(coords[i], coords[i+1], coords[i+2]);
			mesh->vertices.push_back(newVector);
		}
	}

	// handle "coordIndex"
	int coordIndex0[] = {
		5, 0, 6, -1, 1, 6, 0, -1, 6, 1, 7, -1, 2, 7, 1, -1, 7, 2, 8, -1,
		3, 8, 2, -1, 8, 3, 9, -1, 4, 9, 3, -1, 10, 5, 11, -1,
		6, 11, 5, -1, 11, 6, 12, -1, 7, 12, 6, -1, 12, 7, 13, -1,
		8, 13, 7, -1, 13, 8, 14, -1, 9, 14, 8, -1, 15, 10, 16, -1,
		11, 16, 10, -1, 16, 11, 17, -1, 12, 17, 11, -1, 17, 12, 18, -1,
		13, 18, 12, -1, 18, 13, 19, -1, 14, 19, 13, -1, 20, 15, 21, -1,
		16, 21, 15, -1, 21, 16, 22, -1, 17, 22, 16, -1, 22, 17, 23, -1,
		18, 23, 17, -1, 23, 18, 24, -1, 19, 24, 18, -1
	};
	int coordIndex1[] = {
		0, 1, 2, 3, -1,
		4, 7, 6, 5, -1,
		0, 4, 5, 1, -1,
		1, 5, 6, 2, -1,
		2, 6, 7, 3, -1,
		4, 0, 3, 7, -1
	};
	int coordIndex2[] = {
		0, 1, 2, 3, -1,
		4, 7, 6, 5, -1,
		0, 4, 5, 1, -1,
		1, 5, 6, 2, -1,
		2, 6, 7, 3, -1,
		4, 0, 3, 7, -1

	};
	int coordIndex3[] = {
		5, 0, 6, -1, 1, 6, 0, -1, 6, 1, 7, -1, 2, 7, 1, -1, 7, 2, 8, -1,
		3, 8, 2, -1, 8, 3, 9, -1, 4, 9, 3, -1, 10, 5, 11, -1,
		6, 11, 5, -1, 11, 6, 12, -1, 7, 12, 6, -1, 12, 7, 13, -1,
		8, 13, 7, -1, 13, 8, 14, -1, 9, 14, 8, -1, 15, 10, 16, -1,
		11, 16, 10, -1, 16, 11, 17, -1, 12, 17, 11, -1, 17, 12, 18, -1,
		13, 18, 12, -1, 18, 13, 19, -1, 14, 19, 13, -1, 20, 15, 21, -1,
		16, 21, 15, -1, 21, 16, 22, -1, 17, 22, 16, -1, 22, 17, 23, -1,
		18, 23, 17, -1, 23, 18, 24, -1, 19, 24, 18, -1
	};
	int *coordIndex;
	int coordIndexSize;
	if (cnum == 0) {
		coordIndex = coordIndex0;
		coordIndexSize = ARRAY_SIZE(coordIndex0);
	} else if (cnum == 1) {
		coordIndex = coordIndex1;
		coordIndexSize = ARRAY_SIZE(coordIndex1);
	} else if (cnum == 2) {
		coordIndex = coordIndex2;
		coordIndexSize = ARRAY_SIZE(coordIndex2);
	} else if (cnum == 3) {
		coordIndex = coordIndex3;
		coordIndexSize = ARRAY_SIZE(coordIndex3);
	}
	long poligonStartIndex		= 0;
	for (int i = 0; i < coordIndexSize; i++) {
		if (coordIndex[i] != -1)
			continue;

		long nTriangles = i - poligonStartIndex - 2;
		for (long k = 0; k < nTriangles; k++) {
			Triangle triangle;
			triangle.ai = coordIndex[poligonStartIndex];
			triangle.bi = coordIndex[poligonStartIndex + k + 1];
			triangle.ci = coordIndex[poligonStartIndex + k + 2];

			triangle.materialInd = scene->materials.size() - 1;		// means the current material

			mesh->triangles.push_back(triangle);
		}
		poligonStartIndex = i + 1;
	}
	scene->objects.push_back(mesh);
	cnum++;

}

//-----------------------------------------------------------------
void VrmlReader::HandlePointLight() {
//-----------------------------------------------------------------
	PointLight* light = new PointLight;
	light->location.Set( -20.6, 51.43, 28.99);
	light->emission.Set(4, 4, 4);
	scene->lights.push_back(light);
}

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
