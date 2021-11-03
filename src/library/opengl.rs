use libc::{c_int, c_float};

// GLbitfield 
pub const GL_COLOR_BUFFER_BIT:      c_int = 0x00004000;     // Indicates the buffers currently enabled for color writing.
pub const GL_DEPTH_BUFFER_BIT:      c_int = 0x00000100;     // Indicates the depth buffer.
pub const GL_ACCUM_BUFFER_BIT:      c_int = 0x00000200;     // Indicates the accumulation buffer.
pub const GL_STENCIL_BUFFER_BIT:    c_int = 0x00000400;     // Indicates the stencil buffer.


// GLenum

/*  Treats each vertex as a single point. Vertex n defines point n. N points are drawn. */
pub const GL_POINTS: c_int = 0x0000; 

/*  Treats each pair of vertices as an independent line segment. 
    Vertices 2 ⁢ n - 1 and 2 ⁢ n define line n. N 2 lines are drawn. */
pub const GL_LINES: c_int = 0x0001; 

/*  Draws a connected group of line segments from the first vertex to the last, 
    then back to the first. Vertices n and n + 1 define line n. The last line, 
    however, is defined by vertices N and 1 . N lines are drawn. */
pub const GL_LINE_LOOP: c_int = 0x0002; 

/*  Draws a connected group of line segments from the first vertex to the last. 
    Vertices n and n + 1 define line n. N - 1 lines are drawn. */
pub const GL_LINE_STRIP: c_int = 0x0003; 
    
/*  Treats each triplet of vertices as an independent triangle. 
    Vertices 3 ⁢ n - 2 , 3 ⁢ n - 1 , and 3 ⁢ n define triangle n. N 3 triangles are drawn. */
pub const GL_TRIANGLES: c_int = 0x0004; 

/*  Draws a connected group of triangles. One triangle is defined for 
    each vertex presented after the first two vertices. For odd n, 
    vertices n, n + 1 , and n + 2 define triangle n. For even n, vertices 
    n + 1 , n, and n + 2 define triangle n. N - 2 triangles are drawn. */
pub const GL_TRIANGLE_STRIP: c_int = 0x0005; 

/*  Draws a connected group of triangles. One triangle is defined for each 
    vertex presented after the first two vertices. Vertices 1 , n + 1 , and 
    n + 2 define triangle n. N - 2 triangles are drawn. */
pub const GL_TRIANGLE_FAN: c_int = 0x0006; 

/*  Treats each group of four vertices as an independent quadrilateral. Vertices 4 ⁢n - 3 , 
    4 ⁢ n - 2 , 4 ⁢ n - 1 , and 4 ⁢ n define quadrilateral n. N 4 quadrilaterals are drawn. */
pub const GL_QUADS: c_int = 0x0007; 

/*  Draws a connected group of quadrilaterals. One quadrilateral is defined for each 
    pair of vertices presented after the first pair. Vertices 2 ⁢ n - 1 , 2 ⁢ n , 2 ⁢ n + 2 , 
    and 2 ⁢ n + 1 define quadrilateral n. N 2 - 1 quadrilaterals are drawn. Note that the 
    order in which vertices are used to construct a quadrilateral from strip data is 
    different from that used with independent data. */
pub const GL_QUAD_STRIP: c_int = 0x0008; 

/*  Draws a single, convex polygon. Vertices 1 through N define this polygon. */
pub const GL_POLYGON: c_int = 0x0009; 


#[link(name="opengl32")]
extern "C" {
    pub fn glClear(mask: c_int);
    pub fn glBegin(mode: c_int);
    pub fn glEnd();
    pub fn glVertex2f(x: c_float, y: c_float); 
}