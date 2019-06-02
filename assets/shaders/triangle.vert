#version 330 core
//VERTEX SHADER

// "in" for input
// layout (location = 0) refers to the index of a Vertex Attribute
// vec3 is type
// Position is name

// Position attribute
layout (location = 0) in vec3 Position;

// Color attribute
layout (location = 1) in vec3 Color;

// Output
out VS_OUTPUT {
   vec3 Color;
} OUT;

void main()
{
    // At the end of the main function, whatever we set gl_Position to
    // will be used as the output of the vertex shader
    gl_Position = vec4(Position, 1.0);

    OUT.Color = Color;
}
