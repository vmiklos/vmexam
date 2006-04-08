#include <GeoIP.h>

int main (int argc, char **argv)
{
	GeoIP * gp;

	gp = GeoIP_new(GEOIP_STANDARD);
	printf("%s\n", GeoIP_country_code_by_addr(gp, "66.93.236.84"));
}
