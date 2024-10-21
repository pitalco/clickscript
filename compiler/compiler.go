package compiler

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

type Arg struct {
	Type  string      `json:"type"`
	Value interface{} `json:"value"`
}

type Action struct {
	Action string `json:"action"`
	Args   []Arg  `json:"args"`
}

type ClickscriptData struct {
	Client []Action `json:"client"`
	Server []Action `json:"server"`
}

type Compiler struct {
	Data ClickscriptData
}

func NewCompiler(jsonFilePath string) (*Compiler, error) {
	data, err := os.ReadFile(jsonFilePath)
	if err != nil {
		return nil, fmt.Errorf("error reading JSON file: %v", err)
	}

	var clickscriptData ClickscriptData
	err = json.Unmarshal(data, &clickscriptData)
	if err != nil {
		return nil, fmt.Errorf("error unmarshaling JSON: %v", err)
	}

	return &Compiler{Data: clickscriptData}, nil
}

func (c *Compiler) Compile(outputDir string) error {
	err := c.compileServer(outputDir)
	if err != nil {
		return err
	}

	return nil
}

func (c *Compiler) compileServer(outputDir string) error {
	serverCode := c.generateCode(c.Data.Server)
	serverFilePath := filepath.Join(outputDir, "server.js")
	err := os.WriteFile(serverFilePath, []byte(serverCode), 0644)
	if err != nil {
		return fmt.Errorf("error writing server file: %v", err)
	}
	return nil
}

func (c *Compiler) generateCode(actions []Action) string {
	var sb strings.Builder

	for _, action := range actions {
		sb.WriteString(fmt.Sprintf("import { %s } from '@clickscript/actions';\n\n", action.Action))
	}

	for _, action := range actions {
		sb.WriteString(c.generateAction(action))
		sb.WriteString("\n")
	}

	return sb.String()
}

func (c *Compiler) generateAction(action Action) string {
	actionCode := fmt.Sprintf("%s(%s);\n", action.Action, c.generateArgs(action.Args))
	return actionCode
}

func (c *Compiler) generateArgs(args []Arg) string {
	argStrings := make([]string, len(args))
	for i, arg := range args {
		argStrings[i] = arg.toTypes()
	}
	return strings.Join(argStrings, ", ")
}

func (a *Arg) toTypes() string {
	switch a.Type {
	case "text":
		return fmt.Sprintf("'%s'", a.Value)
	case "number":
		return fmt.Sprintf("%v", a.Value)
	case "boolean":
		return fmt.Sprintf("%t", a.Value)
	case "array":
		values := a.Value.([]interface{})
		arrayStrings := make([]string, len(values))
		for i, v := range values {
			arrayStrings[i] = (&Arg{Type: getType(v), Value: v}).toTypes()
		}
		return fmt.Sprintf("[%s]", strings.Join(arrayStrings, ", "))
	case "object":
		obj := a.Value.(map[string]interface{})
		pairs := make([]string, 0, len(obj))
		for k, v := range obj {
			pairs = append(pairs, fmt.Sprintf("%s: %s", k, (&Arg{Type: getType(v), Value: v}).toTypes()))
		}
		return fmt.Sprintf("{ %s }", strings.Join(pairs, ", "))
	default:
		return fmt.Sprintf("%v", a.Value)
	}
}

func getType(v interface{}) string {
	switch v.(type) {
	case string:
		return "text"
	case float64, int:
		return "number"
	case bool:
		return "boolean"
	case []interface{}:
		return "array"
	case map[string]interface{}:
		return "object"
	default:
		return "unknown"
	}
}
