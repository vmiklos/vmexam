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

#ifdef DEBUG
#include <iostream>
#endif
#include <vector>
using namespace std;
#include <float.h>

const short MaxDepth = 5;

#define SIZE 600

class Vector {
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

		friend Vector operator*(float f, const Vector& v) {
			return Vector(f * v.x, f * v.y, f * v.z);
		}
};

const float DefaultCameraNearClip	= 0.05;
const float DefaultCameraFarClip	= 500.0;
int clickx = 0, clicky = 0;
vector<Vector> raypoints;

#define EPSILON                         1e-5F
#define EPSILON4                        1e-4F

class Camera {
	public:
		Vector	eyep;			//! virtual camera position in 3D space
		Vector	lookp;			//! focus point of camera
		Vector	updir;			//! direction pointing upward

		float	viewdist;		//! distance from eyepoint to focus point	
		float	fov, hfov, vfov;	//! half of the field of view, horizontal and vertical, in degrees.

		float	nearClip, farClip;		//! near and far clipping plane distance

		Vector	X, Y, Z;			//! eye coordinate system (right-hand-orientation): X=right, Y=down, Z=viewing direction
		float	pixh, pixv;		//! Width and height of a pixel

		void CompleteCamera() {
			// set up Z
			Z = lookp - eyep;
			/* distance from virtual camera position to focus point */
			viewdist = Z.Norm();
			if (viewdist < EPSILON) {
				// Camera eyepoint and look-point coincide
				return;
			}
			Z *= 1.0 / viewdist;

			// set up X   Camera->X is a direction pointing to the right in the window 
			X = Z % updir;
			float lengthX = X.Norm();
			if (lengthX < EPSILON) {
				// Camera up-direction and viewing direction coincide
				return;
			}
			X *= 1.0 / lengthX;

			// set up Y
			Y = Z % X;
			Y.Normalize();

			// compute horizontal and vertical field of view angle from the specified one
			vfov = fov; 
			hfov = atan(tan(fov * M_PI/180.0)) * 180.0/M_PI;

			float tanFovH = tan(hfov * M_PI / 180.0);
			float tanFovV = tan(vfov * M_PI / 180.0);
			pixh = 2.0 * tanFovH / SIZE;
			pixv = 2.0 * tanFovV / SIZE;
		}
};

class Color {
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
		friend Color operator*(float f, const Color& c) {
			return Color(f * c.r, f * c.g, f * c.b);
		}
};

const Color             gColorBlack(0.0, 0.0, 0.0);
const Color             gColorWhite(0.9, 0.9, 0.9);
const Color             gColorAmbient(1.5, 1.5, 1.5);   // global ambient
const Vector    gVectorNull(0.0, 0.0, 0.0);

class Material {
	public:
		Color Ka;			// ambiens albedo (ka*pi)	
		Color Kd;			// diffuz albedo (kd*pi)
		Color Ks;			// spekularis albedo 
		float shine;	// fenyesseg

		// eloreszamitott ertekek
		Color ka;			// a BRDF ambines tagja
		Color kd;			// a BRDF diffuz tagja

		Color kr;			// tokeletes tukor hanyados
		Color kt;			// tokeletes 
		float n;		// toresmutato

		Material() {
			Ka = Kd = Ks = ka = kd = kr = kt = gColorBlack;
			shine = 0;
			n = 1.2;
		}
		void FinishMaterial (void) {
			ka = Ka / M_PI;			// a BRDF ambines tagja
			kd = Kd / M_PI;			// a BRDF diffuz tagja

			if (shine >= 100.0) {	// 100-as shine eseten tukornek tekintjuk
				kr	= Ks;
				Ks	= gColorBlack;
			}
		}
		Color Brdf(const Vector& inDir, const Vector& outDir, const Vector& normal) {
			double cosIn = -1.0 * (inDir * normal);
			if (cosIn <= EPSILON)		// ha az anyag belsejebol jovunk
				return gColorBlack;

			Color ks = gColorBlack;
			Vector reflDir = normal * (2.0 * cosIn) + inDir;
			double cos_refl_out = reflDir * outDir;
			if (cos_refl_out > EPSILON) {
				Color ref = Ks * (shine + 2) / M_PI / 2.0;
				ks = ref * pow(cos_refl_out, shine);
			}
			return kd + ks;		// diffuz + spekularis BRDF
		}
		bool RefractionDir(const Vector& inDir, const Vector& normal, Vector* outDir) {
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
};

class Ray {
	public:
		Vector	origin;
		Vector dir;
		Ray(const Vector& newOrigin,  const Vector& newDir) { origin = newOrigin; dir = newDir; };

};

class HitRec {
	public:
		int		objectInd;		// objektum index
		int		primitiveInd;	// primitiv index
		Vector	point;			// metszespont
		Vector	normal;			// normalvektor az adott pontban
		float	t;				// sugarparameter

		HitRec() { objectInd = primitiveInd = -1; }
};


class Triangle {
	public:
		Vector			*a, *b, *c;		//! defines the 3 vertices
		long			ai, bi, ci;		// indexes

		Vector			normal;	
		Vector			*Na, *Nb, *Nc;	//! normal for vertex a,b,c

		Material*		material;
		long			materialInd;
	public:
		bool	FinishTriangle(void){
			Vector va, vb;
			va = *b - *a;
			vb = *c - *b;
			normal= va % vb;
			normal.Normalize();		
			// if 3 vertices in the same line, this result normal= (NAN,NAN,NAN), which is OK.
			return !isnan(normal.x) && !isnan(normal.y) && !isnan(normal.z);		
		}

		bool	Intersect(const Ray& ray, HitRec* hitRec) {
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
};



class Mesh {
	public:
		vector <Vector>	vertices;	// csucspontok
		vector <Triangle>	triangles;	// haromszogek

		bool		Intersect(const Ray& ray, HitRec* hitRec) {
			hitRec->primitiveInd = -1;

			float mint = FLT_MAX;
			HitRec hitRecLocal;
			for (unsigned i = 0; i < triangles.size(); i++) {
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
		Material*	GetMaterial(const HitRec& hitRec) { return triangles[hitRec.primitiveInd].material; }
};

class PointLight {
	public:
		Color emission;

		virtual Color	GetEmission() { return emission; };
		Vector location;
};

class Scene;

class Model {
	Scene*					scene;

	public:
	void	HandleCamera ();
	void	HandleMaterial ();
	void	HandleIFaceSet ();
	void	HandlePointLight ();
	void	ComputeView (const float position[3], float orientation[4], float distance, float target[3], float up[3]) {
		// Graphics Gems, p 466. Convert between axis/angle and rotation matrix
		float len = sqrt( orientation[0]*orientation[0] +
				orientation[1]*orientation[1] +
				orientation[2]*orientation[2] );
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

	Model(Scene* pScene) { scene = pScene; }
};


class Scene {
	public:
		Camera					camera;
		vector <Material>	materials;
		vector <Mesh*>	objects;
		vector <PointLight*>	lights;

		bool	Build() {
			Model m(this);
			m.HandleCamera();
			m.HandlePointLight();
			m.HandleMaterial();
			m.HandleIFaceSet();
			m.HandleMaterial();
			m.HandleIFaceSet();
			m.HandleMaterial();
			m.HandleIFaceSet();
			m.HandleMaterial();
			m.HandleIFaceSet();

			// finishScene
			for (unsigned i = 0; i < objects.size(); i++) {
				Mesh* pMesh = objects[i];
				for (unsigned j = 0; j < pMesh->triangles.size(); j++) {
					pMesh->triangles[j].a			= &pMesh->vertices[pMesh->triangles[j].ai];
					pMesh->triangles[j].b			= &pMesh->vertices[pMesh->triangles[j].bi];
					pMesh->triangles[j].c			= &pMesh->vertices[pMesh->triangles[j].ci];
					pMesh->triangles[j].material	= &materials[pMesh->triangles[j].materialInd];
					pMesh->triangles[j].FinishTriangle();
				}
			}
			return true;
		}
		bool	Intersect			(const Ray& ray, HitRec* hitRec) {
			hitRec->objectInd = -1;
			float mint = FLT_MAX;
			HitRec hitRecLocal;
			for (unsigned i = 0; i < objects.size(); i++) {	// min. kereses
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
		Color	Trace				(const Ray& ray, short depth) {
			if (depth > MaxDepth)	   // rekurzio korlatozasa
				return gColorBlack;     

			HitRec hitRec;
			if (!Intersect(ray, &hitRec))
				return gColorBlack;
			if (clickx && clicky)
				raypoints.push_back(hitRec.point);

			// 1. ambiens resz
			Color ambientColor = objects[hitRec.objectInd]->
				GetMaterial(hitRec)->ka * gColorAmbient;
			// 2. fenyforrasok kozvetlen hatasa
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
			// 4. idealis fenyu ateresztes resz
			Color idealRefractor = gColorBlack;
			Color kt = pMaterial->kt;
			if (kt.Lum() > EPSILON) {
				Vector refrDir; //toresmutato fuggo
				if (pMaterial->RefractionDir(ray.dir, hitRec.normal, &refrDir))
					idealRefractor = kt * Trace(Ray(hitRec.point, refrDir), depth + 1);
			}
			return ambientColor + directLightColor + idealReflector + idealRefractor;
		}

		Color	DirectLightsource	(const Vector& inDir, const HitRec& hitRec) {
			Color sumColor = gColorBlack; // akkumulalt radiancia
			for (unsigned i = 0; i < lights.size(); i++) {
				// 2. pontszeru fenyforrasok kezelese
				PointLight* pLight = lights[i];
				// sugar a feluleti pontbol a fenyforrasig
				Ray		rayToLight(hitRec.point, pLight->location - hitRec.point);
				float	lightDist	= rayToLight.dir.Norm();
				rayToLight.dir.Normalize();

				// az arnyalasi normalis az adott pontban
				float	cost = rayToLight.dir * hitRec.normal;
				if (cost <= 0)	// a test belsejebol jovunk
					continue;

				HitRec	hitRecToLight;
				bool isIntersect = Intersect(rayToLight, &hitRecToLight);
				bool meetLight = !isIntersect;
				if (isIntersect) {//a metszespont tavolabb van, mint a fenyforras
					Vector distIntersect = pLight->location - hitRecToLight.point;
					if (distIntersect.Norm() > lightDist)
						meetLight = true; 	
				}
				if (!meetLight)
					continue;	// arnyekban vagyunk

				Color brdf = objects[hitRec.objectInd]->GetMaterial(hitRec)->Brdf(inDir, rayToLight.dir, hitRec.normal);
				sumColor += brdf * lights[i]->emission * cost;
			}
			return sumColor;
		}
};



void Model::HandleCamera() {
	// orientation specifies a rotation relative to the default orientation (0 0 1  0); 
	// orientation field of the Viewpoint does not affect the definition of the down or up vectors
	// navigation types (see NavigationInfo) that require a definition of an up vector shall use the positive Y-axis of 
	// the coordinate system of the currently bound Viewpoint. 


	// Default viewpoint parameters
	float position[3] = { 1.184, 30.52, 61.69};
	float orientation[4] = { 0.9996, -0.02772, -0.006409, -0.4547 };
	float field = 0.6024;
	float target[3], up[3];
	float dist = 20;

	// Compute a target and up vector from position/orientation/distance.
	ComputeView(position, orientation, dist, target, up);

	scene->camera.fov = (field / 3.14f) * 180.0f / 2.0;
	scene->camera.eyep.Set(position[0], position[1], position[2]);
	scene->camera.updir.Set(up[0], up[1], up[2]);
	scene->camera.lookp.Set(target[0], target[1], target[2]);

	scene->camera.nearClip	= DefaultCameraNearClip; //EPSILON;
	scene->camera.farClip	= DefaultCameraFarClip;
	scene->camera.CompleteCamera();
}

void Model::HandleMaterial() {
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
#define BUILD_COORDS(coords) do { \
		for (unsigned i = 0; i < ARRAY_SIZE(coords); i+= 3) { \
			newVector.Set(coords[i], coords[i+1], coords[i+2]); \
			mesh->vertices.push_back(newVector); \
		} \
	} while(0)
void Model::HandleIFaceSet() {
	static int cnum = 0;

	Mesh* mesh = new Mesh;
	// 2. Handle "coord"
	Vector	newVector;

	if (cnum == 0) {
		float coords[] = {
			-24.995596, 0.368900, 25.000000, -12.495596, 0.368900, 25.000000, 0.004404, 0.368900, 25.000000, 12.504404, 0.368900, 25.000000, 25.004404, 0.368900, 25.000000, -24.995596, 0.368900, 12.500000, -12.495596, 0.368900, 12.500000, 0.004404, 0.368900, 12.500000, 12.504404, 0.368900, 12.500000, 25.004404, 0.368900, 12.500000, -24.995596, 0.368900, 0.000000, -12.495596, 0.368900, 0.000000, 0.004404, 0.368900, 0.000000, 12.504404, 0.368900, 0.000000, 25.004404, 0.368900, 0.000000, -24.995596, 0.368900, -12.500000, -12.495596, 0.368900, -12.500000, 0.004404, 0.368900, -12.500000, 12.504404, 0.368900, -12.500000, 25.004404, 0.368900, -12.500000, -24.995596, 0.368900, -25.000000, -12.495596, 0.368900, -25.000000, 0.004404, 0.368900, -25.000000, 12.504404, 0.368900, -25.000000, 25.004404, 0.368900, -25.000000
		};
		BUILD_COORDS(coords);
	} else if (cnum == 1) {
		float coords[] = {
			-9.510000, 17.138000, -15.300000, -9.510000, 7.138000, -15.300000, -19.510000, 7.138000, -15.300000, -19.510000, 17.138000, -15.300000, -9.510000, 17.137989, -5.300000, -9.510010, 7.137999, -5.300000, -19.510000, 7.138000, -5.300000, -19.510000, 17.138000, -5.300000
		};
		BUILD_COORDS(coords);
	} else if (cnum == 2) {
		float coords[] = {
			8.510000, 13.138000, -18.299999, 8.510000, 3.138000, -18.299999, -1.490000, 3.138000, -18.299999, -1.490000, 13.138000, -18.299999, 8.510000, 13.137989, -8.300000, 8.509990, 3.137999, -8.300000, -1.490000, 3.138000, -8.300000, -1.490000, 13.138000, -8.300000
		};
		BUILD_COORDS(coords);
	} else if (cnum == 3) {
		float coords[] = {
			24.778297, -0.005699, -25.468597, 24.781147, 12.494300, -25.471447, 24.783998, 24.994299, -25.474298, 24.786850, 37.494297, -25.477148, 24.789700, 49.994297, -25.480000, 12.278297, -0.002849, -25.471447, 12.281149, 12.497150, -25.474298, 12.283999, 24.997150, -25.477148, 12.286850, 37.497150, -25.480000, 12.289701, 49.997147, -25.482849, -0.221702, 0.000001, -25.474298, -0.218851, 12.500001, -25.477150, -0.216000, 25.000000, -25.480000, -0.213149, 37.500000, -25.482849, -0.210298, 50.000000, -25.485701, -12.721701, 0.002852, -25.477150, -12.718850, 12.502851, -25.480000, -12.716000, 25.002850, -25.482851, -12.713148, 37.502850, -25.485701, -12.710298, 50.002850, -25.488552, -25.221701, 0.005702, -25.480000, -25.218849, 12.505702, -25.482851, -25.215998, 25.005701, -25.485701, -25.213148, 37.505699, -25.488552, -25.210297, 50.005699, -25.491402
		};
		BUILD_COORDS(coords);
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

void Model::HandlePointLight() {
	PointLight* light = new PointLight;
	light->location.Set( -20.6, 51.43, 28.99);
	light->emission.Set(4, 4, 4);
	scene->lights.push_back(light);
}

Scene scene;

Ray GetRay(int x, int y) {
	float	h = scene.camera.pixh;	// pixel horizontalis merete
	float	v = scene.camera.pixv;	// pixel vertikalis merete
	// az aktualis pixel kozeppontja
	float	pix_x = -h * SIZE / 2.0 + x * h + h / 2.0;
	float	pix_y = -v * SIZE / 2.0 + y * v + v / 2.0;

	Vector rayDir = scene.camera.Z + pix_x * scene.camera.X + pix_y * scene.camera.Y;
	rayDir.Normalize();
	return Ray(scene.camera.eyep, rayDir);	// a sugar a szembol
}

float pixels[600*600*3];
void SetPixel(int x, int y, Color col) {
	pixels[((SIZE-y-1) * SIZE + x)*3] = col.r;
	pixels[((SIZE-y-1) * SIZE + x)*3+1] = col.g;
	pixels[((SIZE-y-1) * SIZE + x)*3+2] = col.b;
}

void Render(void) {
	for (int y = 0; y < SIZE; y++) {
		for (int x = 0; x < SIZE; x++) {
			Ray r = GetRay(x, y);
			Color col = scene.Trace(r, 0);
			SetPixel(x, y, col);
		}
#ifdef DEBUG
		if (y % (SIZE/100) == 0) {
			printf("\r%d %%", y / (SIZE/100));
			fflush(stdout);
		}
#endif
	}
#ifdef DEBUG
	putchar('\n');
#endif
}

void onInitialization( ) {
	scene.Build();
	glMatrixMode(GL_MODELVIEW);
	gluLookAt(scene.camera.eyep.x, scene.camera.eyep.y, scene.camera.eyep.z,
			scene.camera.eyep.x-scene.camera.lookp.x, scene.camera.eyep.y-scene.camera.lookp.y, scene.camera.eyep.z-scene.camera.lookp.z,
			scene.camera.updir.x, scene.camera.updir.y, scene.camera.updir.z);
	glMatrixMode(GL_PROJECTION);
	gluPerspective(scene.camera.fov*2, 1, scene.camera.nearClip, scene.camera.farClip);
	Render();
	glutPostRedisplay();
}

void onDisplay( ) {
	glClearColor(0.1f, 0.2f, 0.3f, 1.0f);
	glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

	glDrawPixels(SIZE, SIZE, GL_RGB, GL_FLOAT, pixels);

	glColor3f(1.0f, 1.0f, 1.0f);
	// eloszor a pontokat kotjuk ossze
	glBegin(GL_LINE_STRIP);
	for (unsigned i = 0; i < raypoints.size(); i++) {
		glVertex3f(raypoints[i].x, raypoints[i].y, raypoints[i].z);
	}
	glEnd();
	// majd az egyes pontokat a fenyforrassal
	glBegin(GL_LINES);
	for (unsigned i = 0; i < raypoints.size(); i++) {
		glVertex3f(raypoints[i].x, raypoints[i].y, raypoints[i].z);
		glVertex3f(scene.lights[0]->location.x, scene.lights[0]->location.y, scene.lights[0]->location.z);
	}
	glEnd();

	// Buffercsere: rajzolas vege
	glFinish();
	glutSwapBuffers();
}

void onMouse(int button, int state, int x, int y) {
	// A GLUT_LEFT_BUTTON / GLUT_RIGHT_BUTTON
	// ill. a GLUT_DOWN / GLUT_UP makrokat hasznald.
	if (button != GLUT_LEFT_BUTTON || state != GLUT_DOWN)
		return;
	raypoints.clear();
	clickx = x;
	clicky = y;
	Ray r = GetRay(x, y);
	scene.Trace(r, 0);
	glutPostRedisplay();
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
