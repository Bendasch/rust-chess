#shader vertex
#version 330 core 

layout(location = 0) in vec4 a_Position; 
layout(location = 1) in vec4 a_Color;
out vec4 v_Color;
//layout(location = 1) in vec2 textureCoord;
//out vec2 v_TexCoord;    

uniform mat4 u_MVP;

void main()
{
    gl_Position = u_MVP * a_Position;
    //v_TexCoord = textureCoord;
    v_Color = a_Color;
}

#shader fragment
#version 330 core 

layout(location = 0) out vec4 color; 
in vec4 v_Color; 
//in vec2 v_TexCoord; 

uniform sampler2D u_Texture;

void main()
{
    //vec4 texColor = texture(u_Texture, v_TexCoord);
    //color = texColor;
    color = v_Color;
}