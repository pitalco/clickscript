package compiler

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
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

func (c *Compiler) CompileToWebComponents(outputDir string) (map[string]string, error) {
	components := make(map[string]string)

	for _, component := range c.Components {
		if _, exists := components[component.Name]; !exists {
			componentCode := c.generateWebComponent(component)
			components[component.Name] = componentCode

			// Write each component to a separate file
			fileName := filepath.Join(outputDir, "src", "components", component.Name+".js")
			err := os.MkdirAll(filepath.Dir(fileName), os.ModePerm)
			if err != nil {
				return nil, fmt.Errorf("error creating directory for component %s: %v", component.Name, err)
			}
			err = os.WriteFile(fileName, []byte(componentCode), 0644)
			if err != nil {
				return nil, fmt.Errorf("error writing component file %s: %v", fileName, err)
			}
		}
	}

	// Generate and write index.html
	indexHTML := c.GenerateIndexHTML(c.ComponentNames())
	indexFileName := filepath.Join(outputDir, "index.html")
	err := os.WriteFile(indexFileName, []byte(indexHTML), 0644)
	if err != nil {
		return nil, fmt.Errorf("error writing index.html: %v", err)
	}

	return components, nil
}

func (c *Compiler) generateWebComponent(component Component) string {
	var sb strings.Builder

	// Class definition
	sb.WriteString("class " + toPascalCase(component.Name) + " extends HTMLElement {\n")

	// Constructor
	sb.WriteString("  constructor() {\n")
	sb.WriteString("    super();\n")
	sb.WriteString("    this.attachShadow({ mode: 'open' });\n")
	sb.WriteString("  }\n\n")

	// Observed attributes
	sb.WriteString("  static get observedAttributes() {\n")
	sb.WriteString("    return [" + c.generateObservedAttributes(component.Props) + "];\n")
	sb.WriteString("  }\n\n")

	// Lifecycle methods
	sb.WriteString("  connectedCallback() {\n")
	sb.WriteString("    this.render();\n")
	sb.WriteString(c.generateScriptActions(component.Script))
	sb.WriteString("  }\n\n")

	sb.WriteString("  attributeChangedCallback(name, oldValue, newValue) {\n")
	sb.WriteString("    this.render();\n")
	sb.WriteString("  }\n\n")

	// Render method
	sb.WriteString("  render() {\n")
	sb.WriteString("    this.shadowRoot.innerHTML = `\n")
	sb.WriteString("      <link href=\"https://cdn.jsdelivr.net/npm/flowbite@2.5.1/dist/flowbite.min.css\" rel=\"stylesheet\">\n")
	sb.WriteString(c.renderTemplate(component.Template, 6))
	sb.WriteString("      <script src=\"https://cdn.jsdelivr.net/npm/flowbite@2.5.1/dist/flowbite.min.js\"></script>\n")
	sb.WriteString("    `;\n")
	sb.WriteString("  }\n")

	sb.WriteString("}\n\n")

	// Custom element definition
	sb.WriteString("customElements.define('" + component.Name + "', " + toPascalCase(component.Name) + ");\n")

	return sb.String()
}

func (c *Compiler) generateObservedAttributes(props []Prop) string {
	attributes := make([]string, len(props))
	for i, prop := range props {
		attributes[i] = "'" + prop.Name + "'"
	}
	return strings.Join(attributes, ", ")
}

func (c *Compiler) generateScriptActions(actions []ScriptAction) string {
	var sb strings.Builder
	for _, action := range actions {
		sb.WriteString("    console." + action.Action + "(" + formatArgs(action.Args) + ");\n")
	}
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

func (c *Compiler) GenerateIndexHTML(componentNames []string) string {
	var sb strings.Builder

	sb.WriteString("<!DOCTYPE html>\n")
	sb.WriteString("<html lang=\"en\">\n")
	sb.WriteString("<head>\n")
	sb.WriteString("    <meta charset=\"UTF-8\">\n")
	sb.WriteString("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n")
	sb.WriteString("    <title>Clickscript App</title>\n")
	sb.WriteString("    <link href=\"https://cdn.jsdelivr.net/npm/flowbite@2.5.1/dist/flowbite.min.css\" rel=\"stylesheet\" />\n")

	// Add script tags for components
	for _, name := range componentNames {
		sb.WriteString(fmt.Sprintf("    <script src=\"src/components/%s.js\" type=\"module\"></script>\n", name))
	}

	sb.WriteString("</head>\n")
	sb.WriteString("<body>\n")

	// Add component tags
	for _, name := range componentNames {
		sb.WriteString(fmt.Sprintf("    <%s></%s>\n", name, name))
	}

	// Add Flowbite script
	sb.WriteString("    <script src=\"https://cdn.jsdelivr.net/npm/flowbite@2.5.1/dist/flowbite.min.js\"></script>\n")

	sb.WriteString("</body>\n")
	sb.WriteString("</html>\n")

	return sb.String()
}

func (c *Compiler) ComponentNames() []string {
	names := make([]string, len(c.Components))
	for i, comp := range c.Components {
		names[i] = comp.Name
	}
	return names
}
