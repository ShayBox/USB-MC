package main

import (
	"io"
	"net/http"
	"os"
	"os/exec"
)

func main() {
	directory := ".minecraft/"
	if !DirectoryExists(directory) {
		println("Creating directory")
		os.MkdirAll(directory, os.ModePerm)
	}

	file := "Minecraft.jar"
	if !FileExists(directory + file) {
		println("Downloading launcher")
		DownloadFile(directory+file, "http://s3.amazonaws.com/Minecraft.Download/launcher/Minecraft.jar")
	}

	println("Starting Minecraft")
	cmd := exec.Command("java", "-jar", file, "--workDir", directory)
	cmd.Dir = directory
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err := cmd.Run()
	if err != nil {
		panic(err)
	}
	cmd.Wait()
}

// DirectoryExists checks if a directory exists and is not a dirfileectory before we
// try using it to prevent further errors.
func DirectoryExists(filename string) bool {
	info, err := os.Stat(filename)
	if os.IsNotExist(err) {
		return false
	}
	return info.IsDir()
}

// FileExists checks if a file exists and is not a directory before we
// try using it to prevent further errors.
func FileExists(filename string) bool {
	info, err := os.Stat(filename)
	if os.IsNotExist(err) {
		return false
	}
	return !info.IsDir()
}

// DownloadFile will download a url to a local file. It's efficient because it will
// write as it downloads and not load the whole file into memory.
func DownloadFile(filepath string, url string) error {

	// Get the data
	resp, err := http.Get(url)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	// Create the file
	out, err := os.Create(filepath)
	if err != nil {
		return err
	}
	defer out.Close()

	// Write the body to file
	_, err = io.Copy(out, resp.Body)
	return err
}
