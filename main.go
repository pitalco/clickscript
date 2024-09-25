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

	components := c.CompileToStencil()

	// Set the root directory for the clickscript-starter project
	rootDir := "./clickscript-starter"

	for name, content := range components {
		// Create the directory for the component
		componentDir := filepath.Join(rootDir, "src", "components", name)
		err := os.MkdirAll(componentDir, os.ModePerm)
		if err != nil {
			log.Fatalf("Error creating component directory %s: %v", componentDir, err)
		}

		// Write the component file
		componentFile := filepath.Join(componentDir, name+".tsx")
		err = os.WriteFile(componentFile, []byte(content), 0644)
		if err != nil {
			log.Fatalf("Error writing component file %s: %v", componentFile, err)
		}
		fmt.Printf("Generated StencilJS component: %s\n", componentFile)

		// Create an empty CSS file for the component
		cssFile := filepath.Join(componentDir, name+".css")
		err = os.WriteFile(cssFile, []byte(""), 0644)
		if err != nil {
			log.Fatalf("Error creating CSS file %s: %v", cssFile, err)
		}
		fmt.Printf("Created empty CSS file: %s\n", cssFile)
	}
}
