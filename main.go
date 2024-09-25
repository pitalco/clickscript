package main

import (
	"fmt"
	"log"
	"os"
	"path/filepath"

	"github.com/pitalco/clickscript/compiler"
)

func main() {
	c, err := compiler.NewCompiler("example.click.json")
	if err != nil {
		log.Fatalf("Error creating compiler: %v", err)
	}

	components, err := c.CompileToWebComponents("./clickscript-starter")
	if err != nil {
		log.Fatalf("Error compiling to web components: %v", err)
	}

	// Set the root directory for the clickscript-starter project
	rootDir := "./clickscript-starter"

	// Create the main HTML file
	mainHTML := c.GenerateIndexHTML(c.ComponentNames())
	err = os.WriteFile(filepath.Join(rootDir, "index.html"), []byte(mainHTML), 0644)
	if err != nil {
		log.Fatalf("Error writing main HTML file: %v", err)
	}
	fmt.Println("Generated main HTML file: index.html")

	for name, content := range components {
		// Create the directory for the component
		componentDir := filepath.Join(rootDir, "src", "components")
		err := os.MkdirAll(componentDir, os.ModePerm)
		if err != nil {
			log.Fatalf("Error creating component directory %s: %v", componentDir, err)
		}

		// Write the component file
		componentFile := filepath.Join(componentDir, name+".js")
		err = os.WriteFile(componentFile, []byte(content), 0644)
		if err != nil {
			log.Fatalf("Error writing component file %s: %v", componentFile, err)
		}
		fmt.Printf("Generated Web Component: %s\n", componentFile)
	}
}
