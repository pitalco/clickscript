package compiler

import (
	"encoding/json"
	"fmt"
	"os"
	"strings"
)

type Prop struct {
	Name string `json:"name"`
	Type string `json:"type"`
}

type Attribute struct {
	Name  string `json:"name"`
	Value string `json:"value"`
}

type Element struct {
	Element    string      `json:"element"`
	Attributes []Attribute `json:"attributes,omitempty"`
	Content    string      `json:"content,omitempty"`
	Children   []Element   `json:"children,omitempty"`
}

type ScriptAction struct {
	Action string                 `json:"action"`
	Args   map[string]interface{} `json:"args"`
}

type Component struct {
	Name     string         `json:"name"`
	Props    []Prop         `json:"props,omitempty"`
	Template []Element      `json:"template"`
	Script   []ScriptAction `json:"script,omitempty"`
}

type Compiler struct {
	Components []Component
}

func NewCompiler(jsonFilePath string) (*Compiler, error) {
	data, err := os.ReadFile(jsonFilePath)
	if err != nil {
		return nil, fmt.Errorf("error reading JSON file: %v", err)
	}

	var compilerData struct {
		Components []Component `json:"components"`
	}
	err = json.Unmarshal(data, &compilerData)
	if err != nil {
		return nil, fmt.Errorf("error unmarshaling JSON: %v", err)
	}

	return &Compiler{Components: compilerData.Components}, nil
}

func (c *Compiler) CompileToStencil() map[string]string {
	components := make(map[string]string)

	for _, component := range c.Components {
		components[component.Name] = c.generateStencilComponent(component)
	}

	return components
}

func (c *Compiler) generateStencilComponent(component Component) string {
	var sb strings.Builder

	// Import statements
	sb.WriteString("import { Component, Prop, h } from '@stencil/core';\n\n")

	// Component decorator
	sb.WriteString("@Component({\n")
	sb.WriteString("  tag: '" + component.Name + "',\n")
	sb.WriteString("  styleUrl: '" + component.Name + ".css',\n")
	sb.WriteString("  shadow: true,\n")
	sb.WriteString("})\n")

	// Component class
	sb.WriteString("export class " + toPascalCase(component.Name) + " {\n")

	// Props
	for _, prop := range component.Props {
		sb.WriteString("  @Prop() " + prop.Name + ": " + prop.Type + ";\n")
	}
	sb.WriteString("\n")

	// Lifecycle method
	if len(component.Script) > 0 {
		sb.WriteString("  componentDidLoad() {\n")
		for _, action := range component.Script {
			sb.WriteString("    console." + action.Action + "(" + formatArgs(action.Args) + ");\n")
		}
		sb.WriteString("  }\n\n")
	}

	// Render method
	sb.WriteString("  render() {\n")
	sb.WriteString("    return (\n")
	sb.WriteString(c.renderTemplate(component.Template, 6))
	sb.WriteString("    );\n")
	sb.WriteString("  }\n")

	sb.WriteString("}\n")

	return sb.String()
}

func (c *Compiler) renderTemplate(elements []Element, indent int) string {
	var sb strings.Builder

	for _, element := range elements {
		sb.WriteString(c.renderElement(element, indent))
	}

	return sb.String()
}

func (c *Compiler) renderElement(element Element, indent int) string {
	var sb strings.Builder

	indentStr := strings.Repeat(" ", indent)
	sb.WriteString(indentStr)

	sb.WriteString("<" + element.Element)

	for _, attr := range element.Attributes {
		sb.WriteString(" " + attr.Name + "=\"" + attr.Value + "\"")
	}

	if len(element.Children) == 0 && element.Content == "" {
		sb.WriteString(" />\n")
	} else {
		sb.WriteString(">\n")
		if element.Content != "" {
			sb.WriteString(indentStr + "  " + element.Content + "\n")
		}
		for _, child := range element.Children {
			sb.WriteString(c.renderElement(child, indent+2))
		}
		sb.WriteString(indentStr + "</" + element.Element + ">\n")
	}

	return sb.String()
}

func toPascalCase(s string) string {
	parts := strings.Split(s, "-")
	for i, part := range parts {
		parts[i] = strings.Title(part)
	}
	return strings.Join(parts, "")
}

func formatArgs(args map[string]interface{}) string {
	var formattedArgs []string
	for key, value := range args {
		switch v := value.(type) {
		case string:
			formattedArgs = append(formattedArgs, fmt.Sprintf("%s: '%s'", key, v))
		case []interface{}:
			strValues := make([]string, len(v))
			for i, item := range v {
				strValues[i] = fmt.Sprintf("'%v'", item)
			}
			formattedArgs = append(formattedArgs, fmt.Sprintf("%s: [%s]", key, strings.Join(strValues, ", ")))
		default:
			formattedArgs = append(formattedArgs, fmt.Sprintf("%s: %v", key, v))
		}
	}
	return strings.Join(formattedArgs, ", ")
}
