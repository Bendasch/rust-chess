#shader vertex
#version 450 core 

layout(location = 0) in vec4 a_Position; 
layout(location = 1) in vec2 a_TexCoord;
layout(location = 2) in float a_TexIndex;

out vec4 v_Color;
out vec2 v_TexCoord;    
out float v_TexIndex;    

uniform mat4 u_MVP;

void main()
{
    gl_Position = u_MVP * a_Position;
    v_TexCoord = a_TexCoord;
    v_TexIndex = a_TexIndex;
}

#shader fragment
#version 450 core 

layout(location = 0) out vec4 o_Color; 
in vec2 v_TexCoord; 
in float v_TexIndex; 

uniform sampler2D u_Textures[3];

void main()
{
    int index = int(v_TexIndex);
    if (index < 3.0)
    {
        o_Color = texture(u_Textures[index], v_TexCoord);
    }
    else
    {
        o_Color = vec4(0.9, 0.2, 0.15, 0.85);
    }
}