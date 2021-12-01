#shader vertex
#version 330 core 

layout(location = 0) in vec4 a_Position; 
layout(location = 1) in vec2 a_TexCoord;
layout(location = 2) in float a_TexIndex;

out vec2 v_TexCoord;    
out float v_TexIndex;    

uniform mat4 u_MVP;
uniform mat4 u_MVP_UI;

void main()
{   
    if (a_TexIndex != 3.0)
    {
        gl_Position = u_MVP * a_Position;
        v_TexCoord = a_TexCoord;
        v_TexIndex = a_TexIndex;
    }
    else
    {
        gl_Position = u_MVP_UI * a_Position;
        v_TexCoord = a_TexCoord;
        v_TexIndex = a_TexIndex;
    }
}   

#shader fragment
#version 330 core 

layout(location = 0) out vec4 o_Color; 
in vec2 v_TexCoord; 
in float v_TexIndex; 

uniform sampler2D u_Textures[4];

void main()
{
    int index = int(v_TexIndex);
    if (index < 9.0)
    {
        o_Color = texture(u_Textures[index], v_TexCoord);
    }
    else
    {
        o_Color = vec4(0.9, 0.2, 0.15, 0.85);
    }
}