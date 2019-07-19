package main

import (
	"bytes"
	"encoding/json"
	"flag"
	"fmt"
	"net/http"
	"net/url"
	"os"
	"time"
)

// NominatimResult represents one element in the result array from Nominatim.
type NominatimResult struct {
	Class   string `json:"class"`
	Lat     string `json:"lat"`
	Lon     string `json:"lon"`
	OsmType string `json:"osm_type"`
	OsmID   int    `json:"osm_id"`
}

// TurboTags contains various tags about one Overpass element.
type TurboTags struct {
	City        string `json:"addr:city"`
	HouseNumber string `json:"addr:housenumber"`
	PostCode    string `json:"addr:postcode"`
	Street      string `json:"addr:street"`
}

// TurboElement represents one result from Overpass.
type TurboElement struct {
	Tags TurboTags `json:"tags"`
}

// TurboResult is the result from Overpass.
type TurboResult struct {
	Elements []TurboElement `json:"elements"`
}

// SpinnerResult is sent over a channel to the spinner.
type SpinnerResult struct {
	Result string
	Error  error
}

func queryNominatim(query string) (*[]NominatimResult, error) {
	nominatimURL := "http://nominatim.openstreetmap.org/search.php?"
	params := url.Values{}
	params.Add("q", query)
	params.Add("format", "json")
	nominatimURL += params.Encode()
	resp, err := http.Get(nominatimURL)
	if err != nil {
		return nil, fmt.Errorf("http.Get: %s", err)
	}

	defer resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("http.Get status: %s", resp.Status)
	}

	var result []NominatimResult
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return nil, fmt.Errorf("json decode failed: %s", err)
	}

	return &result, nil
}

func queryTurbo(query string) (*TurboResult, error) {
	turboURL := "http://overpass-api.de/api/interpreter"
	req, err := http.NewRequest("POST", turboURL, bytes.NewBufferString(query))
	if err != nil {
		return nil, fmt.Errorf("http.NewRequest: %s", err)
	}

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return nil, fmt.Errorf("client.Do: %s", err)
	}

	defer resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("http.Get status: %s", resp.Status)
	}

	var result TurboResult
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return nil, fmt.Errorf("json decode failed: %s", err)
	}

	return &result, nil
}

func osmify(query string) (*string, error) {
	// Use nominatim to get the coordinates and the osm type/id.
	elements, err := queryNominatim(query)
	if err != nil {
		return nil, fmt.Errorf("queryNominatim: %s", err)
	}

	if len(*elements) == 0 {
		return nil, fmt.Errorf("No results from nominatim")
	}

	if len(*elements) > 1 {
		// There are multiple elements, prefer buildings if possible.
		// Example where this is useful: 'Karinthy Frigyes út 18, Budapest'.
		buildings := make([]NominatimResult, 0)
		for _, element := range *elements {
			if element.Class == "building" {
				buildings = append(buildings, element)
			}
		}

		if len(buildings) > 0 {
			elements = &buildings
		}
	}

	element := (*elements)[0]
	lat := element.Lat
	lon := element.Lon
	objectType := element.OsmType
	objectID := element.OsmID

	// Use overpass to get the properties of the object.
	overpassQuery := fmt.Sprintf(`[out:json];
(
	%s(%d);
);
out body;`, objectType, objectID)
	turboResult, err := queryTurbo(overpassQuery)
	if err != nil {
		return nil, fmt.Errorf("queryTurbo: %s", err)
	}

	turboElements := turboResult.Elements
	if len(turboElements) == 0 {
		return nil, fmt.Errorf("No results from overpass")
	}

	turboElement := turboElements[0]
	city := turboElement.Tags.City
	houseNumber := turboElement.Tags.HouseNumber
	postCode := turboElement.Tags.PostCode
	street := turboElement.Tags.Street
	addr := fmt.Sprintf("%s %s, %s %s", postCode, city, street, houseNumber)

	// Print the result.
	result := fmt.Sprintf("geo:%s,%s (%s)", lat, lon, addr)
	return &result, nil
}

func spinner(ch chan SpinnerResult) int {
	spinCharacters := `\|/-`
	spinIndex := 0
	for {
		select {
		case result := <-ch:
			if result.Error != nil {
				fmt.Printf("\r%s\n", result.Error)
				return 1
			}

			fmt.Printf("\r%s\n", result.Result)
			return 0
		default:
			fmt.Printf("\r [%c] ", spinCharacters[spinIndex])
			spinIndex = (spinIndex + 1) % len(spinCharacters)
			time.Sleep(100 * time.Millisecond)
		}
	}
}

func main() {
	flag.Parse()
	if flag.NArg() > 0 {
		ch := make(chan SpinnerResult)
		go func() {
			result, err := osmify(flag.Args()[0])
			if err != nil {
				ch <- SpinnerResult{Error: fmt.Errorf("osmify: %s", err)}
			} else {
				ch <- SpinnerResult{Result: *result}
			}
		}()
		os.Exit(spinner(ch))
	} else {
		fmt.Println("usage: addr-osmify <query>")
		fmt.Println()
		fmt.Println("e.g. addr-osmify 'Mészáros utca 58/a, Budapest'")
	}
}
