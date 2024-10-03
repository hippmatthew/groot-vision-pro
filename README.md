## Groot Vision Pro ##
Just a fun project meant to play around with OpenXR and Vulkan to mimic some apple vision pro features. In case apple is reading this... the name is satire. Also for those confused about the 'groot' part, that is my nickname

Some goals I have in mind:
1. create a vulkan dynamic renderer compatible with OpenXR
2. create a gui component library with winit
3. Use ultralight alongside my gui library to make a browser
4. Place the user in a calm environment (like the side of a mountain with a waterfall)
5. For memes - access the menu using the Sword Art Online gesture
6. Ability to spawn primitives in because why not make this a physics sim at the same time
7. apple music api for music
8. Since most of this should be pretty lightweight, i can try path tracing everything :O
9. Word processing application for typing documents?

Lets call this a 'VR Playground / Office thingy'

My initial thoughts:
- Dynamic rendering should be simple. Been there done that. Just not sure how to hook it up to openxr.
- Never used winit, so im not sure how to create a gui library with it. I will build it as I go I guess. Most modern gpu based gui libraries use things called views so i will have to look into that.
- Once the component library is made, i should be able to create a gui easily for the ultralight browser. I believe this should just work by rendering a texture each frame and putting it in a buffer on the gpu. Not sure about audio though.
- I'll have to figure out how to model a calm scene. Rendering the models should be easy enough since I'll already have a dynamic renderer in place
- I think once i have the keyboard functionality working with my gui library, creating a word processing application should be fine. Where would the documents be stored though?
- I'll have to think about how to do gestures / controller input.
- If I can track head position, I can infer where the user's eyes are looking and change the opacity of the windows accordingly. That should provide a nice professional look and ease eye strain?
- It would be neat to have objects like a speaker, which plays 3D sound in the world space when you turn on apple music. Also headphones, which just plays the music as normal. This might be ambitious though as I would have to study the mathematics of world space audio