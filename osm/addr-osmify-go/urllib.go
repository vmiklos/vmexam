package main

import (
	"bytes"
	"fmt"
	"net/http"
)

func urlopen(url string, data string) (string, error) {
	if len(data) == 0 {
		resp, err := http.Get(url)
		if err != nil {
			return "", fmt.Errorf("http.Get: %s", err)
		}

		defer resp.Body.Close()
		if resp.StatusCode != http.StatusOK {
			return "", fmt.Errorf("http.Get status: %s", resp.Status)
		}

		buf := new(bytes.Buffer)
		buf.ReadFrom(resp.Body)
		return buf.String(), nil
	}

	req, err := http.NewRequest("POST", url, bytes.NewBufferString(data))
	if err != nil {
		return "", fmt.Errorf("http.NewRequest: %s", err)
	}

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return "", fmt.Errorf("client.Do: %s", err)
	}

	defer resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		return "", fmt.Errorf("http.Get status: %s", resp.Status)
	}

	buf := new(bytes.Buffer)
	buf.ReadFrom(resp.Body)
	return buf.String(), nil
}

// Urlopen is a wrapper around http.Get() and with POST functionality.
var Urlopen = urlopen
