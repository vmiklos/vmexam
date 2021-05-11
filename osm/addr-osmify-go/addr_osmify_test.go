package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"testing"
)

type URLRoute struct {
	URL string
	DataPath string
	ResultPath string
}

func MockUrlopen(t *testing.T, routes []URLRoute) func(string, string) (string, error) {
	return func(urlString string, data string) (string, error) {
		for _, route := range routes {
			if urlString != route.URL {
				continue
			}

			if len(route.DataPath) > 0 {
				content, err := ioutil.ReadFile(route.DataPath)
				if err != nil {
					return "", fmt.Errorf("ReadFile: %s", err)
				}
				if string(content) != data {
					t.Errorf("unexpected data for urlString=%q:\ngot: %q\nwant:%q", urlString, data, string(content))
				}
			}

			content, err := ioutil.ReadFile(route.ResultPath)
			if err != nil {
				return "", fmt.Errorf("ReadFile: %s", err)
			}
			return string(content), nil
		}

		return "", fmt.Errorf("unexpected urlString=%q and data=%q", urlString, data)
	}
}

// Tests the happy path.
func TestHappy(t *testing.T) {
	OldUrlopen := Urlopen
	defer func() { Urlopen = OldUrlopen }()
	var routes []URLRoute
	route := URLRoute{
		URL: "http://nominatim.openstreetmap.org/search.php?format=json&q=M%C3%A9sz%C3%A1ros+utca+58%2Fa%2C+Budapest",
		ResultPath: "mock/nominatim-happy.json",
	}
	routes = append(routes, route)
	route = URLRoute{
		URL: "http://overpass-api.de/api/interpreter",
		DataPath: "mock/overpass-happy.expected-data",
		ResultPath: "mock/overpass-happy.json",
	}
	routes = append(routes, route)
	Urlopen = MockUrlopen(t, routes)

	want := "geo:47.490592,19.030662 (1016 Budapest, Mészáros utca 58/a)\n"
	argv := []string{"", "Mészáros utca 58/a, Budapest"}
	buf := new(bytes.Buffer)
	Main(argv, buf)
	if buf.String() != want {
		t.Errorf("Main() = %q, want %q", buf.String(), want)
	}
}
