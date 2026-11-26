# Anakena

Anakena is a conversation-driven 3D game in development for iOS and Android. It brings players into interactive stories set in realistic places around the world, where they speak with lifelike AI characters and make choices that shape how the story ends.

## Playing a story

Pick a story from the catalog and choose the language you want to experience it in. You enter a 3D world and take part in the story by talking with its characters. Conversation carries the story forward, and what you say and how you say it are part of play throughout the experience.

Play in your native language to enjoy the story, or choose a language you’re learning to practice using it in situations you might encounter outside a lesson. Part of the challenge can be speaking naturally enough that the characters take you for a native speaker.

## How stories branch

Each story has an authored premise, characters with their own backgrounds, and a defined set of actions and endings. Players can speak in their own words, and AI characters can respond to different ways of expressing the same intent. Different conversations can lead to the same action, while different actions can send the story down different paths. The player’s choices determine which of the authored endings they reach.

## Engine development

The first priority is a stable rendering foundation for realistic 3D environments and avatars. The engine is written in Rust and uses `winit` for window events, `wgpu` for rendering, and WGSL for shaders.

The window and redraw handling are in place, and the render pass clears the surface to blue. The vertex and fragment shaders define a yellow triangle. The render pipeline is under construction, and drawing the triangle is the next milestone.
