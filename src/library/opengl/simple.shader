#shader vertex
#version 330 core 

layout(location = 0) in vec4 position; 
    
void main()
{
    gl_Position = position;
}

#shader fragment
#version 330 core 

layout(location = 0) out vec4 color; 

uniform vec4 u_Color;

void main()
{
    //color = vec4(0.15, 0.25, 0.82, 0.75);
    color = u_Color;
}