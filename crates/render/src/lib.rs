// How rendering works:

// - Every component that supports drawing will have the "Display" component.

// - STEP 1: the "Display" component will contain/be alongside an "AssetInit" component which will 
//           tell the asset system what textures and stuff are needed. 

// - STEP 2: the "Display" component will define how to "build" itself out of the primitive shapes.

// - STEP 3: The "Display" system will go through the entities and put together a collection of 3d primitive
//           shapes and sprites with various attributes and tags. (IE: glowing, no-shadow/shadow, texture=dirt).

// - STEP 4: A "Renderer" system that implements the "Renderer" trait will be able to look at this 
//           collection of tagged primitives and produce an image.

